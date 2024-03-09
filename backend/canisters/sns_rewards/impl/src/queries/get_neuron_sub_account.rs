use ic_cdk_macros::query;
use ic_ledger_types::Subaccount;
use sns_governance_canister::types::NeuronId;
use crate::state::read_state;

#[query]
fn get_neuron_sub_account(id: NeuronId) -> Option<Subaccount> {
    read_state(|state| {
        
        let sub_account = state.data.user_rewards.get_account_id_by_neuron_id(id);

        match sub_account {
            Some(sa) => Some(Subaccount(sa.0)),
            None => None
        }
    })
}