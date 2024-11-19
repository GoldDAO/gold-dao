pub use gldt_swap_api_canister::get_owned_nfts::{
    Args as GetOwnedNftsArgs, Response as GetOwnedNftsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

// note this should not be used for anything other than integration testing since canister_owned_nfts is
// only updated once an hour and so can't be taken as an accurate map.
#[query(hidden = true)]
async fn get_owned_nfts(_: GetOwnedNftsArgs) -> GetOwnedNftsResponse {
    read_state(|s| s.data.canister_owned_nfts.clone())
}
