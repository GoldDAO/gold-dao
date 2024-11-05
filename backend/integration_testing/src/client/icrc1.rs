use crate::{generate_pocket_query_call, generate_pocket_update_call};
use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};

// Queries
generate_pocket_query_call!(icrc1_balance_of);
generate_pocket_query_call!(icrc1_total_supply);

// Updates
generate_pocket_update_call!(icrc1_transfer);

pub mod icrc1_balance_of {
    use super::*;

    pub type Args = Account;
    pub type Response = Nat;
}

pub mod icrc1_transfer {
    use super::*;

    pub type Args = TransferArg;
    pub type Response = Result<Nat, TransferError>;
}

pub mod icrc1_total_supply {
    use super::*;

    pub type Args = ();
    pub type Response = Nat;
}

pub mod client {
    use super::*;
    use candid::Principal;
    use icrc_ledger_types::icrc1::account::Subaccount;
    use pocket_ic::PocketIc;
    use types::CanisterId;

    pub fn transfer(
        pic: &mut PocketIc,
        sender: Principal,
        ledger_canister_id: CanisterId,
        from: Option<Subaccount>,
        recipient: impl Into<Account>,
        amount: u128,
    ) -> icrc1_transfer::Response {
        icrc1_transfer(
            pic,
            sender,
            ledger_canister_id,
            &(icrc1_transfer::Args {
                from_subaccount: from,
                to: recipient.into(),
                fee: None,
                created_at_time: None,
                memo: None,
                amount: amount.into(),
            }),
        )
    }

    pub fn balance_of(
        pic: &PocketIc,
        ledger_canister_id: CanisterId,
        account: impl Into<Account>,
    ) -> icrc1_balance_of::Response {
        icrc1_balance_of(
            pic,
            Principal::anonymous(),
            ledger_canister_id,
            &account.into(),
        )
    }
}
