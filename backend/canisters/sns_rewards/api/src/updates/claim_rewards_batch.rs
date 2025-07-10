use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub claim_reward_args: Vec<ClaimRewardArgs>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ClaimRewardArgs {
    pub neuron_id: NeuronId,
    pub token: TokenSymbol,
}

pub type Response = Result<(), Vec<ClaimRewardError>>;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ClaimRewardError {
    pub neuron_id: NeuronId,
    pub token: Option<TokenSymbol>,
    pub error: ClaimRewardErrorType,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ClaimRewardErrorType {
    NeuronHotKeyInvalid, // Hotkeys exist but they don't match the caller's principal
    NeuronOwnerInvalid(Option<Principal>), // Neuron has a hotkey owned by a different caller
    NeuronNotClaimed,    // Nobody has claimed this neuron yet.
    NeuronDoesNotExist,
    InternalError(String),
    TransferFailed(String),
    TokenSymbolInvalid(TokenSymbol),
}
