use candid::Principal;
use usdg_minter_api::VaultError;

pub mod timer;
pub mod vault;

pub fn reject_anonymous_caller() -> Result<(), VaultError> {
    if ic_cdk::caller() == Principal::anonymous() {
        return Err(VaultError::AnonymousCaller);
    }
    Ok(())
}
