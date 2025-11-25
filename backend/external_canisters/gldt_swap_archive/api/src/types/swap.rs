use bity_ic_canister_time::{timestamp_millis, HOUR_IN_MS, MINUTE_IN_MS};
use candid::{CandidType, Nat, Principal};
use ic_ledger_types::{AccountIdentifier, TransferError};
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::{
    icrc1::{
        account::{Account, Subaccount},
        transfer::TransferError as TransferErrorIcrc,
    },
    icrc2::{approve::ApproveError, transfer_from::TransferFromError},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::{Milliseconds, TimestampMillis};

use crate::{
    gldt::{GldtNumTokens, GLDT_TX_FEE},
    nft::NftID,
};

pub const STALE_SWAP_TIME_THRESHOLD_MINUTES: u64 = 3;

// ----------------------
//     CRON JOB INTERVALS & Retries & delays
// ----------------------
pub const MANAGE_GLDT_SUPPLY_INTERVAL: Milliseconds = HOUR_IN_MS * 6;
pub const MANAGE_GLDT_SUPPLY_RETRY_DELAY: Milliseconds = MINUTE_IN_MS * 3;
pub const MANAGE_ARCHIVE_CYCLE_INTERVAL: Milliseconds = MINUTE_IN_MS * 10;
pub const MANAGE_NEW_ARCHIVES_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_OGY_FEE_ACCOUNTS_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_SERVICE_STATUS_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_STALE_SWAPS_INTERVAL: Milliseconds = MINUTE_IN_MS;

// -----------------
//     Shared
// -----------------

pub type SwapIndex = Nat;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SwapId(pub NftID, pub SwapIndex);

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapInfo {
    Forward(SwapDetailForward),
    Reverse(SwapDetailReverse),
}

impl From<SwapInfo> for ICRC3Value {
    fn from(info: SwapInfo) -> Self {
        match info {
            SwapInfo::Forward(fwd) => {
                let mut map = BTreeMap::new();
                map.insert("index".into(), ICRC3Value::Nat(fwd.index.clone()));
                map.insert("sale_id".into(), ICRC3Value::Text(fwd.sale_id.clone()));
                map.insert("nft_id".into(), ICRC3Value::Nat(fwd.nft_id.0.clone()));
                map.insert(
                    "nft_id_string".into(),
                    ICRC3Value::Text(fwd.nft_id_string.clone()),
                );
                map.insert(
                    "status".into(),
                    ICRC3Value::Text(format!("{:?}", fwd.status)),
                );
                map.insert(
                    "created_at".into(),
                    ICRC3Value::Nat(Nat::from(fwd.created_at)),
                );
                map.insert(
                    "tokens_to_mint".into(),
                    ICRC3Value::Nat(fwd.tokens_to_mint.value.clone()),
                );
                map.insert(
                    "tokens_to_mint_with_fee".into(),
                    ICRC3Value::Nat(fwd.tokens_to_mint.value_with_fee.clone()),
                );
                map.insert(
                    "escrow_sub_account".into(),
                    ICRC3Value::Blob(fwd.escrow_sub_account.to_vec().into()),
                );
                map.insert(
                    "gldt_receiver".into(),
                    ICRC3Value::Text(format!("{:?}", fwd.gldt_receiver)),
                );
                map.insert(
                    "nft_canister".into(),
                    ICRC3Value::Text(fwd.nft_canister.to_text()),
                );
                ICRC3Value::Map(map)
            }
            SwapInfo::Reverse(rev) => {
                let mut map = BTreeMap::new();
                map.insert("index".into(), ICRC3Value::Nat(rev.index.clone()));
                map.insert("nft_id".into(), ICRC3Value::Nat(rev.nft_id.0.clone()));
                map.insert(
                    "nft_id_string".into(),
                    ICRC3Value::Text(rev.nft_id_string.clone()),
                );
                map.insert(
                    "nft_canister".into(),
                    ICRC3Value::Text(rev.nft_canister.to_text()),
                );
                map.insert(
                    "status".into(),
                    ICRC3Value::Text(format!("{:?}", rev.status)),
                );
                map.insert(
                    "created_at".into(),
                    ICRC3Value::Nat(Nat::from(rev.created_at)),
                );
                map.insert(
                    "tokens_to_receive".into(),
                    ICRC3Value::Nat(rev.tokens_to_receive.value.clone()),
                );
                map.insert(
                    "tokens_to_receive_with_fee".into(),
                    ICRC3Value::Nat(rev.tokens_to_receive.value_with_fee.clone()),
                );
                map.insert("swap_fee".into(), ICRC3Value::Nat(rev.swap_fee.clone()));
                map.insert(
                    "transfer_fees".into(),
                    ICRC3Value::Nat(rev.transfer_fees.clone()),
                );
                map.insert("user".into(), ICRC3Value::Text(rev.user.to_text()));

                ICRC3Value::Map(map)
            }
        }
    }
}

impl SwapInfo {
    pub fn new(swap_type: SwapType) -> Self {
        match swap_type {
            SwapType::Forward => Self::Forward(SwapDetailForward::default()),
            SwapType::Reverse => Self::Reverse(SwapDetailReverse::default()),
        }
    }

    pub fn get_status(&self) -> SwapStatus {
        match &self {
            SwapInfo::Forward(deets) => SwapStatus::Forward(deets.status.clone()),
            SwapInfo::Reverse(deets) => SwapStatus::Reverse(deets.status.clone()),
        }
    }

    pub fn get_user_principal(&self) -> Principal {
        match &self {
            SwapInfo::Forward(deets) => deets.gldt_receiver.owner,
            SwapInfo::Reverse(deets) => deets.user,
        }
    }

    pub fn get_nft_id(&self) -> NftID {
        match &self {
            SwapInfo::Forward(details) => details.nft_id.clone(),
            SwapInfo::Reverse(details) => details.nft_id.clone(),
        }
    }

    pub fn get_nft_canister(&self) -> Principal {
        match &self {
            SwapInfo::Forward(details) => details.nft_canister.clone(),
            SwapInfo::Reverse(details) => details.nft_canister.clone(),
        }
    }

    pub fn is_swap_over_time_threshold(&self) -> bool {
        let now = timestamp_millis();

        match self {
            // although a swap can be technically stuck in our system. we will never re-process a stuck forward swap
            // because the nft canister will release funds and cancel sale after 1 minute
            SwapInfo::Forward(details) => {
                let threshold =
                    details.created_at + MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES;
                let is_old = now > threshold;

                return is_old;
            }
            SwapInfo::Reverse(details) => {
                let threshold =
                    details.created_at + MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES;
                let is_old = now > threshold;

                return is_old;
            }
        }
    }

    pub fn get_swap_id(&self) -> SwapId {
        match &self {
            SwapInfo::Forward(deets) => SwapId(deets.nft_id.clone(), deets.index.clone()),
            SwapInfo::Reverse(deets) => SwapId(deets.nft_id.clone(), deets.index.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SwapType {
    Forward,
    Reverse,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BlockFailReason {
    InvalidOperation,
    NotFound,
    QueryRequestFailed,
    ReceiverNotCorrectAccountId(Subaccount),
    SenderNotPrincipalDefaultSubaccount(AccountIdentifier),
    AmountTooSmall,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BurnFailReason {
    TransferError(TransferError),
    CallError(String),
    TokenBalanceAndSwapRequestDontMatch,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum TransferFailReason {
    TransferFromError(TransferFromError),
    TransferError(TransferErrorIcrc),
    CallError(String),
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, PartialEq, Eq)]
pub enum ImpossibleErrorReason {
    PrincipalNotFound,
    AmountNotFound,
    NFTResponseInvalid,
}

// -----------------
//     Forward swap
// -----------------

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapDetailForward {
    pub index: Nat,
    pub sale_id: String,
    pub nft_id: NftID,
    pub nft_id_string: String,
    pub status: SwapStatusForward,
    pub created_at: TimestampMillis,
    pub tokens_to_mint: GldtNumTokens,
    pub escrow_sub_account: Subaccount,
    pub gldt_receiver: Account,
    pub nft_canister: Principal,
}

impl Default for SwapDetailForward {
    fn default() -> Self {
        Self {
            index: SwapIndex::default(),
            sale_id: Default::default(),
            nft_id: Default::default(),
            nft_id_string: String::default(),
            status: SwapStatusForward::Init,
            created_at: Default::default(),
            tokens_to_mint: Default::default(),
            escrow_sub_account: Default::default(),
            gldt_receiver: Account {
                owner: Principal::anonymous(),
                subaccount: None,
            },
            nft_canister: Principal::anonymous(),
        }
    }
}

impl SwapDetailForward {
    pub fn update_escrow_account(&mut self, subaccount: Subaccount) {
        self.escrow_sub_account = subaccount;
    }

    pub fn update_sale_id(&mut self, sale_id: String) {
        self.sale_id = sale_id;
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatusForward {
    Init,
    NotificationInProgress,
    NotificationFailed(NotificationError),
    MintRequest,
    MintInProgress,
    MintFailed(MintError),
    BidRequest,
    BidInProgress,
    BidFail(BidFailError),
    BurnFeesRequest,
    BurnFeesInProgress,
    BurnFeesFailed(BurnFeesError),
    DepositRecoveryRequest(Box<SwapStatusForward>),
    DepositRecoveryInProgress(Box<SwapStatusForward>),
    DepositRecoveryFailed(Box<SwapStatusForward>, DepositRecoveryError),
    Complete,
    Failed(SwapErrorForward),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum MintError {
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BurnFeesError {
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BidFailError {
    TransferFailed(String),
    CallError(String),
    UnexpectedError(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum DepositRecoveryError {
    CantRecover(String),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapErrorForward {
    NotificationFailed(NotificationError),
    MintFailed(MintError),
    BidFailed(BidFailError),
    UnexpectedError(ImpossibleErrorReason),
    DepositRecoveryFailed(DepositRecoveryError),
    Expired(Box<SwapStatusForward>),
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, PartialEq, Eq)]
pub enum NotificationError {
    OrigynStringIdDoesNotMatch(String),
    CollectionDoesNotMatch(String),
    SellerAndReceiverDoesNotMatch(String),
    InvalidEscrowSubaccount(String),
    InvalidTokenSpec,
    InvalidTokenAmount,
    InvalidSaleSubaccount,
    SellerIsNotPrincipalOrAccount(String),
    TooManyPrincipalsInAllowList,
    AllowListDoesNotContainCorrectPrincipal,
    InvalidCustomAskFeature,
    InvalidPricingConfig,
    TimeoutInvalid(String),
    SaleIDStringTooLong(String),
}
// -----------------
//     Reverse swap
// -----------------

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapDetailReverse {
    pub index: Nat,
    pub nft_id: NftID,
    pub nft_id_string: String,
    pub nft_canister: Principal,
    pub status: SwapStatusReverse,
    pub created_at: TimestampMillis,
    pub tokens_to_receive: GldtNumTokens,
    pub swap_fee: Nat,
    pub transfer_fees: Nat,
    pub user: Principal,
}

impl Default for SwapDetailReverse {
    fn default() -> Self {
        Self {
            index: SwapIndex::default(),
            nft_id: Default::default(),
            nft_id_string: String::default(),
            nft_canister: Principal::anonymous(),
            status: SwapStatusReverse::Init,
            created_at: Default::default(),
            tokens_to_receive: GldtNumTokens::default(),
            swap_fee: Nat::from(100_000_000u64),
            user: Principal::anonymous(),
            transfer_fees: Nat::from(GLDT_TX_FEE * 2),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatusReverse {
    Init,
    EscrowRequest,
    EscrowRequestInProgress,
    EscrowFailed(EscrowError),
    NftTransferRequest,
    NftTransferRequestInProgress,
    NftTransferFailed(NftTransferError),
    RefundRequest,
    RefundRequestInProgress,
    RefundFailed(RefundError),
    BurnRequest,
    BurnRequestInProgress,
    BurnFailed(BurnError),
    FeeTransferRequest,
    FeeTransferRequestInProgress,
    FeeTransferFailed(FeeTransferError),
    Complete,
    Failed(SwapErrorReverse),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapErrorReverse {
    NftValidationFailed(Vec<NftValidationError>),
    LockFailed(LockError),
    EscrowFailed(EscrowError),
    NftTransferFailed(NftTransferError),
    BurnFailed(BurnError),
    FeeTransferFailed(FeeTransferError),
    Refunded(Box<SwapStatusReverse>),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum FeeTransferError {
    TransferError(TransferErrorIcrc),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum EscrowError {
    ApproveError(ApproveError),
    RequestFailed(String),
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum LockError {
    NftAlreadyLocked(Vec<NftID>),
    NftNotLocked,
    UnexpectedError(),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum NftValidationError {
    InvalidNftWeight,
    WeightParseError,
    CanisterInvalid,
    InvalidGldtTokensFromWeight,
    CantGetOrigynID(String),
    NotOwnedBySwapCanister,
    CantVerifySwapCanisterOwnsNft,
    NftIdStringTooLong(String),
    UserDoesNotHaveTheRequiredGLDT(String),
    CantValidateUserBalanceOfGLDT(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum NftTransferError {
    InvalidFee(String),
    ApprovalError(ApproveError),
    ApprovalCallError(String),
    TransferFailed(String),
    UnexpectedError(ImpossibleErrorReason),
    FailedToGetOgyFeeAllowance(String),
    CallError(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BurnError {
    CallError(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum RefundError {
    TransferFailed(TransferErrorIcrc),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatus {
    Forward(SwapStatusForward),
    Reverse(SwapStatusReverse),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceStatus {
    Up,
    Down(ServiceDownReason),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceDownReason {
    Initializing,
    ArchiveRelated(ArchiveDownReason),
    ActiveSwapCapacityFull,
    LowOrigynToken(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveStatus {
    Up,
    Down(ArchiveDownReason),
    Upgrading,
    Initializing,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveDownReason {
    NewArchiveError(NewArchiveError),
    Upgrading,
    UpgradingArchivesFailed(String),
    ActiveSwapCapacityFull,
    NoArchiveCanisters(String),
    LowOrigynToken(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum NewArchiveError {
    FailedToSerializeInitArgs(String),
    CreateCanisterError(String),
    InstallCodeError(String),
    CantFindControllers(String),
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseSwapInfoError {
    #[error("expected a map ICRC3Value for SwapInfo")]
    NotAMap,
    #[error("missing field: {0}")]
    Missing(&'static str),
    #[error("invalid type for field: {0}")]
    InvalidType(&'static str),
    #[error("invalid principal in field {0}: {1}")]
    InvalidPrincipal(&'static str, String),
    #[error("invalid nat (too large) in field: {0}")]
    NatTooLarge(&'static str),
    #[error("invalid subaccount length for field {0}: expected 32, got {1}")]
    BadSubaccount(&'static str, usize),
    #[error("unsupported/opaque field {0}: {1}")]
    Unsupported(&'static str, String),
}

// impl TryFrom<ICRC3Value> for SwapInfo {
//     type Error = ParseSwapInfoError;

//     fn try_from(value: ICRC3Value) -> Result<Self, Self::Error> {
//         let m = as_map(&value).ok_or(ParseSwapInfoError::NotAMap)?;

//         // swap_type: "forward" | "reverse"
//         let swap_type = as_text(get(m, "swap_type")?, "swap_type")?;
//         match swap_type {
//             "forward" => parse_forward(m),
//             "reverse" => parse_reverse(m),
//             other => Err(ParseSwapInfoError::Unsupported(
//                 "swap_type",
//                 other.to_string(),
//             )),
//         }
//     }
// }

// fn as_map(v: &ICRC3Value) -> Option<&BTreeMap<String, ICRC3Value>> {
//     if let ICRC3Value::Map(m) = v {
//         Some(m)
//     } else {
//         None
//     }
// }

// fn get<'a>(
//     m: &'a BTreeMap<String, ICRC3Value>,
//     k: &str,
// ) -> Result<&'a ICRC3Value, ParseSwapInfoError> {
//     m.get(k).ok_or(ParseSwapInfoError::Missing(Box::leak(
//         k.to_string().into_boxed_str(),
//     )))
// }

// fn as_text<'a>(v: &'a ICRC3Value, field: &'static str) -> Result<&'a str, ParseSwapInfoError> {
//     if let ICRC3Value::Text(s) = v {
//         Ok(s)
//     } else {
//         Err(ParseSwapInfoError::InvalidType(field))
//     }
// }

// fn as_nat<'a>(v: &'a ICRC3Value, field: &'static str) -> Result<&'a Nat, ParseSwapInfoError> {
//     if let ICRC3Value::Nat(n) = v {
//         Ok(n)
//     } else {
//         Err(ParseSwapInfoError::InvalidType(field))
//     }
// }

// fn as_blob<'a>(v: &'a ICRC3Value, field: &'static str) -> Result<&'a [u8], ParseSwapInfoError> {
//     if let ICRC3Value::Blob(b) = v {
//         Ok(b)
//     } else {
//         Err(ParseSwapInfoError::InvalidType(field))
//     }
// }

// fn nat_to_u64(n: &Nat, field: &'static str) -> Result<u64, ParseSwapInfoError> {
//     // candid::Nat is BigUint. Convert only if it fits in u64.
//     let limbs = n.0.to_u64_digits();
//     match limbs.as_slice() {
//         [] => Ok(0),
//         [lo] => Ok(*lo),
//         _ => Err(ParseSwapInfoError::NatTooLarge(field)),
//     }
// }

// fn parse_forward(m: &BTreeMap<String, ICRC3Value>) -> Result<SwapInfo, ParseSwapInfoError> {
//     let sale_id = as_text(get(m, "sale_id")?, "sale_id")?.to_string();
//     let index = as_nat(get(m, "index")?, "index")?.clone();
//     let nft_id_nat = as_nat(get(m, "nft_id")?, "nft_id")?.clone();
//     let nft_id_string = as_text(get(m, "nft_id_string")?, "nft_id_string")?.to_string();
//     let created_at_nat = as_nat(get(m, "created_at")?, "created_at")?;
//     let created_at = nat_to_u64(created_at_nat, "created_at")?;
//     let tokens_to_mint_val = as_nat(get(m, "tokens_to_mint")?, "tokens_to_mint")?.clone();
//     let tokens_to_mint_with_fee_val = as_nat(
//         get(m, "tokens_to_mint_with_fee")?,
//         "tokens_to_mint_with_fee",
//     )?
//     .clone();

//     // escrow_sub_account: Blob([u8; 32])
//     let escrow_blob = as_blob(get(m, "escrow_sub_account")?, "escrow_sub_account")?;
//     if escrow_blob.len() != 32 {
//         return Err(ParseSwapInfoError::BadSubaccount(
//             "escrow_sub_account",
//             escrow_blob.len(),
//         ));
//     }
//     let mut escrow_sub = [0u8; 32];
//     escrow_sub.copy_from_slice(escrow_blob);

//     // gldt_receiver was serialized via Debug string => not reliably parseable.
//     // Fail loudly so you notice if you depend on it:
//     if let Ok(s) = as_text(get(m, "gldt_receiver")?, "gldt_receiver") {
//         return Err(ParseSwapInfoError::Unsupported(
//             "gldt_receiver",
//             s.to_string(),
//         ));
//     }

//     // nft_canister: Principal (text)
//     let nft_canister_text = as_text(get(m, "nft_canister")?, "nft_canister")?;
//     let nft_canister = Principal::from_text(nft_canister_text)
//         .map_err(|e| ParseSwapInfoError::InvalidPrincipal("nft_canister", e.to_string()))?;

//     // status: stored as Debug string; default to Init to avoid lying.
//     // If you later add structured status, parse it here.
//     // let status_str = as_text(get(m, "status")?, "status")?;
//     let status = SwapStatusForward::Init;

//     let info = SwapDetailForward {
//         sale_id,
//         index,
//         nft_id: NftID(nft_id_nat),
//         nft_id_string,
//         status,
//         created_at,
//         tokens_to_mint: GldtNumTokens {
//             value: tokens_to_mint_val,
//             value_with_fee: tokens_to_mint_with_fee_val,
//         },
//         escrow_sub_account: escrow_sub,
//         gldt_receiver: Account {
//             owner: Principal::anonymous(),
//             subaccount: None,
//         }, // opaque for now
//         nft_canister,
//     };

//     Ok(SwapInfo::Forward(info))
// }

// fn parse_reverse(m: &BTreeMap<String, ICRC3Value>) -> Result<SwapInfo, ParseSwapInfoError> {
//     let index = as_nat(get(m, "index")?, "index")?.clone();
//     let nft_id_nat = as_nat(get(m, "nft_id")?, "nft_id")?.clone();
//     let nft_id_string = as_text(get(m, "nft_id_string")?, "nft_id_string")?.to_string();
//     let nft_canister_text = as_text(get(m, "nft_canister")?, "nft_canister")?;
//     let nft_canister = Principal::from_text(nft_canister_text)
//         .map_err(|e| ParseSwapInfoError::InvalidPrincipal("nft_canister", e.to_string()))?;
//     let created_at_nat = as_nat(get(m, "created_at")?, "created_at")?;
//     let created_at = nat_to_u64(created_at_nat, "created_at")?;

//     let tokens_to_receive_val = as_nat(get(m, "tokens_to_receive")?, "tokens_to_receive")?.clone();
//     let tokens_to_receive_with_fee_val = as_nat(
//         get(m, "tokens_to_receive_with_fee")?,
//         "tokens_to_receive_with_fee",
//     )?
//     .clone();

//     let swap_fee = as_nat(get(m, "swap_fee")?, "swap_fee")?.clone();
//     let transfer_fees = as_nat(get(m, "transfer_fees")?, "transfer_fees")?.clone();

//     let user_text = as_text(get(m, "user")?, "user")?;
//     let user = Principal::from_text(user_text)
//         .map_err(|e| ParseSwapInfoError::InvalidPrincipal("user", e.to_string()))?;

//     // status stored as Debug -> default to Init
//     let status = SwapStatusReverse::Init;

//     let info = SwapDetailReverse {
//         index,
//         nft_id: NftID(nft_id_nat),
//         nft_id_string,
//         nft_canister,
//         status,
//         created_at,
//         tokens_to_receive: GldtNumTokens {
//             value: tokens_to_receive_val,
//             value_with_fee: tokens_to_receive_with_fee_val,
//         },
//         swap_fee,
//         transfer_fees,
//         user,
//     };

//     Ok(SwapInfo::Reverse(info))
// }
