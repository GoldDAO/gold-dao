use crate::state::mutate_state;
pub use gldt_stake_api_canister::_add_whitelisted_principal::{
    Args as AddWhitelistPrincipalArgs, Response as AddWhitelistPrincipalResponse,
};

#[cfg(feature = "inttest")]
use crate::guards::caller_is_whitelisted;
#[cfg(feature = "inttest")]
use crate::state::read_state;
#[cfg(feature = "inttest")]
use ic_cdk::update;

#[update(guard = "caller_is_whitelisted")]
#[cfg(feature = "inttest")]
async fn _add_whitelisted_principal(
    args: AddWhitelistPrincipalArgs,
) -> AddWhitelistPrincipalResponse {
    _add_whitelisted_principal_impl(args).await
}

async fn _add_whitelisted_principal_impl(
    mut args: AddWhitelistPrincipalArgs,
) -> AddWhitelistPrincipalResponse {
    mutate_state(|s| s.data.whitelist.append(&mut args));

    Ok("Whitelist updated".to_string())
}
