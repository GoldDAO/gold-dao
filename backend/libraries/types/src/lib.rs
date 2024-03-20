use std::borrow::Cow;

use candid::{ CandidType, Decode, Encode, Principal };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };

mod http;
mod neuron_info;
mod proposals;
mod rewards_recipients;

pub use http::*;
pub use neuron_info::*;
pub use proposals::*;
pub use rewards_recipients::*;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}

pub type CanisterId = Principal;
pub type Cycles = u64;
pub type Hash = [u8; 32];
pub type Maturity = u64;
pub type Milliseconds = u64;
pub type NnsNeuronId = u64;
pub type ProposalId = u64;
pub type SnsNeuronId = [u8; 32];
pub type TimestampSeconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;

#[derive(Debug, Serialize, Clone, Deserialize, CandidType, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    OGY,
    ICP,
    GLDGov,
}

const MAX_VALUE_SIZE: u32 = 8;
impl Storable for Token {
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
