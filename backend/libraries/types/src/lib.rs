use candid::{ CandidType, Principal };
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



#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub enum TokenType {
    ICP,
    OGY,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct TransferTokenArgs {
    pub token_type: TokenType,
    pub amount: u64,
    pub from_canister: Principal,
}