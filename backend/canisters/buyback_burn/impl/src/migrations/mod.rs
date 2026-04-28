use crate::state::Data;

use crate::state::RuntimeState;

use self::types::state::RuntimeStateV0;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                authorized_principals: old_state.data.authorized_principals,
                icp_swap_canister_id: old_state.data.icp_swap_canister_id,
                exchange_jobs: old_state.data.exchange_jobs,
                exchange_job_guards: Default::default(),
                token_swaps: old_state.data.token_swaps.into(),
            },
        }
    }
}
