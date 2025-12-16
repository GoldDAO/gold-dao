use crate::guards::GuardPrincipal;
use crate::manage_stake_position_impls::add_stake_impl;
use crate::{guards::reject_anonymous_caller, state::read_state};
use candid::Principal;
pub use gldt_stake_api_canister::manage_stake_position::Response as AddStakeMockResponse;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
pub use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::stake_position::StakePosition;
use ic_cdk::api::msg_caller;
use ic_cdk::update;

#[cfg(feature = "inttest")]
#[update]
async fn _add_stake_mock(args: ManageStakePositionArgs) -> AddStakeMockResponse {
    let caller = msg_caller();
    reject_anonymous_caller()
        .map_err(|e| ManageStakePositionError::GeneralError(GeneralError::InvalidPrincipal(e)))?;
    let _guard_principal = GuardPrincipal::new(caller)
        .map_err(|e| ManageStakePositionError::GeneralError(GeneralError::AlreadyProcessing(e)))?;

    match args {
        ManageStakePositionArgs::AddStake { amount } => add_stake_impl(caller, amount).await,

        other_args => match other_args {
            _ => Err(ManageStakePositionError::GeneralError(
                GeneralError::CallError(
                    "GLDT staking operations other than AddStake are disabled.".to_string(),
                ),
            )),
        },
    }
}

fn load_stake_position(caller: Principal) -> Result<StakePosition, ManageStakePositionError> {
    read_state(|s| s.data.stake_system.get_stake_position(&caller)).ok_or_else(|| {
        ManageStakePositionError::GeneralError(GeneralError::StakePositionNotFound(
            "Stake position not found".to_string(),
        ))
    })
}
