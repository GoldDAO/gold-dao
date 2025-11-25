use crate::model::swap_transaction::SwapTransaction;
use crate::state::icrc3_prepare_transaction;
use bity_ic_icrc3::types::prepare_transaction::PreparedTransaction;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::swap::SwapStatus;
use gldt_swap_common::swap::{SwapIndex, SwapInfo};
use std::collections::HashMap;
use tracing::{error, info};

pub fn prepare_transactions(
    swaps: &HashMap<SwapIndex, SwapInfo>,
) -> Result<HashMap<SwapIndex, (SwapTransaction, PreparedTransaction)>, GeneralError> {
    let mut prepared_transactions = HashMap::new();

    for (swap_id, swap) in swaps {
        let mut swap_clone = swap.clone();
        // Only completed swaps will be written to the ledger
        swap_clone.status = SwapStatus::Complete;
        let transaction = SwapTransaction::new(swap_clone);

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
