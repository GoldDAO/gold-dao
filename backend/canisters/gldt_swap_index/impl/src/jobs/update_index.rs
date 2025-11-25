use crate::{
    blocks::get_all_blocks,
    index::add_block_to_index,
    state::{mutate_state, read_state},
};
use bity_ic_canister_time::{run_interval, MINUTE_IN_MS};
use std::time::Duration;
use tracing::error;
const UPDATE_INDEX_INTERVAL: u64 = MINUTE_IN_MS;
const BLOCK_BATCH_SIZE: u64 = 100;

pub fn start_job() {
    run_interval(
        Duration::from_millis(UPDATE_INDEX_INTERVAL),
        update_index_job,
    );
}

fn update_index_job() {
    ic_cdk::futures::spawn(update_index());
}

async fn update_index() {
    let mut last_block_id: u64 = read_state(|state| state.data.last_block_id);

    // If last_block_id is 0, try to bootstrap to the first available block
    if last_block_id == 0 {
        match get_first_block_id().await {
            Ok(first_id) => {
                last_block_id = first_id;
            }
            Err(e) => {
                error!("Failed to get first block id: {}", e);
                return;
            }
        }
    }

    // Generate block IDs array starting from last_block_id
    let block_ids: Vec<u64> = (last_block_id..last_block_id + BLOCK_BATCH_SIZE).collect();

    let blocks = get_all_blocks(block_ids, None).await;
    match blocks {
        Ok(blocks) => {
            for block in blocks {
                let block_id_u64: u64 = match u64::try_from(&block.id.0) {
                    Ok(id) => id,
                    Err(_) => {
                        error!("Invalid block id {:?}", block.id);
                        continue;
                    }
                };

                match add_block_to_index(&block) {
                    Ok(_) => {
                        last_block_id = block_id_u64 + 1;
                    }
                    Err(e) => {
                        error!(
                            "Error while adding block {} into index: {:?}",
                            block_id_u64, e
                        );
                        break;
                    }
                }
            }
        }
        Err(e) => {
            error!("Error while fetching blocks: {}", e);
            ic_cdk::trap(e);
        }
    }

    mutate_state(|state| {
        state.data.last_block_id = last_block_id;
    });
}

use bity_ic_icrc3_c2c_client::icrc3_get_blocks;
use candid::Nat;
use icrc_ledger_types::icrc3::blocks::GetBlocksRequest;
async fn get_first_block_id() -> Result<u64, String> {
    let ledger_canister_id = read_state(|state| state.data.ledger_canister_id);

    let blocks = icrc3_get_blocks(
        ledger_canister_id,
        vec![GetBlocksRequest {
            start: Nat::from(0_u64),
            length: Nat::from(1_u64),
        }],
    )
    .await
    .map_err(|e| e.to_string())?;

    // If the ledger has any blocks, the first one returned is the lowest
    if let Some(block) = blocks.blocks.first() {
        u64::try_from(&block.id.0).map_err(|_| "Invalid block id".to_string())
    } else {
        // No blocks exist yet
        Ok(0)
    }
}
