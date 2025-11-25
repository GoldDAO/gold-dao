use crate::nft::Nft;
use crate::swap_canister_config::GeneralFractionalizationConfig;
use bity_ic_canister_time::{timestamp_millis, HOUR_IN_MS, MINUTE_IN_MS};
use candid::{CandidType, Nat};
use icrc_ledger_types::{
    icrc::generic_value::ICRC3Value,
    icrc1::{account::Account, transfer::TransferError as TransferErrorIcrc},
    icrc2::transfer_from::TransferFromError,
};
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::{Milliseconds, TimestampMillis};

pub const STALE_SWAP_TIME_THRESHOLD_MINUTES: u64 = 3;
// ----------------------
//     CRON JOB INTERVALS & Retrys & delays
// ----------------------
pub const MANAGE_GLDT_SUPPLY_INTERVAL: Milliseconds = HOUR_IN_MS * 6;
pub const MANAGE_GLDT_SUPPLY_RETRY_DELAY: Milliseconds = MINUTE_IN_MS * 3;
pub const MANAGE_ARCHIVE_CYCLE_INTERVAL: Milliseconds = MINUTE_IN_MS * 10;
pub const MANAGE_NEW_ARCHIVES_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_OGY_FEE_ACCOUNTS_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_SERVICE_STATUS_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_STALE_SWAPS_INTERVAL: Milliseconds = MINUTE_IN_MS;

pub type SwapIndex = Nat;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapInfo {
    pub index: SwapIndex,

    pub status: SwapStatus,
    pub created_at: TimestampMillis,
    pub swap_type: SwapType,
    pub user_account: Account,

    pub nft: Nft,
    pub tokens_amount: GeneralFractionalizationConfig,
}

impl SwapInfo {
    pub fn new(
        index: SwapIndex,
        swap_type: SwapType,
        caller_account: Account,
        nft: Nft,
        tokens_to_burn: GeneralFractionalizationConfig,
    ) -> Self {
        SwapInfo {
            index,
            status: SwapStatus::Init,
            created_at: timestamp_millis(),
            swap_type,
            user_account: caller_account,
            nft,
            tokens_amount: tokens_to_burn,
        }
    }

    pub fn is_swap_over_time_threshold(&self) -> bool {
        let now = timestamp_millis();
        let threshold = self.created_at + MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES;
        now > threshold
    }

    pub fn update_status(&mut self, new_status: SwapStatus) -> Result<(), String> {
        use SwapStatus::*;
        use SwapType::*;

        match (self.swap_type, &self.status, &new_status) {
            // ========================
            // Forward swap transitions
            // ========================
            (Forward, Init, NftTransferredFrom)
            | (Forward, Init, NftTransferFromFailed(_))
            | (Forward, NftTransferredFrom, Minted)
            | (Forward, NftTransferredFrom, MintFailed(_))
            | (Forward, MintFailed(_), Reimbursed)
            | (Forward, MintFailed(_), ReimburseFailed(_))
            | (Forward, ReimburseFailed(_), Reimbursed)
            | (Forward, Minted, Complete) => {
                self.status = new_status;
                Ok(())
            }

            // ========================
            // Reverse swap transitions
            // ========================
            (Reverse, Init, Burned)
            | (Reverse, Init, BurnFailed(_))
            | (Reverse, Burned, NftTransferred)
            | (Reverse, Burned, NftTransferFailed(_))
            | (Reverse, NftTransferFailed(_), Reimbursed)
            | (Reverse, NftTransferFailed(_), ReimburseFailed(_))
            | (Reverse, ReimburseFailed(_), Reimbursed)
            | (Reverse, NftTransferred, Complete) => {
                self.status = new_status;
                Ok(())
            }

            // ========================
            // Allow Complete/Reimbursed to stay the same
            // ========================
            (_, Complete, Complete) | (_, Reimbursed, Reimbursed) => Ok(()),

            // ========================
            // Catch-all for invalid transitions
            // ========================
            _ => Err(format!(
                "Invalid transition for {:?} swap from {:?} to {:?}",
                self.swap_type, self.status, new_status
            )),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwapType {
    Forward,
    Reverse,
}

impl std::fmt::Display for SwapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SwapType::Forward => "forward_swap",
            SwapType::Reverse => "reverse_swap",
        };
        write!(f, "{}", s)
    }
}

