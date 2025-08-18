use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::{token::TokenSymbol, TimestampMillis};

const MAX_VALUE_SIZE: u32 = 160;

/// The maturity information about a neuron
#[derive(Serialize, Clone, Deserialize, CandidType, Debug, PartialEq, Eq, Default)]
pub struct NeuronInfo {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
    pub rewarded_maturity: HashMap<TokenSymbol, u64>,
    pub last_disburse_event_considered: Option<TimestampMillis>,
}

impl Storable for NeuronInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

const MAX_VALUE_SIZE_V0: u32 = 130;

#[derive(Serialize, Clone, Deserialize, CandidType, Debug, PartialEq, Eq, Default)]
pub struct NeuronInfoV0 {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
    pub rewarded_maturity: HashMap<TokenSymbol, u64>,
    pub last_disburse_event_considered: Option<TimestampMillis>,
}

impl Storable for NeuronInfoV0 {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
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
