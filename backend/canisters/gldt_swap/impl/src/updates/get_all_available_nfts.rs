use crate::model::nft_manager::NftCollector;
pub use gldt_swap_api_canister::get_available_nfts::{
    Args as GetAvailableNftsArgs, Response as GetAvailableNftsResponse,
};
use ic_cdk::update;

use crate::state::read_state;

#[update]
async fn get_available_nfts(arg: GetAvailableNftsArgs) -> GetAvailableNftsResponse {
    let swap_configs = read_state(|s| s.data.swap_configs.clone());
    let principal = arg.unwrap_or(ic_cdk::api::canister_self());
    let nft_guards = read_state(|s| s.data.nft_guards.clone());

    Ok(swap_configs
        .collect_nfts(principal)
        .await?
        .filter(&nft_guards)
        .grouped)
}
