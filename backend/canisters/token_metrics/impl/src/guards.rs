use crate::state::read_state;

pub fn caller_is_authorised_principal() -> Result<(), String> {
    if read_state(|state| state.is_caller_authorised_principal()) {
        Ok(())
    } else {
        Err("Caller is not an authorised principal".to_string())
    }
}
