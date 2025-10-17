use candid::Principal;
use ic_ledger_types::AccountIdentifier;
use icp_neuron_common::{neurons::Neurons, outstanding_payments::OutstandingPaymentsList};
use serde::{Deserialize, Serialize};
use types::RewardsRecipientList;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct CanisterEnv {
    test_mode: bool,
}

impl CanisterEnv {
    pub fn is_test_mode(&self) -> bool {
        self.test_mode
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub public_key: Vec<u8>,
    pub authorized_principals: Vec<Principal>,
    pub neurons: Neurons,
    pub nns_governance_canister_id: Principal,
    pub icp_ledger_canister_id: Principal,
    pub rewards_recipients: RewardsRecipientList,
    pub outstanding_payments: OutstandingPaymentsList,
    pub cycle_management_account: Option<AccountIdentifier>,
}
