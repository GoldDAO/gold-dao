use candid::Principal;
use ic_cdk_macros::query;

use crate::state::{ read_state, GovernanceStats };

#[query]
fn get_neurons_stats(principal: Option<Principal>) -> GovernanceStats {
    if let Some(prin) = principal {
        let stats_by_principal = read_state(|state| state.data.principal_gov_stats.clone());
        return stats_by_principal.get(&prin).unwrap_or(&GovernanceStats::default()).clone();
    } else {
        read_state(|state| state.data.all_gov_stats.clone())
    }
}
