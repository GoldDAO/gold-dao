use self::types::state::RuntimeStateV0;
use crate::model::analytics_system::AnalyticsSystem;
use crate::model::stake_system::StakeSystem;
use crate::{
    state::{Data, RuntimeState},
    utils::TimeInterval,
};
use std::collections::HashMap;
pub mod types;
use crate::model::allocated_rewards_pool::AllocatedRewardsPool;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                gldt_ledger_id: old_state.data.gldt_ledger_id,
                goldao_ledger_id: old_state.data.goldao_ledger_id,
                authorized_principals: old_state.data.authorized_principals,
                whitelist: old_state.data.whitelist,
                analytics_system: AnalyticsSystem::default(),
                stake_system: StakeSystem {
                    stakes: old_state.data.stake_system.stakes,
                    total_staked: old_state.data.stake_system.total_staked,
                    cached_total_weighted_stake: old_state
                        .data
                        .stake_system
                        .cached_total_weighted_stake,
                    stake_positions_quantity_limit: old_state
                        .data
                        .stake_system
                        .stake_positions_quantity_limit,
                    reward_types: old_state.data.stake_system.reward_types,
                    pending_fee_transfer_amount: old_state
                        .data
                        .stake_system
                        .pending_fee_transfer_amount,
                    genesis_datetime: old_state.data.stake_system.genesis_datetime,
                    token_usd_values: old_state.data.stake_system.token_usd_values,
                    cached_daily_timestamp: old_state.data.stake_system.daily_apy_timestamp,
                    // daily_apy_history: old_state.data.stake_system.daily_apy_history,
                },
                goldao_sns_rewards_canister_id: old_state.data.goldao_sns_rewards_canister_id,
                goldao_sns_governance_canister_id: old_state.data.goldao_sns_governance_canister_id,
                neuron_system: old_state.data.neuron_system,
                unallocated_rewards_pool: old_state.data.unallocated_rewards_pool,
                allocate_rewards_interval: Some(TimeInterval {
                    weekday: Some("Thursday".to_string()),
                    start_hour: 11,
                    end_hour: 12,
                }),
                processing_rewards_pool: old_state.data.processing_rewards_pool,
                allocated_rewards_pool: AllocatedRewardsPool {
                    state: old_state.data.allocated_rewards_pool.state,
                    cached_rewards: HashMap::new(),
                    last_allocation_time: old_state
                        .data
                        .allocated_rewards_pool
                        .last_allocation_time,
                    reward_history: old_state.data.allocated_rewards_pool.reward_history,
                },
                reward_claim_interval: Some(TimeInterval {
                    weekday: Some("Thursday".to_string()),
                    start_hour: 10,
                    end_hour: 11,
                }),
                principal_guards: old_state.data.principal_guards,
                proposal_system: old_state.data.proposal_system,
            },
        }
    }
}
