use crate::guard::GuardPrincipal;
use crate::lifecycle::timer::check_postcondition;
use crate::logs::INFO;
use crate::management::transfer_from;
use crate::state::audit::process_event;
use crate::state::event::EventType;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::vault::Vault;
use crate::USDG;
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use usdg_minter_api::updates::repay_debt_to_vault::RepayDebtArg;
use usdg_minter_api::VaultError;

#[update]
async fn repay_debt_to_vault(arg: RepayDebtArg) -> Result<u64, VaultError> {
    check_postcondition(_repay_debt_to_vault(arg)).await
}

async fn _repay_debt_to_vault(arg: RepayDebtArg) -> Result<u64, VaultError> {
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let vault: Vault =
        read_state(|s| s.get_vault(arg.vault_id)).ok_or(VaultError::VaultNotFound)?;

    if vault.owner.owner != caller {
        return Err(VaultError::CallerNotOwner);
    }

    let debt_amount = USDG::from_e8s(arg.debt_amount);

    if vault.borrowed_amount < debt_amount {
        return Err(VaultError::RepayingAmountTooBig {
            maximum_repayable_amount: vault.borrowed_amount.0,
        });
    }

    let usdg_ledger_id = read_state(|s| s.usdg_ledger_id);

    log!(INFO, "[repay_debt_to_vault] {arg}");

    match transfer_from(
        vault.owner,
        ic_cdk::id(),
        Nat::from(arg.debt_amount),
        None,
        usdg_ledger_id,
    )
    .await
    {
        Ok(block_index) => {
            log!(INFO, "[repay_debt_to_vault] Succesfully repayed debt for vault {} at index {block_index}", vault.vault_id);
            mutate_state(|s| {
                process_event(
                    s,
                    EventType::Repay {
                        vault_id: arg.vault_id,
                        debt: debt_amount,
                        block_index,
                    },
                )
            });
            Ok(block_index)
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
