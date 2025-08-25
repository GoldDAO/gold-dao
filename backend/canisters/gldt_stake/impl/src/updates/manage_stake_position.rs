use crate::guards::caller_is_whitelisted;
use crate::guards::GuardPrincipal;
use crate::manage_stake_position_impls::*;
use crate::{guards::reject_anonymous_caller, state::read_state};
use candid::Principal;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::manage_stake_position::Response as ManageStakePositionResponse;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
pub use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::stake_position::StakePosition;
use ic_cdk::api::msg_caller;
use ic_cdk::update;

#[update(guard = "caller_is_whitelisted")]
#[trace]
async fn manage_stake_position(args: ManageStakePositionArgs) -> ManageStakePositionResponse {
    // 0. validate the caller and concurrent position processing
    let caller = msg_caller();
    reject_anonymous_caller()
        .map_err(|e| ManageStakePositionError::GeneralError(GeneralError::InvalidPrincipal(e)))?;
    let _guard_principal = GuardPrincipal::new(caller)
        .map_err(|e| ManageStakePositionError::GeneralError(GeneralError::AlreadyProcessing(e)))?;

    // 1. match the operation over position and call the appropriate implementation
    match args {
        ManageStakePositionArgs::AddStake { amount } => add_stake_impl(caller, amount).await,

        other_args => {
            let mut position = load_stake_position(caller)?;

            match other_args {
                ManageStakePositionArgs::ClaimRewards { tokens } => {
                    let results = claim_rewards_batch_impl(caller, tokens).await?;
                    let errors: Vec<_> = results.into_iter().filter_map(Result::err).collect();

                    if errors.is_empty() {
                        let updated_position = load_stake_position(caller)?;
                        Ok((updated_position, timestamp_millis()).into())
                    } else {
                        Err(ManageStakePositionError::ClaimRewardError(errors))
                    }
                }

                ManageStakePositionArgs::StartDissolving { fraction } => {
                    start_dissolving_impl(caller, &mut position, fraction)
                }

                ManageStakePositionArgs::DissolveInstantly { fraction } => {
                    dissolve_instantly_impl(caller, &mut position, fraction).await
                }

                ManageStakePositionArgs::Withdraw {} => withdraw_impl(caller, position).await,

                // NOTE: should not happen because AddStake was handled above
                ManageStakePositionArgs::AddStake { .. } => unreachable!(),
            }
        }
    }
}

fn load_stake_position(caller: Principal) -> Result<StakePosition, ManageStakePositionError> {
    read_state(|s| s.data.stake_system.get_stake_position(&caller)).ok_or_else(|| {
        ManageStakePositionError::GeneralError(GeneralError::StakePositionNotFound(
            "Stake position not found".to_string(),
        ))
    })
}
