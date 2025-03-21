use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferArg, TransferError};
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub ledger_id: CanisterId,
    pub transfer_args: TransferArg,
}

pub type Response = Result<Result<BlockIndex, TransferError>, (RejectionCode, String)>;
