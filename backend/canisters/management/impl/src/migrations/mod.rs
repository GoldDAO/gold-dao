use self::types::state::RuntimeStateV0;
use crate::state::RuntimeState;
use bity_ic_types::BuildVersion;
use utils::env::CanisterEnv;

pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: CanisterEnv::new(
                old_state.env.is_test_mode(),
                BuildVersion::default(),
                "".to_string(),
            ),
            data: old_state.data,
        }
    }
}
