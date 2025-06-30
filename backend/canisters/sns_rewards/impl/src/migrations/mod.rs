use ::types::BuildVersion;
use utils::env::CanisterEnv;

use crate::state::RuntimeState;

use self::types::state::RuntimeStateV0;

pub mod types;

use crate::state::Data;
use std::convert::TryFrom;
impl TryFrom<RuntimeStateV0> for RuntimeState {
    type Error = String; // Or a custom error type

    fn try_from(old_state: RuntimeStateV0) -> Result<Self, Self::Error> {
        let data = Data::try_from(old_state.data)?;
        Ok(Self {
            env: CanisterEnv::new(
                old_state.env.is_test_mode(),
                BuildVersion::default(),
                "".to_string(),
            ),
            data,
        })
    }
}
