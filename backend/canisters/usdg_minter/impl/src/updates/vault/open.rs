use crate::logs::INFO;
use crate::management::transfer_from;
use crate::numeric::{GLDT, USDG};
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
    // Check anonymous caller
    reject_anonymous_caller()?;

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
        owner: ic_cdk::caller(),
        subaccount: arg.maybe_subaccount,
    };
    let gldt_ledger_id = read_state(|s| s.gldt_ledger_id);

    log!(
        INFO,
        "[open_vault] {} requested vault opening with args: {}",
        ic_cdk::caller(),
        arg
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
                s.record_vault_creation(
                    from,
                    arg.borrowed_amount.into(),
                    arg.margin_amount.into(),
                    arg.fee_bucket,
                )
            });
            log!(
                INFO,
                "[open_vault] {} successfully opened vault at index {block_index} with id: {vault_id}",
                ic_cdk::caller(),
            );
            Ok(OpenVaultSuccess {
                block_index,
                vault_id,
            })
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
