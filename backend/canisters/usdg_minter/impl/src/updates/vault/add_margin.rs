use crate::guard::GuardPrincipal;
use crate::logs::INFO;
use crate::management::transfer_from;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::vault::Vault;
use crate::GLDT;
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use usdg_minter_api::updates::add_margin_to_vault::AddMarginArg;
use usdg_minter_api::VaultError;

#[update]
async fn add_margin_to_vault(arg: AddMarginArg) -> Result<u64, VaultError> {
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let vault: Vault =
        read_state(|s| s.get_vault(arg.vault_id)).ok_or(VaultError::VaultNotFound)?;

    if vault.owner.owner != caller {
        return Err(VaultError::CallerNotOwner);
    }

    let margin_amount = GLDT::from_e8s(arg.margin_amount);
    let gldt_ledger_id = read_state(|s| s.gldt_ledger_id);

    log!(INFO, "[add_margin_to_vault] {arg}");

    match transfer_from(
        vault.owner,
        ic_cdk::id(),
        Nat::from(arg.margin_amount),
        None,
        gldt_ledger_id,
    )
    .await
    {
        Ok(block_index) => {
            log!(INFO, "[add_margin_to_vault] Succesfully added margin for vault {} at index {block_index}", vault.vault_id);
            mutate_state(|s| s.record_add_margin_to_vault(vault.vault_id, margin_amount));
            Ok(block_index)
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
