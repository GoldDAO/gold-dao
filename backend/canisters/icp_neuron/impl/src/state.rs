use canister_time::{ MINUTE_IN_MS, NANOS_PER_MILLISECOND };

use ic_stable_structures::Storable;
use ic_transport_types::EnvelopeContent;
use icrc_ledger_types::icrc1::account::Account;
use k256::{ pkcs8::EncodePublicKey, PublicKey };
use nns_governance_canister::types::Neuron;
use serde::{ Deserialize, Serialize };
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::{ CanisterId, TimestampMillis };
use utils::{
    consts::{ ICP_LEDGER_CANISTER_ID, NNS_GOVERNANCE_CANISTER_ID, SNS_GOVERNANCE_CANISTER_ID },
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};

use crate::ecdsa::{ get_key_id, CanisterEcdsaRequest };

const IC_URL: &str = "https://icp-api.io";

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        Self { env, data }
    }
    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                now: self.env.now(),
                test_mode: self.env.test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
            },
            public_key: hex::encode(&self.data.public_key),
            public_key_der: hex::encode(&self.data.get_public_key_der()),
            own_principal: self.data.get_principal(),
            authorized_principals: self.data.authorized_principals.clone(),
            // neurons: self.neurons,
            nns_governance_canister_id: self.data.nns_governance_canister_id,
            icp_ledger_canister_id: self.data.icp_ledger_canister_id,
            rewards_recipients: self.data.rewards_recipients.clone(),
        }
    }

    pub fn is_caller_governance_principal(&self) -> bool {
        let caller = self.env.caller();
        self.data.authorized_principals.contains(&caller)
    }

    pub fn prepare_canister_call_via_ecdsa<A: CandidType>(
        &mut self,
        canister_id: CanisterId,
        method_name: String,
        args: A,
        nonce: Option<Vec<u8>>
    ) -> CanisterEcdsaRequest {
        let envelope_content = EnvelopeContent::Call {
            nonce,
            ingress_expiry: self.env.now_nanos() + 5 * MINUTE_IN_MS * NANOS_PER_MILLISECOND,
            sender: self.data.get_principal(),
            canister_id,
            method_name,
            arg: candid::encode_one(&args).unwrap(),
        };

        CanisterEcdsaRequest {
            envelope_content,
            request_url: format!("{IC_URL}/api/v2/canister/{canister_id}/call"),
            public_key: self.data.get_public_key_der(),
            key_id: get_key_id(false),
            this_canister_id: self.env.canister_id(),
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub public_key: String,
    pub public_key_der: String,
    pub own_principal: Principal,
    pub authorized_principals: Vec<Principal>,
    pub nns_governance_canister_id: Principal,
    pub icp_ledger_canister_id: Principal,
    pub rewards_recipients: Vec<RewardsRecipients>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub public_key: Vec<u8>,
    pub authorized_principals: Vec<Principal>,
    pub neurons: Neurons,
    pub nns_governance_canister_id: Principal,
    pub icp_ledger_canister_id: Principal,
    pub rewards_recipients: Vec<RewardsRecipients>,
}

impl Data {
    pub fn new(rewards_recipients: Vec<RewardsRecipients>) -> Self {
        Self {
            rewards_recipients,
            public_key: Vec::new(),
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            neurons: Neurons::default(),
            nns_governance_canister_id: NNS_GOVERNANCE_CANISTER_ID,
            icp_ledger_canister_id: ICP_LEDGER_CANISTER_ID,
        }
    }
}

impl Data {
    pub fn get_public_key_der(&self) -> Vec<u8> {
        PublicKey::from_sec1_bytes(&self.public_key).unwrap().to_public_key_der().unwrap().to_vec()
    }

    pub fn get_principal(&self) -> Principal {
        Principal::self_authenticating(&self.get_public_key_der())
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Neurons {
    pub timestamp: TimestampMillis,
    pub active_neurons: Vec<Neuron>,
    pub spawning_neurons: Vec<Neuron>,
    pub disbursed_neurons: Vec<u64>,
}

// pub enum NeuronState {
//     Active,
//     Spawning,
//     Disbursed,
// }

// impl Neurons {
//     pub fn list(&self, neuron_state: NeuronState) -> Vec<Neuron> {
//         match neuron_state {
//             NeuronState::Active => self.active_neurons.clone(),
//             NeuronState::Spawning => self.spawning_neurons.clone(),
//             NeuronState::Disbursed =>
//                 self.disbursed_neurons
//                     .iter()
//                     .map(|id| Neuron { id: *id, ..Default::default() })
//                     .collect(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct RewardsRecipients {
    /// The account to which the rewards will be disbursed
    pub account: Account,
    /// A tag to identify the recipient
    pub tag: String,
    /// The weight of the rewards to be disbursed to this recipient. The weight is a number between 1 and 10000.
    /// For consistency, the sum of all weights should be 10000.
    pub reward_weight: u64,
}
