use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::stake_position_response::StakePositionResponse;

pub type Args = ManageStakePositionArgs;
pub type Response = Result<StakePositionResponse, ManageStakePositionError>;
