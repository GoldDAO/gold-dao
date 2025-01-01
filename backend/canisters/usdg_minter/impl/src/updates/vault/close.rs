use crate::guard::GuardPrincipal;
use crate::lifecycle::tasks::{schedule_now, TaskType};
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
use usdg_minter_api::VaultError;

#[update]
async fn close_vault(vault_id: u64) -> Result<Option<u64>, VaultError> {
    check_postcondition(_close_vault(vault_id)).await
}

async fn _close_vault(vault_id: u64) -> Result<Option<u64>, VaultError> {
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let vault: Vault = read_state(|s| s.get_vault(vault_id)).ok_or(VaultError::VaultNotFound)?;

    if vault.owner.owner != caller {
        return Err(VaultError::CallerNotOwner);
    }

    if vault.borrowed_amount == USDG::ZERO {
        mutate_state(|s| {
            process_event(
                s,
                EventType::Close {
                    vault_id: vault.vault_id,
                    block_index: None,
                },
            )
        });
        return Ok(None);
    }

    let usdg_ledger_id = read_state(|s| s.usdg_ledger_id);

    log!(INFO, "[close_vault] closing vault with id {vault_id}");

    match transfer_from(
        vault.owner,
        ic_cdk::id(),
        Nat::from(vault.borrowed_amount),
        None,
        usdg_ledger_id,
    )
    .await
    {
        Ok(block_index) => {
            log!(
                INFO,
                "[close_vault] Succesfully closed vault {} at index {block_index}",
                vault.vault_id
            );
            mutate_state(|s| {
                process_event(
                    s,
                    EventType::Close {
                        vault_id: vault.vault_id,
                        block_index: Some(block_index),
                    },
                )
            });
            schedule_now(TaskType::ProcessPendingTransfer);
            Ok(Some(block_index))
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
