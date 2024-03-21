use serde::{ Deserialize, Serialize };
use candid::Principal;
use types::RewardsRecipientList;
use utils::env::CanisterEnv;

use crate::state::Neurons;

use super::outstanding_payments::OutstandingPaymentsList;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
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
