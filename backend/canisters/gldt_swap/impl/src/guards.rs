use crate::state::read_state;

pub fn caller_is_nft_canister() -> Result<(), String> {
    if read_state(|state| state.is_caller_is_nft_canister()) {
        Ok(())
    } else {
        Err("Caller is not a valid NFT canister".to_string())
    }
}

pub fn caller_is_authorized() -> Result<(), String> {
    if read_state(|state| state.is_caller_authorized()) {
        Ok(())
    } else {
        Err("Caller is not an authorized principal".to_string())
    }
}
