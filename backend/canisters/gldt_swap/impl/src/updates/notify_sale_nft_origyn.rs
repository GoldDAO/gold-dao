use gldt_swap_common::swap::{ SwapInfo, SwapStatusForward };

pub use gldt_swap_api_canister::notify_sale_nft_origyn::Args as SubscriberNotification;
use ic_cdk::update;
use tracing::debug;
use crate::{
    guards::caller_is_nft_canister,
    state::read_state,
    swap::forward_swap::{
        forward_swap_perform_bid_on_nft,
        forward_swap_perform_burn_fees,
        forward_swap_perform_mint_to_escrow,
        forward_swap_validate_notification,
    },
};

#[update(guard = "caller_is_nft_canister", hidden = true)]
async fn notify_sale_nft_origyn(args: SubscriberNotification) {
    notify_sale_nft_origyn_impl(args).await;
}

pub async fn notify_sale_nft_origyn_impl(args: SubscriberNotification) {
    match read_state(|s| s.data.swaps.get_active_swap_by_string_id(&args.sale.token_id)) {
        Some((swap_id, swap_info)) => {
            // TODO
            // if the swap already has a sale_id that isn't equal to a blank string then its a duplicate notify_sale_nft_origyn request or a fraudulent one

            forward_swap_validate_notification(&swap_id, &args);
            forward_swap_perform_mint_to_escrow(&swap_id).await;
            forward_swap_perform_bid_on_nft(&swap_id, args).await;
            forward_swap_perform_burn_fees(&swap_id).await;
        }
        None => {
            let nft_string_id = &args.sale.token_id;
            let msg = format!(
                "FORWARD SWAP :: notification endpoint :: NFT ID string = {nft_string_id}. msg = no swap was found"
            );
            debug!(msg);
        }
    }
}
