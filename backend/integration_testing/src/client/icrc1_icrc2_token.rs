use crate::{generate_pocket_query_call, generate_pocket_update_call};
use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use icrc_ledger_types::icrc2::allowance::{Allowance, AllowanceArgs};
use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};

use serde::Deserialize;

generate_pocket_query_call!(icrc1_balance_of);
generate_pocket_query_call!(icrc1_total_supply);
generate_pocket_query_call!(icrc2_allowance);
// Updates
generate_pocket_update_call!(icrc1_transfer);
generate_pocket_update_call!(icrc2_transfer_from);
generate_pocket_update_call!(icrc2_approve);

pub mod icrc1_balance_of {
    use super::*;

    pub type Args = Account;
    pub type Response = Nat;
}

pub mod icrc1_total_supply {
    use super::*;

    pub type Args = ();
    pub type Response = Nat;
}

pub mod icrc1_transfer {
    use super::*;

    pub type Args = TransferArg;
    #[derive(CandidType, Deserialize)]
    pub enum Response {
        Ok(Nat),
        Err(TransferError),
    }
}

pub mod icrc2_transfer_from {
    use super::*;
    pub type Args = TransferFromArgs;
    #[derive(CandidType, Deserialize)]
    pub enum Response {
        Ok(Nat),
        Err(TransferFromError),
    }
}

pub mod icrc2_approve {
    use super::*;

    pub type Args = ApproveArgs;
    #[derive(CandidType, Deserialize, Debug)]
    pub enum Response {
        Ok(Nat),
        Err(ApproveError),
    }
}

pub mod icrc2_allowance {
    use super::*;

    pub type Args = AllowanceArgs;
    pub type Response = Allowance;
}
