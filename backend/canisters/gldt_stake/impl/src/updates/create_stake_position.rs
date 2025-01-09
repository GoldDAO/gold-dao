use candid::Nat;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::create_stake_position::{
    Args as CreateStakePositionArgs, Response as CreateStakePositionResponse,
};

use gldt_stake_common::{
    ledgers::GLDT_TX_FEE,
    stake_position::{
        AddStakePositionErrors, GLDT_STAKE_MAX_ACTIVE_STAKE_POSITIONS_PER_USER,
        MINIMUM_STAKE_AMOUNT_WITH_FEE,
    },
};
use ic_cdk::{caller, update};
use icrc_ledger_canister_c2c_client::icrc2_transfer_from;
use icrc_ledger_types::{icrc1::account::Account, icrc2::transfer_from::TransferFromArgs};
use tracing::error;
use utils::{env::Environment, retry_async::retry_async};

use crate::{
    guards::{reject_anonymous_caller, GuardPrincipal},
    state::{mutate_state, read_state},
};

#[update]
#[trace]
async fn create_stake_position(args: CreateStakePositionArgs) -> CreateStakePositionResponse {
    create_stake_position_impl(args.amount).await
}

async fn create_stake_position_impl(amount: Nat) -> CreateStakePositionResponse {
    // 1. check user isn't anon
    let caller = caller();
    reject_anonymous_caller().map_err(|e| AddStakePositionErrors::InvalidPrincipal(e))?;
    let _guard_principal =
        GuardPrincipal::new(caller).map_err(|e| AddStakePositionErrors::AlreadyProcessing(e))?;

    // 2 - check minimum stake amount
    if amount < MINIMUM_STAKE_AMOUNT_WITH_FEE {
        return Err(AddStakePositionErrors::InvalidStakeAmount(format!(
            "Can't create a stake position with {} GLDT. The minimum stake amount including a transaction fee is {} GLDT",
            amount, MINIMUM_STAKE_AMOUNT_WITH_FEE
        )));
    }

    // 3. check user has less than 10 active positions
    if read_state(|s: &crate::state::RuntimeState| {
        s.data.stake_system.count_user_stake_positions(&caller)
    }) >= GLDT_STAKE_MAX_ACTIVE_STAKE_POSITIONS_PER_USER
    {
        return Err(AddStakePositionErrors::MaxActiveStakePositions(format!(
            "You may not have more than 10 stake positions at any given moment"
        )));
    }

    // 4. approve call
    let amount_without_fee = amount.clone() - Nat::from(GLDT_TX_FEE);
    transfer_gldt(&amount_without_fee).await?;

    // 5. create stake position and update internals
    let (id, new_stake_position) = mutate_state(|s| {
        s.data
            .stake_system
            .add_stake_position(amount - Nat::from(GLDT_TX_FEE), caller)
    });
    Ok((new_stake_position, timestamp_millis(), id).into())
}

async fn transfer_gldt(amount: &Nat) -> Result<Nat, AddStakePositionErrors> {
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    let caller = caller();
    let transfer_from_args = TransferFromArgs {
        spender_subaccount: None,
        from: Account {
            owner: caller,
            subaccount: None,
        },
        to: Account {
            owner: this_canister_id,
            subaccount: None,
        },
        amount: amount.clone(),
        fee: Some(GLDT_TX_FEE.into()),
        memo: None,
        created_at_time: None,
    };
    match retry_async(
        || icrc2_transfer_from(gldt_ledger_id, transfer_from_args.clone()),
        3,
    )
    .await
    {
        Ok(transfer_response) => match transfer_response {
            icrc_ledger_canister::icrc2_transfer_from::Response::Ok(_) => {
                Ok(amount.clone() - Nat::from(GLDT_TX_FEE))
            }
            icrc_ledger_canister::icrc2_transfer_from::Response::Err(e) => {
                error!(
                    "CLAIM REWARD :: Failed :: Call error -  - {:?}. transfer args - {:?}",
                    e, &transfer_from_args
                );
                Err(AddStakePositionErrors::TransferError(format!("{e:?}")))
            }
        },
        Err(e) => {
            error!(
                "CLAIM REWARD :: Failed :: Call error -  - {:?}. transfer args - {:?}",
                e, &transfer_from_args
            );
            Err(AddStakePositionErrors::CallError(format!("{e:?}")))
        }
    }
}
