use crate::state::read_state;
use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_proposals_metrics::Response as GetProposalsMetricsResponse;

#[query]
fn get_proposals_metrics() -> GetProposalsMetricsResponse {
    read_state(|state| state.data.porposals_metrics.clone())
}
