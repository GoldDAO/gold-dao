use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferError;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Deserialize;

pub mod lifecycle;
pub mod queries;
pub mod types;
pub mod updates;

pub const USDG_TRANSFER_FEE: u64 = 1_000_000;

#[derive(CandidType, Deserialize, Debug, Eq, PartialEq)]
pub enum ApiFeeBucket {
    Low,
    Medium,
    High,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum VaultError {
    TransferFromError(TransferFromError),
    TransferError(TransferError),
    AnonymousCaller,
    AmountTooLow { minimum_amount: u64 },
    NoRecentGoldPrice,
    BorrowedAmountTooBig { maximum_borrowable_amount: u64 },
    RepayingAmountTooBig { maximum_repayable_amount: u64 },
    VaultNotFound,
    CallerNotOwner,
    AlreadyProcessing,
    TooManyConcurrentRequests,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum LiquidityError {
    TransferFromError(TransferFromError),
    TransferError(TransferError),
    AnonymousCaller,
    BalanceTooLow { balance: u64 },
    NotEnoughGLDT { minimum_amount: u64 },
    AlreadyProcessing,
    TooManyConcurrentRequests,
}

pub enum GuardError {
    AlreadyProcessing,
    TooManyConcurrentRequests,
}

impl From<GuardError> for LiquidityError {
    fn from(g: GuardError) -> LiquidityError {
        match g {
            GuardError::AlreadyProcessing => LiquidityError::AlreadyProcessing,
            GuardError::TooManyConcurrentRequests => LiquidityError::TooManyConcurrentRequests,
        }
    }
}

impl From<GuardError> for VaultError {
    fn from(g: GuardError) -> VaultError {
        match g {
            GuardError::AlreadyProcessing => VaultError::AlreadyProcessing,
            GuardError::TooManyConcurrentRequests => VaultError::TooManyConcurrentRequests,
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Eq, PartialEq)]
pub struct ApiVault {
    pub vault_id: u64,
    pub owner: Account,
    pub borrowed_amount: u64,
    pub margin_amount: u64,
    pub fee_bucket: ApiFeeBucket,
}
