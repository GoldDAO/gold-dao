use crate::{ state::{ Data, RuntimeState }, types::outstanding_payments::OutstandingPaymentsList };

use self::types::state::RuntimeStateV0;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                public_key: old_state.data.public_key,
                authorized_principals: old_state.data.authorized_principals,
                neurons: old_state.data.neurons,
                nns_governance_canister_id: old_state.data.nns_governance_canister_id,
                icp_ledger_canister_id: old_state.data.icp_ledger_canister_id,
                rewards_recipients: old_state.data.rewards_recipients,
                outstanding_payments: OutstandingPaymentsList::default(),
            },
        }
    }
}
