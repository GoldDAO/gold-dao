use crate::state::BurnConfig;
use crate::state::Data;
use utils::numeric::Percentage;

use crate::state::RuntimeState;

use self::types::state::RuntimeStateV0;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                authorized_principals: old_state.data.authorized_principals,
                gldgov_token_info: old_state.data.gldgov_token_info,
                icp_swap_canister_id: old_state.data.icp_swap_canister_id,
                buyback_interval: old_state.data.buyback_burn_interval,
                swap_clients: old_state.data.swap_clients,
                burn_config: BurnConfig {
                    burn_percentage: Percentage::new(old_state.data.burn_config.burn_rate).unwrap(),
                    min_burn_amount: old_state.data.burn_config.min_burn_amount,
                },
                token_swaps: old_state.data.token_swaps,
            },
        }
    }
}
