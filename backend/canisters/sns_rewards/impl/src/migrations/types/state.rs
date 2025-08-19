use crate::state::Data;
use crate::state::SyncInfo;
use crate::{
    model::{maturity_history::MaturityHistory, payment_processor::PaymentProcessor},
    utils::TimeInterval,
};
use candid::CandidType;
use candid::Decode;
use candid::Encode;
use candid::{Nat, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::{ReserveTokenAmounts, TokenRewardTypes};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
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

const MAX_VALUE_SIZE_V0: u32 = 130;

#[derive(Serialize, Clone, Deserialize, CandidType, Debug, PartialEq, Eq, Default)]
pub struct NeuronInfoV0 {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
    pub rewarded_maturity: HashMap<TokenSymbol, u64>,
    pub last_disburse_event_considered: Option<TimestampMillis>,
}

#[derive(Serialize, Clone, Deserialize, CandidType, Debug, PartialEq, Eq, Default)]
pub struct NeuronInfoLegacy {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
    pub rewarded_maturity: HashMap<TokenSymbolLegacy, u64>,
    pub last_disburse_event_considered: Option<TimestampMillis>,
}

#[derive(
    Debug, Serialize, Clone, Deserialize, CandidType, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct TokenSymbolLegacy(String);

impl Storable for NeuronInfoV0 {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        // Try decoding in new format (TokenSymbol enum)
        if let Ok(val) = Decode!(&bytes, NeuronInfo) {
            return NeuronInfoV0 {
                last_synced_maturity: val.last_synced_maturity,
                accumulated_maturity: val.accumulated_maturity,
                rewarded_maturity: val.rewarded_maturity,
                last_disburse_event_considered: val.last_disburse_event_considered,
            };
        }

        // Fallback: decode legacy format with string keys
        if let Ok(legacy) = Decode!(&bytes, NeuronInfoLegacy) {
            let rewarded_maturity = legacy
                .rewarded_maturity
                .into_iter()
                .map(|(k, v)| {
                    let symbol = TokenSymbol::parse(&k.0).unwrap_or_else(|err| {
                        panic!("Unknown token symbol string '{}': {err}", k.0)
                    });
                    (symbol, v)
                })
                .collect();

            return NeuronInfoV0 {
                last_synced_maturity: legacy.last_synced_maturity,
                accumulated_maturity: legacy.accumulated_maturity,
                rewarded_maturity,
                last_disburse_event_considered: legacy.last_disburse_event_considered,
            };
        }

        panic!(
            "Failed to decode NeuronInfoV0 from bytes: {:?}",
            bytes.as_ref()
        );
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE_V0,
        is_fixed_size: false,
    };
}

impl From<NeuronInfoV0> for NeuronInfo {
    fn from(v0: NeuronInfoV0) -> Self {
        NeuronInfo {
            last_synced_maturity: v0.last_synced_maturity,
            accumulated_maturity: v0.accumulated_maturity,
            rewarded_maturity: v0.rewarded_maturity,
            last_disburse_event_considered: v0.last_disburse_event_considered,
        }
    }
}
