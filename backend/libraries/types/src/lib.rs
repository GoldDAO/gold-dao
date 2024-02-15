use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };

mod http;
mod neuron_info;
mod proposals;

pub use http::*;
pub use neuron_info::*;
pub use proposals::*;

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
