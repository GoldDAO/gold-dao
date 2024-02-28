use canister_time::{ MINUTE_IN_MS, NANOS_PER_MILLISECOND };

use ic_transport_types::EnvelopeContent;
use k256::{ pkcs8::EncodePublicKey, PublicKey };
use ledger_utils::principal_to_legacy_account_id;
use nns_governance_canister::types::Neuron;
use serde::{ Deserialize, Serialize };
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::{ CanisterId, TimestampMillis, RewardsRecipientList };
use utils::{
    consts::{ ICP_LEDGER_CANISTER_ID, NNS_GOVERNANCE_CANISTER_ID, SNS_GOVERNANCE_CANISTER_ID },
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};

use crate::{
    ecdsa::{ get_key_id, CanisterEcdsaRequest },
    types::{ neuron_metrics::NeuronWithMetric, outstanding_payments::OutstandingPaymentsList },
};

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
                test_mode: self.env.is_test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
            },
            public_key: hex::encode(&self.data.public_key),
            public_key_der: hex::encode(&self.data.get_public_key_der().unwrap_or_default()),
            own_principal: self.data.get_principal(),
            canister_default_account_id: principal_to_legacy_account_id(
                self.env.canister_id(),
                None
            ).to_string(),
            authorized_principals: self.data.authorized_principals.clone(),
            neurons: self.data.get_neuron_list(),
            nns_governance_canister_id: self.data.nns_governance_canister_id,
            icp_ledger_canister_id: self.data.icp_ledger_canister_id,
            rewards_recipients: self.data.rewards_recipients.clone(),
            outstanding_payments: self.data.outstanding_payments.clone(),
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
        nonce: Option<[u8; 8]>
    ) -> Result<CanisterEcdsaRequest, String> {
        let envelope_content = EnvelopeContent::Call {
            nonce: nonce.map(|n| n.to_vec()),
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
    pub canister_default_account_id: String,
    pub authorized_principals: Vec<Principal>,
    pub nns_governance_canister_id: Principal,
    pub icp_ledger_canister_id: Principal,
    pub rewards_recipients: RewardsRecipientList,
    pub neurons: NeuronList,
    pub outstanding_payments: OutstandingPaymentsList,
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
    pub outstanding_payments: OutstandingPaymentsList,
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
            outstanding_payments: OutstandingPaymentsList::default(),
        }
    }

    pub fn get_neuron_list(&self) -> NeuronList {
        NeuronList {
            active: self.neurons.active_neurons
                .iter()
                .map(|n| NeuronWithMetric::from(n.clone()))
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
    active: Vec<NeuronWithMetric>,
    spawning: Vec<u64>,
    disbursed: Vec<u64>,
}
