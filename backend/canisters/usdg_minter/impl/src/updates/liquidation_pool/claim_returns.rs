use crate::guard::GuardPrincipal;
use crate::logs::INFO;
use crate::management::transfer;
use crate::state::{mutate_state, read_state};
use crate::updates::reject_anonymous_caller;
use crate::{GLDT, MINIMUM_CLAIMABLE_RETURN};
use candid::Nat;
use ic_canister_log::log;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::LiquidityError;

#[update]
async fn claim_returns(maybe_subaccount: Option<[u8; 32]>) -> Result<u64, LiquidityError> {
    reject_anonymous_caller().map_err(|_| LiquidityError::AnonymousCaller)?;
    let caller = ic_cdk::caller();
    let _guard_principal = GuardPrincipal::new(caller)?;

    let from = Account {
        owner: ic_cdk::caller(),
        subaccount: maybe_subaccount,
    };

    let available_returns = read_state(|s| *s.liquidation_return.get(&from).unwrap_or(&GLDT::ZERO));

    if available_returns < MINIMUM_CLAIMABLE_RETURN {
        return Err(LiquidityError::NotEnoughGLDT {
            minimum_amount: MINIMUM_CLAIMABLE_RETURN.0,
        });
    }

    log!(INFO, "[claim_returns] {from}");

    let gldt_ledger_id = read_state(|s| s.gldt_ledger_id);

    match transfer(from, Nat::from(available_returns.0), None, gldt_ledger_id).await {
        Ok(block_index) => {
            log!(
                INFO,
                "[claim_returns] Succesfully claimed {available_returns} at index {block_index}",
            );
            // TODO RECORD EVENT
            mutate_state(|s| s.record_claimed_returns(from, available_returns));
            Ok(block_index)
        }
        Err(e) => Err(LiquidityError::TransferError(e)),
    }
}
