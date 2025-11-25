// pub use gldt_swap_api_canister::get_owned_nfts::{
//     Args as GetOwnedNftsArgs, Response as GetOwnedNftsResponse,
// };
// use ic_cdk::query;

// use crate::state::read_state;

// #[query(hidden = true)]
// async fn get_owned_nfts(_: GetOwnedNftsArgs) -> GetOwnedNftsResponse {
//     read_state(|s| s.data.canister_owned_nfts.clone())
// }
