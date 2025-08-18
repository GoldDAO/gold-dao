use ::types::BuildVersion;
use utils::env::CanisterEnv;

use crate::state::RuntimeState;

use self::types::state::RuntimeStateV0;

pub mod jobs;
pub mod types;

use crate::state::Data;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        let data = Data::from(old_state.data);
        Self {
            env: CanisterEnv::new(
                old_state.env.is_test_mode(),
                BuildVersion::default(),
                "".to_string(),
            ),
            data,
        }
    }
}
