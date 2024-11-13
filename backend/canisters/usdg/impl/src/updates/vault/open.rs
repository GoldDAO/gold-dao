use crate::management::transfer_from;
use crate::numeric::{GLDT, USDG};
use crate::state::{mutate_state, read_state};
use crate::updates::{reject_anonymous_caller, VaultError};
use crate::vault::FeeBucket;
use crate::MINIMUM_MARGIN_AMOUNT;
use candid::{CandidType, Nat, Principal};
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Deserialize;
use usdg_minter_api::updates::open_vault::{OpenVaultArg, OpenVaultSuccess};

#[update]
async fn open_vault(arg: OpenVaultArg) -> Result<OpenVaultSuccess, VaultError> {
    // Check anonymous caller
    reject_anonymous_caller()?;

    // Check minimum margin amount
    if GLDT::from_e8s(arg.margin_amount) < MINIMUM_MARGIN_AMOUNT {
        return Err(VaultError::AmountTooLow {
            minimum_amount: MINIMUM_MARGIN_AMOUNT.0,
        });
    }

    // TODO: Check if borrowed amount makes sense
    // read_state(|s| s.are_valid_parameters(arg.borrowed_amount, arg.margin_amount))?;

    let from = Account {
        owner: ic_cdk::caller(),
        subaccount: arg.maybe_subaccount,
    };
    let gldt_ledger_id = read_state(|s| s.gldt_ledger_id);
    match transfer_from(
        from,
        ic_cdk::id(),
        Nat::from(arg.margin_amount),
        None,
        gldt_ledger_id,
    )
    .await
    {
        Ok(block_index) => {
            // TODO log success
            let vault_id = mutate_state(|s| {
                s.record_vault_creation(
                    from,
                    arg.borrowed_amount.into(),
                    arg.margin_amount.into(),
                    arg.fee_bucket,
                )
            });
            Ok(OpenVaultSuccess {
                block_index,
                vault_id,
            })
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
