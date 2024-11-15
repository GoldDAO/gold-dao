use ic_cdk_macros::query;
pub use token_metrics_api::{
    queries::get_neurons_stats::{
        Args as GetNeuronsStatsArgs, Response as GetNeuronsStatsResponse,
    },
    token_data::GovernanceStats,
};

use crate::state::read_state;

#[query]
fn get_neurons_stats(principal: GetNeuronsStatsArgs) -> GetNeuronsStatsResponse {
    if let Some(prin) = principal {
        read_state(|state| {
            let stats = &state.data.principal_gov_stats;

            stats
                .get(&prin)
                .unwrap_or(GovernanceStats::default())
                .clone()
        })
    } else {
        read_state(|state| state.data.all_gov_stats.clone())
    }
}
