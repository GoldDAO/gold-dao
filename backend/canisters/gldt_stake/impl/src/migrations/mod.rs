use self::types::state::RuntimeStateV0;
use crate::{
    model::proposal_system::ProposalSystem,
    state::{Data, RuntimeState},
};
pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                gldt_ledger_id: old_state.data.gldt_ledger_id,
                goldao_ledger_id: old_state.data.goldao_ledger_id,
                authorized_principals: old_state.data.authorized_principals,
                whitelist: vec![],
                stake_system: old_state.data.stake_system,
                goldao_sns_rewards_canister_id: old_state.data.goldao_sns_rewards_canister_id,
                goldao_sns_governance_canister_id: old_state.data.goldao_sns_governance_canister_id,
                neuron_system: old_state.data.neuron_system,
                // reward_system: old_state.data.reward_system,
                unallocated_rewards_pool: old_state.data.unallocated_rewards_pool,
                processing_rewards_pool: old_state.data.processing_rewards_pool,
                allocated_rewards_pool: old_state.data.allocated_rewards_pool,
                reward_claim_interval: old_state.data.reward_claim_interval,
                principal_guards: old_state.data.principal_guards,
                proposal_system: ProposalSystem::default(),
            },
        }
    }
}
