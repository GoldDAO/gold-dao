use crate::{
    model::proposal_system::ProposalSystem,
    state::{Data, RuntimeState},
};

use self::types::state::RuntimeStateV0;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                gldt_ledger_id: old_state.data.gldt_ledger_id,
                ogy_ledger_id: old_state.data.ogy_ledger_id,
                goldao_ledger_id: old_state.data.goldao_ledger_id,
                icp_ledger_id: old_state.data.icp_ledger_id,
                authorized_principals: old_state.data.authorized_principals,
                stake_system: old_state.data.stake_system,
                goldao_sns_rewards_canister_id: old_state.data.goldao_sns_rewards_canister_id,
                goldao_sns_governance_canister_id: old_state.data.goldao_sns_governance_canister_id,
                neuron_system: old_state.data.neuron_system,
                reward_system: old_state.data.reward_system,
                reward_claim_interval: old_state.data.reward_claim_interval,
                is_reward_claim_in_progress: old_state.data.is_reward_claim_in_progress,
                is_reward_allocation_in_progress: old_state.data.is_reward_allocation_in_progress,
                is_archive_cron_running: old_state.data.is_archive_cron_running,
                principal_guards: old_state.data.principal_guards,
                archive_system: old_state.data.archive_system,
                proposal_system: ProposalSystem::default(),
            },
        }
    }
}
