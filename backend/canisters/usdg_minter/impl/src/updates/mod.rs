use candid::Principal;
use usdg_minter_api::VaultError;

pub mod liquidation_pool;
pub mod vault;

pub fn reject_anonymous_caller() -> Result<(), String> {
    if ic_cdk::caller() == Principal::anonymous() {
        return Err("anonymous caller".to_string());
    }
    Ok(())
}
