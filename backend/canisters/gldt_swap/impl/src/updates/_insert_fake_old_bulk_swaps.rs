use crate::state::icrc3_commit_prepared_transaction;
pub use gldt_swap_api_canister::_insert_fake_old_bulk_swaps::{
    Args as InsertFakeOldBulkSwapsArgs, Response as InsertFakeOldBulkSwapsResponse,
};
use tracing::error;
use tracing::info;

#[cfg(feature = "inttest")]
use crate::guards::caller_is_authorized;
#[cfg(feature = "inttest")]
use ic_cdk::update;

#[update(guard = "caller_is_authorized")]
#[cfg(feature = "inttest")]
async fn _insert_fake_old_bulk_swaps(
    args: InsertFakeOldBulkSwapsArgs,
) -> InsertFakeOldBulkSwapsResponse {
    _insert_fake_old_bulk_swaps_impl(args).await
}

async fn _insert_fake_old_bulk_swaps_impl(
    args: InsertFakeOldBulkSwapsArgs,
) -> InsertFakeOldBulkSwapsResponse {
    let prepared_transactions = prepare_old_transactions(&args).unwrap();

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

use crate::model::swap_transaction::SwapTransaction;
use crate::state::icrc3_prepare_transaction;
use bity_ic_icrc3::types::prepare_transaction::PreparedTransaction;
use gldt_swap_api_archive::swap::SwapIndex;
use gldt_swap_api_archive::swap::SwapInfo;
use gldt_swap_common::general_error::GeneralError;
use std::collections::HashMap;
pub fn prepare_old_transactions(
    swaps: &HashMap<SwapIndex, SwapInfo>,
) -> Result<HashMap<SwapIndex, (SwapTransaction, PreparedTransaction)>, GeneralError> {
    let mut prepared_transactions = HashMap::new();

    for (swap_id, swap) in swaps {
        let transaction = SwapTransaction::migrate(swap.clone());

        match icrc3_prepare_transaction(transaction.clone()) {
            Ok(prepared_tx) => {
                prepared_transactions.insert(swap_id.clone(), (transaction, prepared_tx));
            }
            Err(err) => {
                error!(
                    "Failed to prepare transaction for swap {:?}: {:?}",
                    swap_id, err
                );
                return Err(GeneralError::TransactionPreparationError(err.to_string()));
            }
        }
    }

    info!(
        "Prepared {} ICRC3 transactions",
        prepared_transactions.len()
    );

    Ok(prepared_transactions)
}
