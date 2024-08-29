use crate::lifecycle::Args;
use ic_cdk::export_candid;
use lifecycle::init::InitArgs;
use lifecycle::post_upgrade::UpgradeArgs;
use queries::list_neurons::ListNeuronsResponse;
use updates::manage_nns_neuron::{ManageNnsNeuronRequest, ManageNnsNeuronResponse};
use updates::manage_recipients::{ManageRewardRecipientsRequest, ManageRewardRecipientsResponse};
use updates::stake_nns_neuron::StakeNnsNeuronResponse;

mod ecdsa;
mod guards;
mod jobs;
mod lifecycle;
mod memory;
mod migrations;
mod queries;
mod state;
mod testing;
mod types;
mod updates;

export_candid!();
