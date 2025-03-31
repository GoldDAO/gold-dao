use ::types::BuildVersion;
use candid::Nat;
use utils::env::CanisterEnv;

use crate::state::NeuronManagers;
use crate::state::RuntimeState;
use crate::types::OgyManager;
use crate::types::WtnManager;

use self::types::state::RuntimeStateV0;
use crate::state::Data;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: CanisterEnv::new(
                old_state.env.is_test_mode(),
                BuildVersion::default(),
                "".to_string(),
            ),
            data: Data {
                authorized_principals: old_state.data.authorized_principals,
                sns_rewards_canister_id: old_state.data.sns_rewards_canister_id,
                neuron_managers: NeuronManagers {
                    now: old_state.data.neuron_managers.now,
                    ogy: OgyManager {
                        ogy_sns_governance_canister_id: old_state
                            .data
                            .neuron_managers
                            .ogy
                            .ogy_sns_governance_canister_id,
                        ogy_sns_ledger_canister_id: old_state
                            .data
                            .neuron_managers
                            .ogy
                            .ogy_sns_ledger_canister_id,
                        ogy_sns_rewards_canister_id: old_state
                            .data
                            .neuron_managers
                            .ogy
                            .ogy_sns_rewards_canister_id,
                        neurons: old_state.data.neuron_managers.ogy.neurons,
                        ogy_rewards_threshold: Nat::from(100_000_000 * 1_000_000_u64), // 1_000_000 OGY
                    },
                    wtn: WtnManager::default(),
                },
            },
        }
    }
}
