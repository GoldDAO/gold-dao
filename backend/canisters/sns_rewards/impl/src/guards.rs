use crate::state::read_state;

pub fn caller_is_governance_principal() -> Result<(), String> {
    if read_state(|state| state.is_caller_governance_principal()) {
        Ok(())
    } else {
        Err("Caller is not a governance principal".to_string())
    }
}

pub fn is_test_mode() -> Result<(), String> {
    if read_state(|s| s.env.is_test_mode()) {
        Ok(())
    } else {
        Err("Function only available in test mode.".to_string())
    }
}
