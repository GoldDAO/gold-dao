use crate::guard::GuardPrincipal;
use crate::lifecycle::timer::check_postcondition;
use crate::logs::INFO;
use crate::management::transfer;
use crate::memory::record_event;
use crate::state::event::EventType;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::USDG;
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::updates::withdraw_liquidity::WithdrawArg;
use usdg_minter_api::LiquidityError;

#[update]
async fn withdraw_liquidity(arg: WithdrawArg) -> Result<u64, LiquidityError> {
    check_postcondition(_withdraw_liquidity(arg)).await
}

async fn _withdraw_liquidity(arg: WithdrawArg) -> Result<u64, LiquidityError> {
    reject_anonymous_caller().map_err(|_| LiquidityError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let withdraw_amount = USDG::from_e8s(arg.amount);
    let usdg_ledger_id = read_state(|s| s.usdg_ledger_id);

    log!(INFO, "[withdraw_liquidity] {arg}");

    let to = Account {
        owner: ic_cdk::caller(),
        subaccount: arg.maybe_subaccount,
    };

    let balance = read_state(|s| *s.liquidation_pool.get(&to).unwrap_or(&USDG::ZERO));

    if withdraw_amount > balance {
        return Err(LiquidityError::BalanceTooLow { balance: balance.0 });
    }

    mutate_state(|s| s.withdraw_liquidity(withdraw_amount, to));

    match transfer(to, Nat::from(arg.amount), None, usdg_ledger_id).await {
        Ok(block_index) => {
            log!(INFO, "[withdraw_liquidity] Succesfully withdrew liquidity to pool {withdraw_amount} at index {block_index}",);
            record_event(
                EventType::WithdrawLiquidity {
                    caller: to,
                    amount: withdraw_amount,
                    block_index,
                },
                ic_cdk::api::time(),
            );
            Ok(block_index)
        }
        Err(e) => {
            mutate_state(|s| s.deposit_liquidity(to, withdraw_amount));
            log!(INFO, "[withdraw_liquidity] {caller} failed to withdraw adding USDG back to balance, error: {e}");
            Err(LiquidityError::TransferError(e))
        }
    }
}
