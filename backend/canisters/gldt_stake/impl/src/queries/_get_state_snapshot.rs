use crate::state::read_state;
use gldt_stake_api_canister::_get_state_snapshot::StateSnapshot;
pub use gldt_stake_api_canister::_get_state_snapshot::{
    Args as StateSnapshotArgs, Response as StateSnapshotResponse,
};

#[cfg(feature = "inttest")]
use ic_cdk_macros::query;

#[query]
#[cfg(feature = "inttest")]
fn _get_state_snapshot(_args: StateSnapshotArgs) -> StateSnapshotResponse {
    _get_state_snapshot_impl()
}

fn _get_state_snapshot_impl() -> StateSnapshotResponse {
    let caller = ic_cdk::api::msg_caller();
    let (position, total_staked) = read_state(|s| {
        (
            s.data.stake_system.get_stake_position(&caller),
            s.data.stake_system.total_staked.clone(),
        )
    });

    StateSnapshot {
        position,
        total_staked,
    }
}
