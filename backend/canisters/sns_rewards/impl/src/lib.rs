use std::borrow::Cow;

use ic_cdk::export_candid;
use candid::{ CandidType, Decode, Encode, Principal };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use queries::{ GetNeuronResponse, get_maturity_history_of_neuron::MaturityHistoryResponse };
use lifecycle::Args;
use types::{ HttpRequest, HttpResponse, NeuronInfo, TimestampMillis };

mod jobs;
mod lifecycle;
mod model;
mod queries;
mod state;
mod memory;
mod updates;

export_candid!();
