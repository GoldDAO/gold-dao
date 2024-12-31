use crate::guard::GuardPrincipal;
use crate::logs::INFO;
use crate::management::transfer_from;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::{MINIMUM_REDEEM_AMOUNT, USDG};
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::updates::redeem::RedeemArg;
use usdg_minter_api::VaultError;

#[update]
async fn redeem(arg: RedeemArg) -> Result<u64, VaultError> {
    reject_anonymous_caller().map_err(|_| VaultError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let from = Account {
        owner: caller,
        subaccount: arg.maybe_subaccount,
    };

    let redeem_amount = USDG::from_e8s(arg.amount);

    if redeem_amount < MINIMUM_REDEEM_AMOUNT {
        return Err(VaultError::AmountTooLow {
            minimum_amount: redeem_amount.0,
        });
    }

    let usdg_ledger_id = read_state(|s| s.usdg_ledger_id);

    log!(INFO, "[redeem] {arg}");

    match transfer_from(
        from,
        ic_cdk::id(),
        Nat::from(arg.amount),
        None,
        usdg_ledger_id,
    )
    .await
    {
        Ok(block_index) => {
            log!(
                INFO,
                "[redeem] Succesfully redeemed margin for vault at index {block_index}",
            );
            mutate_state(|s| {
                s.record_redemption(from, redeem_amount, s.one_centigram_of_gold_price)
            });
            Ok(block_index)
        }
        Err(e) => Err(VaultError::TransferFromError(e)),
    }
}
