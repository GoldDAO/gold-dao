use crate::{ generate_pocket_query_call, generate_pocket_update_call };

generate_pocket_query_call!(get_all_neurons);
generate_pocket_query_call!(get_neuron_by_id);
generate_pocket_update_call!(sync_neurons_manual_trigger);

// Updates
// generate_update_call!(icrc1_transfer);

pub mod get_all_neurons {
    use super::*;

    pub type Args = ();
    pub type Response = u64;
}

pub mod get_neuron_by_id {
    use sns_governance_canister::types::NeuronId;
    use types::NeuronInfo;

    use super::*;

    pub type Args = NeuronId;
    pub type Response = Option<NeuronInfo>;
}

pub mod sync_neurons_manual_trigger {
    use super::*;
    pub type Args = ();
    pub type Response = ();
}

pub mod get_active_payment_rounds {
    use sns_rewards::model::payment_processor::PaymentRound;

    use super::*;
    pub type Args = ();
    pub type Response = Vec<PaymentRound>;
}
