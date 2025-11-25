use self::types::state::RuntimeStateV0;
use crate::state::{Data, RuntimeState};
pub mod jobs;
pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                authorized_principals: old_state.data.authorized_principals,
                buyback_burn_canister: old_state.data.buyback_burn_canister,
                nft_guards: old_state.data.nft_guards,
                swap_system: old_state.data.swap_system,
                swap_configs: old_state.data.swap_configs,
                is_remove_stale_swaps_cron_running: old_state
                    .data
                    .is_remove_stale_swaps_cron_running,
                is_archive_cron_running: old_state.data.is_archive_cron_running,
                is_gldt_supply_balancer_running: old_state.data.is_gldt_supply_balancer_running,
                migration_finished: old_state.data.migration_finished,
                old_archive_canister: old_state.data.old_archive_canister,
            },
        }
    }
}
