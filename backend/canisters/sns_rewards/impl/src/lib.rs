use std::collections::HashMap;

use ic_cdk::export_candid;
use candid::Principal;
use model::payment_processor::PaymentRound;
use sns_governance_canister::types::NeuronId;
use queries::{ GetNeuronResponse, get_maturity_history_of_neuron::MaturityHistoryResponse };
use lifecycle::Args;
use types::{ HttpRequest, HttpResponse, NeuronInfo, TimestampMillis, TokenInfo, TokenSymbol };
use updates::set_reward_token_types::{ SetRewardTokenTypesRequest, SetRewardTokenTypesResponse };
use updates::claim_rewards::UserClaimErrorResponse;

mod utils;
mod guards;
mod jobs;
mod lifecycle;
mod model;
mod queries;
mod state;
mod memory;
mod updates;

export_candid!();
