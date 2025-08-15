use crate::guards::caller_is_whitelisted;
use crate::state::mutate_state;
pub use gldt_stake_api_canister::add_whitelisted_principal::{
    Args as AddWhitelistPrincipalArgs, Response as AddWhitelistPrincipalResponse,
};
use ic_cdk::update;

#[update(guard = "caller_is_whitelisted")]
async fn add_whitelisted_principal(
    args: AddWhitelistPrincipalArgs,
) -> AddWhitelistPrincipalResponse {
    add_whitelisted_principal_impl(args).await
}

async fn add_whitelisted_principal_impl(
    mut args: AddWhitelistPrincipalArgs,
) -> AddWhitelistPrincipalResponse {
    mutate_state(|s| s.data.whitelist.append(&mut args));

    Ok("Whitelist updated".to_string())
}
