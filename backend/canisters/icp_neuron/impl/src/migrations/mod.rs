use ::types::BuildVersion;
use ic_ledger_types::AccountIdentifier;
use utils::env::CanisterEnv;

use crate::state::{Data, RuntimeState};

use self::types::state::RuntimeStateV0;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        // migrate cycle management account
        let mut cycle_management_account = match old_state.data.cycle_management_account {
            Some(account) => vec![account],
            None => vec![],
        };
        // extend by new account
        if let Ok(new_cycle_management_account) = AccountIdentifier::from_hex(
            "8fab530a08fc70fd40140c5b4896fca7a8b8dab1e7ff2f3d60aa21f248a256e9",
        ) {
            cycle_management_account.push(new_cycle_management_account);
        }

        // construct new state
        Self {
            env: CanisterEnv::new(
                old_state.env.is_test_mode(),
                BuildVersion::default(),
                "".to_string(),
            ),
            data: Data {
                public_key: old_state.data.public_key,
                authorized_principals: old_state.data.authorized_principals,
                neurons: old_state.data.neurons,
                nns_governance_canister_id: old_state.data.nns_governance_canister_id,
                icp_ledger_canister_id: old_state.data.icp_ledger_canister_id,
                rewards_recipients: old_state.data.rewards_recipients,
                outstanding_payments: old_state.data.outstanding_payments,
                cycle_management_account,
            },
        }
    }
}
