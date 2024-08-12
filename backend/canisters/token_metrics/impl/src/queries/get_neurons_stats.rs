use ic_cdk_macros::query;
pub use token_metrics_api::{
    token_data::GovernanceStats,
    queries::get_neurons_stats::{
        Args as GetNeuronsStatsArgs,
        Response as GetNeuronsStatsResponse,
    },
};

use crate::state::read_state;

#[query]
fn get_neurons_stats(principal: GetNeuronsStatsArgs) -> GetNeuronsStatsResponse {
    if let Some(prin) = principal {
        let stats_by_principal = read_state(|state| state.data.principal_gov_stats.clone());
        return stats_by_principal.get(&prin).unwrap_or(&GovernanceStats::default()).clone();
    } else {
        read_state(|state| state.data.all_gov_stats.clone())
    }
}
