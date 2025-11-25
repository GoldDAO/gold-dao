use crate::state::icrc3_commit_prepared_transaction;
use crate::utils::prepare_transactions;
pub use gldt_swap_api_canister::_insert_fake_bulk_swaps::{
    Args as InsertFakeBulkSwapsArgs, Response as InsertFakeBulkSwapsResponse,
};
use tracing::error;
use tracing::info;

#[cfg(feature = "inttest")]
use crate::guards::caller_is_authorized;
#[cfg(feature = "inttest")]
use ic_cdk::update;

#[update(guard = "caller_is_authorized")]
#[cfg(feature = "inttest")]
async fn _insert_fake_bulk_swaps(args: InsertFakeBulkSwapsArgs) -> InsertFakeBulkSwapsResponse {
    _insert_fake_bulk_swaps_impl(args).await
}

async fn _insert_fake_bulk_swaps_impl(
    args: InsertFakeBulkSwapsArgs,
) -> InsertFakeBulkSwapsResponse {
    let prepared_transactions = prepare_transactions(&args).unwrap();

    for (swap_index, (swap_transaction, prepared_transaction)) in prepared_transactions {
        if let Err(e) = icrc3_commit_prepared_transaction(
            swap_transaction.clone(),
            prepared_transaction.timestamp,
        ) {
            ic_cdk::println!("Commit failed for swap_index {:?}: {:?}", swap_index, e);
            error!(
                "icrc3_commit_prepared_transaction failed for swap_index {:?}: {:?}",
                swap_index, e
            );
        } else {
            ic_cdk::println!(
                "Successfully committed transaction for swap_index {:?}",
                swap_index
            );
            info!(
                "Successfully committed transaction for swap_index {:?}",
                swap_index
            );
        }
    }

    Ok(())
}
