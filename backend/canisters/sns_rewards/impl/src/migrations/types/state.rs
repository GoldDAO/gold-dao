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
    /// SNS governance canister
    pub sns_governance_canister: Principal,
    /// Stores the maturity information about each neuron
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// Information about periodic synchronization
    pub sync_info: SyncInfo,
    /// The history of each neuron's maturity.
    pub maturity_history: MaturityHistory,
    /// Payment processor - responsible for queuing and processing rounds of payments
    pub payment_processor: PaymentProcessor,
    /// valid tokens and their associated ledger data
    pub tokens: TokenRewardTypes,
    /// authorized Principals for guarded calls
    pub authorized_principals: Vec<Principal>,
    /// a boolean check for if we're currently synchronizing neuron data into the canister.
    pub is_synchronizing_neurons: bool,
    /// The daily amount of tokens to transfer from the reserve pool sub account to the reward pool sub account in e8s for each token type
    pub daily_reserve_transfer: ReserveTokenAmounts,
    /// Last time the daily reserve transfer completed - used to make sure we don't transfer multiple times per day after upgrades
    pub last_daily_reserve_transfer_time: TimestampMillis,
    /// The daily burn rate of GOLDAO - settable via a proposal
    pub daily_goldao_burn_rate: Option<Nat>,
    /// The last time a burn of GOLDAO was done
    pub last_daily_goldao_burn: Option<TimestampMillis>,
    /// The weekly interval for which a reward distribution occurs
    pub reward_distribution_interval: Option<TimeInterval>,
    /// An internal check if the distribution is running
    pub reward_distribution_in_progress: Option<bool>,
    /// The daily interval for which a neuron sync occurs
    pub neuron_sync_interval: Option<TimeInterval>,
}

impl From<DataV0> for Data {
    fn from(v0: DataV0) -> Self {
        Data {
            sns_governance_canister: v0.sns_governance_canister,
            neuron_maturity: v0
                .neuron_maturity
                .into_iter()
                .map(|(k, v)| (k, NeuronInfo::from(v)))
                .collect(),
            sync_info: v0.sync_info,
            maturity_history: v0.maturity_history,
            payment_processor: PaymentProcessor::from(v0.payment_processor),
            tokens: v0.tokens,
            authorized_principals: v0.authorized_principals,
            is_synchronizing_neurons: v0.is_synchronizing_neurons,
            reward_distribution_interval: v0.reward_distribution_interval,
            reward_distribution_in_progress: v0.reward_distribution_in_progress,
            neuron_sync_interval: v0.neuron_sync_interval,
        }
    }
}
