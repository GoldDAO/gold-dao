use crate::guard::GuardPrincipal;
use crate::logs::INFO;
use crate::management::transfer;
use crate::numeric::{DisplayAmount, USDG};
use crate::state::{mutate_state, read_state};
use crate::updates::{reject_anonymous_caller, VaultError};
use crate::vault::Vault;
use crate::MINIMUM_BORROW_AMOUNT;
use ic_canister_log::log;
use ic_cdk::update;
use usdg_minter_api::updates::borrow_from_vault::BorrowArg;

#[update]
async fn borrow_from_vault(arg: BorrowArg) -> Result<u64, VaultError> {
    // Check anonymous caller
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;

    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    // Check minimum margin amount
    if USDG::from_e8s(arg.borrowed_amount) < MINIMUM_BORROW_AMOUNT {
        return Err(VaultError::AmountTooLow {
            minimum_amount: MINIMUM_BORROW_AMOUNT.0,
        });
    }

    let vault: Vault =
        read_state(|s| s.get_vault(arg.vault_id)).ok_or(VaultError::VaultNotFound)?;

    if vault.owner.owner != ic_cdk::caller() {
        return Err(VaultError::CallerNotOwner);
    }

    // Check if borrowed amount makes sense
    let borrowed_amount = USDG::from_e8s(arg.borrowed_amount);
    let usdg_borrowed_after_operation = vault.borrowed_amount.checked_add(borrowed_amount).unwrap();
    read_state(|s| {
        s.check_max_borrowable_amount(vault.margin_amount, usdg_borrowed_after_operation)
    })?;

    log!(
        INFO,
        "[borrow_from_vault] {} borrowed with args: {arg}",
        ic_cdk::caller()
    );

    let usdg_ledger_id = read_state(|s| s.usdg_ledger_id);

    match transfer(vault.owner, borrowed_amount.into(), None, usdg_ledger_id).await {
        Ok(block_index) => {
            log!(
                INFO,
                "[borrow_from_vault] {} successfully borrowed {} USDG from vault with id {}",
                ic_cdk::caller(),
                DisplayAmount(borrowed_amount.0),
                vault.vault_id
            );
            mutate_state(|s| {
                // TODO when recording the event we should keep track of the block index
                s.record_borrow_from_vault(arg.vault_id, borrowed_amount)
            });
            Ok(block_index)
        }

        Err(e) => Err(VaultError::TransferError(e)),
    }
}
