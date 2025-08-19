use crate::memory::get_payment_round_history_memory;
use crate::memory::VM;
use crate::state::Data;
use crate::state::SyncInfo;
use crate::{
    model::{maturity_history::MaturityHistory, payment_processor::PaymentProcessor},
    utils::TimeInterval,
};
use candid::{Nat, Principal};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::payment_round::PaymentRound;
use sns_rewards_api_canister::{ReserveTokenAmounts, TokenRewardTypes};
use std::collections::BTreeMap;
use std::collections::HashMap;
use types::TokenInfo;
use types::TokenSymbol;
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
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfoV0>,
    pub sync_info: SyncInfo,
    pub maturity_history: MaturityHistory,
    pub payment_processor: PaymentProcessor,
    pub tokens: TokenRewardTypesV0,
    pub authorized_principals: Vec<Principal>,
    pub is_synchronizing_neurons: bool,
    pub daily_reserve_transfer: ReserveTokenAmountsV0,
    pub last_daily_reserve_transfer_time: TimestampMillis,
    pub daily_gldgov_burn_rate: Option<Nat>,
    pub last_daily_gldgov_burn: Option<TimestampMillis>,
    pub reward_distribution_interval: Option<TimeInterval>,
    pub reward_distribution_in_progress: Option<bool>,
    pub neuron_sync_interval: Option<TimeInterval>,
}

impl From<DataV0> for Data {
    fn from(v0: DataV0) -> Self {
        let mut tokens: TokenRewardTypes = TokenRewardTypes::new();

        for (symbol_v0, info) in v0.tokens {
            match TokenSymbol::parse(&symbol_v0.0) {
                Ok(symbol) => {
                    tokens.insert(symbol, info);
                }
                Err(e) => {
                    panic!("Failed to parse token symbol '{}': {:?}", symbol_v0.0, e);
                }
            }
        }

        // NOTE: add support of WTN
        tokens.insert(TokenSymbol::WTN, TokenSymbol::WTN.get_token_info());

        // Convert daily_reserve_transfer keys from TokenIdentifierV0 -> TokenSymbol
        let mut daily_reserve_transfer: ReserveTokenAmounts = HashMap::new();

        for (symbol_v0, amount) in v0.daily_reserve_transfer {
            match TokenSymbol::parse(&symbol_v0.0) {
                Ok(symbol) => {
                    daily_reserve_transfer.insert(symbol, amount);
                }
                Err(e) => {
                    panic!(
                        "Failed to parse token symbol in daily_reserve_transfer '{}': {:?}",
                        symbol_v0.0, e
                    );
                }
            }
        }

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
            tokens,
            authorized_principals: v0.authorized_principals,
            is_synchronizing_neurons: v0.is_synchronizing_neurons,
            daily_reserve_transfer,
            last_daily_reserve_transfer_time: v0.last_daily_reserve_transfer_time,
            daily_goldao_burn_rate: v0.daily_gldgov_burn_rate,
            last_daily_goldao_burn: v0.last_daily_gldgov_burn,
            reward_distribution_interval: v0.reward_distribution_interval,
            reward_distribution_in_progress: v0.reward_distribution_in_progress,
            neuron_sync_interval: v0.neuron_sync_interval,
        }
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct NeuronInfoV0 {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
    pub rewarded_maturity: HashMap<TokenSymbolV0, u64>,
    pub last_disburse_event_considered: Option<TimestampMillis>,
}

impl From<NeuronInfoV0> for NeuronInfo {
    fn from(v0: NeuronInfoV0) -> Self {
        let mut rewarded_maturity: HashMap<TokenSymbol, u64> = HashMap::new();

        for (symbol_v0, amount) in v0.rewarded_maturity {
            match TokenSymbol::parse(&symbol_v0.0) {
                Ok(symbol) => {
                    rewarded_maturity.insert(symbol, amount);
                }
                Err(e) => {
                    panic!(
                        "Failed to parse token symbol in rewarded_maturity '{}': {:?}",
                        symbol_v0.0, e
                    );
                }
            }
        }

        NeuronInfo {
            last_synced_maturity: v0.last_synced_maturity,
            accumulated_maturity: v0.accumulated_maturity,
            rewarded_maturity,
            last_disburse_event_considered: v0.last_disburse_event_considered,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TokenSymbolV0(pub String);

pub type ReserveTokenAmountsV0 = HashMap<TokenSymbolV0, Nat>;
pub type TokenRewardTypesV0 = HashMap<TokenSymbolV0, TokenInfo>;

#[derive(Serialize, Deserialize)]
pub struct PaymentProcessorV0 {
    #[serde(skip, default = "init_map")]
    round_history: StableBTreeMap<(TokenSymbol, u16), PaymentRound, VM>,
    active_rounds: BTreeMap<TokenSymbolV0, PaymentRound>,
}

fn init_map() -> StableBTreeMap<(TokenSymbol, u16), PaymentRound, VM> {
    let memory = get_payment_round_history_memory();
    StableBTreeMap::init(memory)
}

impl From<PaymentProcessorV0> for PaymentProcessor {
    fn from(v0: PaymentProcessorV0) -> Self {
        let mut active_rounds: BTreeMap<TokenSymbol, PaymentRound> = BTreeMap::new();

        for (symbol_v0, round) in v0.active_rounds {
            match TokenSymbol::parse(&symbol_v0.0) {
                Ok(symbol) => {
                    active_rounds.insert(symbol, round);
                }
                Err(e) => {
                    panic!(
                        "Failed to parse token symbol in active_rounds '{}': {:?}",
                        symbol_v0.0, e
                    );
                }
            }
        }

        PaymentProcessor {
            round_history: v0.round_history,
            active_rounds,
        }
    }
}
