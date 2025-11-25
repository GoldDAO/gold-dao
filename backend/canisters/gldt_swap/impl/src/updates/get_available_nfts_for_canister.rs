pub use gldt_swap_api_canister::get_available_nfts_for_canister::{
    Args as GetAvailableNftsForCanisterArgs, Response as GetAvailableNftsForCanisterResponse,
};
use ic_cdk::update;

use crate::state::read_state;

#[update]
async fn get_available_nfts_for_canister(
    arg: GetAvailableNftsForCanisterArgs,
) -> GetAvailableNftsForCanisterResponse {
    let nft_guards = read_state(|s| s.data.nft_guards.clone());
    let principal = arg.principal.unwrap_or(ic_cdk::api::canister_self());

    collect_nfts_for_canister(&nft_guards, arg.canister_id, principal).await
}

use candid::{Nat, Principal};
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::nft::Nft;
use origyn_nft_canister::icrc7_tokens_of;
use origyn_nft_canister_c2c_client::icrc7_tokens_of;
use std::collections::BTreeSet;
pub async fn collect_nfts_for_canister(
    nft_guards: &BTreeSet<Nft>,
    canister_id: Principal,
    user: Principal,
) -> Result<Vec<Nat>, GeneralError> {
    let res = icrc7_tokens_of(
        canister_id,
        (
            icrc7_tokens_of::Args0 {
                owner: user,
                subaccount: None,
            },
            None,
            None,
        ),
    )
    .await
    .map_err(|e| GeneralError::CallError(e.to_string()))?;

    let (nft_ids,) = res;

    let available_ids: Vec<Nat> = nft_ids
        .into_iter()
        .filter(|nft_id| {
            let nft = Nft {
                canister_id,
                id: nft_id.clone(),
            };
            !nft_guards.contains(&nft)
        })
        .collect();

    Ok(available_ids)
}
