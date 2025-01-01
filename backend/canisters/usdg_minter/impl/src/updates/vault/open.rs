use crate::guard::GuardPrincipal;
use crate::lifecycle::tasks::{schedule_now, TaskType};
use crate::lifecycle::timer::check_postcondition;
use crate::logs::INFO;
use crate::management::transfer_from;
use crate::numeric::{GLDT, USDG};
use crate::state::audit::process_event;
use crate::state::event::EventType;
use crate::state::{mutate_state, read_state};
use crate::updates::{reject_anonymous_caller, VaultError};
use crate::MINIMUM_MARGIN_AMOUNT;
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::updates::open_vault::{OpenVaultArg, OpenVaultSuccess};

#[update]
async fn open_vault(arg: OpenVaultArg) -> Result<OpenVaultSuccess, VaultError> {
    check_postcondition(_open_vault(arg)).await
}

async fn _open_vault(arg: OpenVaultArg) -> Result<OpenVaultSuccess, VaultError> {
    // Check anonymous caller
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;

    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    // Check minimum margin amount
    if GLDT::from_e8s(arg.margin_amount) < MINIMUM_MARGIN_AMOUNT {
        return Err(VaultError::AmountTooLow {
            minimum_amount: MINIMUM_MARGIN_AMOUNT.0,
        });
    }

    // Check if borrowed amount makes sense
    let usdg_borrowed = USDG::from_e8s(arg.borrowed_amount);
    let gldt_margin = GLDT::from_e8s(arg.margin_amount);
    read_state(|s| s.check_max_borrowable_amount(gldt_margin, usdg_borrowed))?;

    let from = Account {
        owner: caller,
        subaccount: arg.maybe_subaccount,
    };
    let gldt_ledger_id = read_state(|s| s.gldt_ledger_id);

    log!(
        INFO,
        "[open_vault] {caller} requested vault opening with args: {arg}",
    );
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
            let vault_id = mutate_state(|s| {
                let vault_id = s.next_vault_id;
                process_event(
                    s,
                    EventType::OpenVault {
                        owner: from,
                        margin_amount: arg.margin_amount.into(),
                        borrowed_amount: arg.borrowed_amount.into(),
                        fee_bucket: arg.fee_bucket.into(),
                        block_index,
                    },
                );
                vault_id
            });
            log!(
                INFO,
                "[open_vault] {caller} successfully opened vault at index {block_index} with id: {vault_id}",
            );
            schedule_now(TaskType::ProcessPendingTransfer);
            Ok(OpenVaultSuccess {
                block_index,
                vault_id,
            })
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