#[derive(
    Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode,
)]
pub enum SwapStatus {
    #[n(0)]
    Init,
    #[n(1)]
    Minted,
    #[n(2)]
    MintFailed(#[n(0)] String),
    #[n(3)]
    Burned,
    #[n(4)]
    BurnFailed(#[n(0)] String),
    #[n(5)]
    NftTransferred,
    #[n(6)]
    NftTransferFailed(#[n(0)] String),
    #[n(7)]
    NftTransferredFrom,
    #[n(8)]
    NftTransferFromFailed(#[n(0)] String),
    #[n(9)]
    Complete,
    #[n(10)]
    Failed(#[n(0)] String),
    #[n(11)]
    Reimbursed,
    #[n(12)]
    ReimburseFailed(#[n(0)] String),
}
impl From<SwapStatus> for ICRC3Value {
    fn from(val: SwapStatus) -> Self {
        match val {
            SwapStatus::Init => ICRC3Value::Text("Init".into()),
            SwapStatus::Minted => ICRC3Value::Text("Minted".into()),
            SwapStatus::MintFailed(s) => ICRC3Value::Text(format!("MintFailed:{}", s)),
            SwapStatus::Burned => ICRC3Value::Text("Burned".into()),
            SwapStatus::BurnFailed(s) => ICRC3Value::Text(format!("BurnFailed:{}", s)),
            SwapStatus::NftTransferred => ICRC3Value::Text("NftTransferred".into()),
            SwapStatus::NftTransferFailed(s) => {
                ICRC3Value::Text(format!("NftTransferFailed:{}", s))
            }
            SwapStatus::NftTransferredFrom => ICRC3Value::Text("NftTransferredFrom".into()),
            SwapStatus::NftTransferFromFailed(s) => {
                ICRC3Value::Text(format!("NftTransferFromFailed:{}", s))
            }
            SwapStatus::Complete => ICRC3Value::Text("Complete".into()),
            SwapStatus::Failed(s) => ICRC3Value::Text(format!("Failed:{}", s)),
            SwapStatus::Reimbursed => ICRC3Value::Text("Reimbursed".into()),
            SwapStatus::ReimburseFailed(s) => ICRC3Value::Text(format!("ReimburseFailed:{}", s)),
        }
    }
}

impl TryFrom<&ICRC3Value> for SwapStatus {
    type Error = String;

    fn try_from(value: &ICRC3Value) -> Result<Self, Self::Error> {
        match value {
            ICRC3Value::Text(s) => {
                match s.as_str() {
                    "Init" => Ok(SwapStatus::Init),
                    "Minted" => Ok(SwapStatus::Minted),
                    "Burned" => Ok(SwapStatus::Burned),
                    "NftTransferred" => Ok(SwapStatus::NftTransferred),
                    "NftTransferredFrom" => Ok(SwapStatus::NftTransferredFrom),
                    "Complete" => Ok(SwapStatus::Complete),
                    "Reimbursed" => Ok(SwapStatus::Reimbursed),
                    other => {
                        // Handle variants with extra info like "MintFailed:<reason>"
                        if let Some(stripped) = other.strip_prefix("MintFailed:") {
                            Ok(SwapStatus::MintFailed(stripped.to_string()))
                        } else if let Some(stripped) = other.strip_prefix("BurnFailed:") {
                            Ok(SwapStatus::BurnFailed(stripped.to_string()))
                        } else if let Some(stripped) = other.strip_prefix("NftTransferFailed:") {
                            Ok(SwapStatus::NftTransferFailed(stripped.to_string()))
                        } else if let Some(stripped) = other.strip_prefix("NftTransferFromFailed:")
                        {
                            Ok(SwapStatus::NftTransferFromFailed(stripped.to_string()))
                        } else if let Some(stripped) = other.strip_prefix("Failed:") {
                            Ok(SwapStatus::Failed(stripped.to_string()))
                        } else if let Some(stripped) = other.strip_prefix("ReimburseFailed:") {
                            Ok(SwapStatus::ReimburseFailed(stripped.to_string()))
                        } else {
                            Err(format!("Unknown SwapStatus string: {}", s))
                        }
                    }
                }
            }
            _ => Err("ICRC3Value is not a Text variant".to_string()),
        }
    }
}
impl From<SwapInfo> for ICRC3Value {
    fn from(val: SwapInfo) -> Self {
        let mut map = BTreeMap::new();
        map.insert("index".to_string(), ICRC3Value::Nat(Nat::from(val.index.0)));
        map.insert("status".to_string(), val.status.into());
        map.insert(
            "created_at".to_string(),
            ICRC3Value::Nat(Nat::from(val.created_at)),
        );
        map.insert(
            "swap_type".to_string(),
            ICRC3Value::Text(match val.swap_type {
                SwapType::Forward => "Forward".into(),
                SwapType::Reverse => "Reverse".into(),
            }),
        );
        map.insert(
            "user_account".to_string(),
            ICRC3Value::Text(val.user_account.owner.to_text()), // could later be expanded into a map if needed
        );
        map.insert("nft".to_string(), val.nft.into());
        map.insert("tokens_amount".to_string(), val.tokens_amount.into());

        ICRC3Value::Map(map)
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum MintError {
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
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
