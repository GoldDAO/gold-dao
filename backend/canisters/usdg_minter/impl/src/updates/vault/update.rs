use crate::guard::GuardPrincipal;
use crate::lifecycle::timer::check_postcondition;
use crate::logs::INFO;
use crate::state::audit::process_event;
use crate::state::event::EventType;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::vault::Vault;
use ic_canister_log::log;
use ic_cdk::update;
use usdg_minter_api::updates::update_vault::UpdateVaultArg;
use usdg_minter_api::VaultError;

#[update]
async fn update_vault(arg: UpdateVaultArg) -> Result<(), VaultError> {
    check_postcondition(_update_vault(arg)).await
}

async fn _update_vault(arg: UpdateVaultArg) -> Result<(), VaultError> {
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let vault: Vault =
        read_state(|s| s.get_vault(arg.vault_id)).ok_or(VaultError::VaultNotFound)?;

    if vault.owner.owner != caller {
        return Err(VaultError::CallerNotOwner);
    }

    if arg.new_owner.is_none() && arg.fee_bucket.is_none() {
        return Err(VaultError::NoChange);
    }

    log!(INFO, "[update_vault] {arg}");

    mutate_state(|s| {
        process_event(
            s,
            EventType::UpdateVault {
                vault_id: arg.vault_id,
                new_owner: arg.new_owner,
                fee_bucket: arg.fee_bucket.map(|f| f.into()),
            },
        )
    });

    Ok(())
}
