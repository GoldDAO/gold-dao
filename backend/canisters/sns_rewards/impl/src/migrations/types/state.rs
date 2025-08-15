use crate::state::Data;
use crate::state::SyncInfo;
use crate::{
    model::{maturity_history::MaturityHistory, payment_processor::PaymentProcessor},
    utils::TimeInterval,
};
use candid::{Nat, Principal};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::{ReserveTokenAmounts, TokenRewardTypes};
use std::collections::BTreeMap;
use types::{NeuronInfo, TimestampMillis};
use utils::env::CanisterEnv;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub sns_governance_canister: Principal,
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    pub sync_info: SyncInfo,
    pub maturity_history: MaturityHistory,
    pub payment_processor: PaymentProcessor,
    pub tokens: TokenRewardTypes,
    pub authorized_principals: Vec<Principal>,
    pub is_synchronizing_neurons: bool,
    pub daily_reserve_transfer: ReserveTokenAmounts,
    pub last_daily_reserve_transfer_time: TimestampMillis,
    pub daily_goldao_burn_rate: Option<Nat>,
    pub last_daily_goldao_burn: Option<TimestampMillis>,
    pub reward_distribution_interval: Option<TimeInterval>,
    pub reward_distribution_in_progress: Option<bool>,
    pub neuron_sync_interval: Option<TimeInterval>,
}

impl From<DataV0> for Data {
    fn from(v0: DataV0) -> Self {
        Data {
            sns_governance_canister: v0.sns_governance_canister,
            neuron_maturity: v0.neuron_maturity,
            sync_info: v0.sync_info,
            maturity_history: MaturityHistory::default(),
            payment_processor: v0.payment_processor,
            tokens: v0.tokens,
            authorized_principals: v0.authorized_principals,
            is_synchronizing_neurons: v0.is_synchronizing_neurons,
            daily_reserve_transfer: v0.daily_reserve_transfer,
            last_daily_reserve_transfer_time: v0.last_daily_reserve_transfer_time,
            daily_goldao_burn_rate: v0.daily_goldao_burn_rate,
            last_daily_goldao_burn: v0.last_daily_goldao_burn,
            reward_distribution_interval: v0.reward_distribution_interval,
            reward_distribution_in_progress: v0.reward_distribution_in_progress,
            neuron_sync_interval: v0.neuron_sync_interval,
            migration_finished: None,
        }
    }
}
