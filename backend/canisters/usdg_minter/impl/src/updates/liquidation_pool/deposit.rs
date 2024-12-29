use crate::guard::GuardPrincipal;
use crate::logs::INFO;
use crate::management::transfer_from;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::USDG;
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::updates::deposit_liquidity::DepositArg;
use usdg_minter_api::LiquidityError;

#[update]
async fn deposit_liquidity(arg: DepositArg) -> Result<u64, LiquidityError> {
    reject_anonymous_caller().map_err(|_| LiquidityError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let deposit_amount = USDG::from_e8s(arg.deposited_amount);
    let usdg_ledger_id = read_state(|s| s.usdg_ledger_id);

    log!(INFO, "[deposit_liquidity] {arg}");

    let from = Account {
        owner: ic_cdk::caller(),
        subaccount: arg.maybe_subaccount,
    };

    match transfer_from(
        from,
        ic_cdk::id(),
        Nat::from(arg.deposited_amount),
        None,
        usdg_ledger_id,
    )
    .await
    {
        Ok(block_index) => {
            log!(INFO, "[deposit_liquidity] Succesfully added liquidity to pool {deposit_amount} at index {block_index}",);
            /// TODO RECORD EVENT
            mutate_state(|s| s.deposit_liquidity(from, deposit_amount));
            Ok(block_index)
        }
        Err(e) => Err(LiquidityError::TransferFromError(e)),
    }
}
