use canister_time::{ MINUTE_IN_MS, NANOS_PER_MILLISECOND };

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
            public_key_der: hex::encode(&self.data.get_public_key_der().unwrap_or_default()),
            own_principal: self.data.get_principal(),
            authorized_principals: self.data.authorized_principals.clone(),
            neurons: self.data.get_neuron_list(),
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
        &self,
        canister_id: CanisterId,
        method_name: String,
        args: A,
        nonce: Option<Vec<u8>>
    ) -> Result<CanisterEcdsaRequest, String> {
        let envelope_content = EnvelopeContent::Call {
            nonce,
            ingress_expiry: self.env.now_nanos() + 5 * MINUTE_IN_MS * NANOS_PER_MILLISECOND,
            sender: self.data.get_principal(),
            canister_id,
            method_name,
            arg: candid::encode_one(&args).unwrap(),
        };

        let public_key = self.data.get_public_key_der()?;

        Ok(CanisterEcdsaRequest {
            envelope_content,
            request_url: format!("{IC_URL}/api/v2/canister/{canister_id}/call"),
            public_key,
            key_id: get_key_id(false),
            this_canister_id: self.env.canister_id(),
        })
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
    pub rewards_recipients: RewardsRecipientList,
    pub neurons: NeuronList,
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
    pub rewards_recipients: RewardsRecipientList,
}

impl Data {
    pub fn new(rewards_recipients: RewardsRecipientList) -> Self {
        Self {
            rewards_recipients,
            public_key: Vec::new(),
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            neurons: Neurons::default(),
            nns_governance_canister_id: NNS_GOVERNANCE_CANISTER_ID,
            icp_ledger_canister_id: ICP_LEDGER_CANISTER_ID,
        }
    }

    pub fn get_neuron_list(&self) -> NeuronList {
        NeuronList {
            active: self.neurons.active_neurons
                .iter()
                .filter_map(|n| n.id.as_ref().map(|id| id.id))
                .collect(),
            spawning: self.neurons.spawning_neurons
                .iter()
                .filter_map(|n| n.id.as_ref().map(|id| id.id))
                .collect(),
            disbursed: self.neurons.disbursed_neurons.clone(),
        }
    }
}

impl Data {
    pub fn get_public_key_der(&self) -> Result<Vec<u8>, String> {
        match PublicKey::from_sec1_bytes(&self.public_key) {
            Ok(val) =>
                match val.to_public_key_der() {
                    Ok(pk) => Ok(pk.to_vec()),
                    Err(_) => Err("Error converting public key.".to_string()),
                }
            Err(_) => Err("Error converting public key.".to_string()),
        }
    }

    pub fn get_principal(&self) -> Principal {
        Principal::self_authenticating(&self.get_public_key_der().unwrap_or_default())
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Neurons {
    pub timestamp: TimestampMillis,
    pub active_neurons: Vec<Neuron>,
    pub spawning_neurons: Vec<Neuron>,
    pub disbursed_neurons: Vec<u64>,
}

#[derive(CandidType, Serialize)]
pub struct NeuronList {
    active: Vec<u64>,
    spawning: Vec<u64>,
    disbursed: Vec<u64>,
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct RewardsRecipient {
    /// The account to which the rewards will be disbursed
    pub account: Account,
    /// A tag to identify the recipient
    pub tag: String,
    /// The weight of the rewards to be disbursed to this recipient. The weight is a number between 1 and 10000.
    /// For consistency, the sum of all weights should add up to 10000. If you are defining % values, define them as
    /// multiples of 100. E.g. 33% would be 3300, 1.5% would be 150 and 75.23% would be 7523.
    pub reward_weight: u64,
}

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct RewardsRecipientList(Vec<RewardsRecipient>);

impl RewardsRecipientList {
    pub fn new(list: Vec<RewardsRecipient>) -> Result<Self, String> {
        Self::validate(&list)?;
        Ok(Self(list))
    }

    fn validate(list: &Vec<RewardsRecipient>) -> Result<(), String> {
        if list.is_empty() {
            return Err("Invalid rewards recipients: empty list.".to_string());
        }
        // expecting 4 recipients in the current design. Limit can be lifted if needed.
        if list.len() > 5 {
            return Err("Invalid rewards recipients: too many recipients.".to_string());
        }
        let mut sum = 0;
        for recipient in list {
            if recipient.account.owner == Principal::anonymous() {
                return Err("Invalid rewards recipient: account owner is anonymous.".to_string());
            }
            if recipient.reward_weight == 0 || recipient.reward_weight > 10000 {
                return Err(
                    "Invalid rewards recipient: reward weight has to be between 1 and 10000.".to_string()
                );
            }
            sum += recipient.reward_weight;
        }
        if sum != 10000 {
            return Err(
                "Invalid rewards recipient: the sum of all needs to add up to 10000.".to_string()
            );
        }
        Ok(())
    }
}
