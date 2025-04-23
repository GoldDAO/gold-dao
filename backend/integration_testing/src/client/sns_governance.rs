use crate::{generate_pocket_query_call, generate_pocket_update_call};
use sns_governance_canister::*;

// Queries
generate_pocket_query_call!(get_neuron);
generate_pocket_query_call!(get_metadata);
generate_pocket_query_call!(get_nervous_system_parameters);
generate_pocket_query_call!(list_proposals);
generate_pocket_query_call!(list_neurons);
generate_pocket_query_call!(get_proposal);

// Updates

generate_pocket_update_call!(manage_neuron);
