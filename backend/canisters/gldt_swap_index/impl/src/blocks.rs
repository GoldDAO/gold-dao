use crate::cache::{insert_value, try_get_value};
use crate::state::read_state;
use bity_ic_icrc3_archive_c2c_client::icrc3_get_blocks as archive_get_blocks;
use bity_ic_icrc3_c2c_client::icrc3_get_blocks;
use candid::Principal;
use candid::{CandidType, Nat};
use gldt_swap_common::nft::Nft;
use gldt_swap_common::nft_migration_mapping::parse_nft;
use gldt_swap_common::swap::SwapStatus;
use gldt_swap_index_api_canister::index::SortBy;
use gldt_swap_index_api_canister::wrapped_values::WrappedAccount;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc3::blocks::{BlockWithId, GetBlocksRequest};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::error;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockType {
    ForwardSwap(ForwardSwapBlock),
    ReverseSwap(ReverseSwapBlock),
    ForwardSwapOld(ForwardSwapOldBlock),
    ReverseSwapOld(ReverseSwapOldBlock),
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForwardSwapBlock;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReverseSwapBlock;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForwardSwapOldBlock;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReverseSwapOldBlock;

impl FromStr for BlockType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward_swap" => Ok(BlockType::ForwardSwap(ForwardSwapBlock)),
            "reverse_swap" => Ok(BlockType::ReverseSwap(ReverseSwapBlock)),
            "forward_swap_old" => Ok(BlockType::ForwardSwapOld(ForwardSwapOldBlock)),
            "reverse_swap_old" => Ok(BlockType::ReverseSwapOld(ReverseSwapOldBlock)),
            _ => Err(format!("Unknown block type: {}", s)),
        }
    }
}

impl ToString for BlockType {
    fn to_string(&self) -> String {
        match self {
            BlockType::ForwardSwap(_) => "forward_swap".to_string(),
            BlockType::ReverseSwap(_) => "reverse_swap".to_string(),
            BlockType::ForwardSwapOld(_) => "forward_swap_old".to_string(),
            BlockType::ReverseSwapOld(_) => "reverse_swap_old".to_string(),
        }
    }
}

pub fn get_block_instance(block_type: &BlockType) -> Box<dyn TransactionDataExtractor> {
    match block_type {
        BlockType::ForwardSwap(_) => Box::new(ForwardSwapBlock),
        BlockType::ReverseSwap(_) => Box::new(ReverseSwapBlock),
        BlockType::ForwardSwapOld(_) => Box::new(ForwardSwapOldBlock),
        BlockType::ReverseSwapOld(_) => Box::new(ReverseSwapOldBlock),
    }
}

pub trait BlockIndexer {
    fn should_index_by_timestamp(&self) -> bool;
    fn should_index_by_account(&self) -> bool;

    fn should_index_by_block_type(&self) -> bool;
    fn account_to_index(&self) -> Option<WrappedAccount>;
}

pub trait TransactionDataExtractor {
    fn extract_accounts(&self, data: &ICRC3Value) -> Result<Vec<WrappedAccount>, String>;
    fn extract_swap_statuses(&self, data: &ICRC3Value) -> Result<Vec<SwapStatus>, String>;
    fn extract_nfts(&self, data: &ICRC3Value) -> Result<Vec<Nft>, String>;

    fn extract_block_type(&self, data: &ICRC3Value) -> Result<BlockType, String> {
        if let ICRC3Value::Map(map) = data {
            if let Some(ICRC3Value::Text(btype_str)) = map.get("btype") {
                Ok(BlockType::from_str(btype_str).map_err(|_| "Invalid block type".to_string())?)
            } else {
                error!("Missing or invalid block type field");
                Err("Missing or invalid block type field".to_string())
            }
        } else {
            error!("Invalid block data");
            Err("Invalid block data".to_string())
        }
    }

    fn extract_timestamp(&self, data: &ICRC3Value) -> Result<u64, String> {
        if let ICRC3Value::Map(map) = data {
            if let Some(ICRC3Value::Nat(timestamp_nat)) = map.get("timestamp") {
                Ok(u64::try_from(timestamp_nat.0.clone())
                    .map_err(|_| "Invalid timestamp format")?)
            } else {
                Err("Missing or invalid timestamp field".to_string())
            }
        } else {
            Err("Invalid block data".to_string())
        }
    }
}

impl BlockIndexer for BlockType {
    fn should_index_by_timestamp(&self) -> bool {
        true
    }

    fn should_index_by_account(&self) -> bool {
        match self {
            BlockType::ForwardSwap(_) => true,
            BlockType::ReverseSwap(_) => true,
            BlockType::ForwardSwapOld(_) => true,
            BlockType::ReverseSwapOld(_) => true,
        }
    }

    fn should_index_by_block_type(&self) -> bool {
        true
    }

    fn account_to_index(&self) -> Option<WrappedAccount> {
        None
    }
}

impl TransactionDataExtractor for ForwardSwapBlock {
    fn extract_accounts(&self, data: &ICRC3Value) -> Result<Vec<WrappedAccount>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut accounts = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(ICRC3Value::Text(from)) = tx.get("from") {
                        if let Ok(account) = WrappedAccount::from_str(from) {
                            accounts.push(account);
                        }
                    }
                    if let Some(ICRC3Value::Text(to)) = tx.get("to") {
                        if let Ok(account) = WrappedAccount::from_str(to) {
                            accounts.push(account);
                        }
                    }
                }
                Ok(accounts)
            }
            _ => Err("Transfer transaction data must be a map".to_string()),
        }
    }

    fn extract_swap_statuses(&self, data: &ICRC3Value) -> Result<Vec<SwapStatus>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut statuses = Vec::new();

                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(status) = tx.get("status") {
                        match SwapStatus::try_from(&status.clone()) {
                            Ok(parsed_status) => {
                                statuses.push(parsed_status);
                            }
                            Err(e) => {
                                error!("extract_swap_statuses: Failed to parse SwapStatus: {}", e);
                                return Err(format!("Failed to parse SwapStatus: {}", e));
                            }
                        }
                    } else {
                        error!("extract_swap_statuses: No 'status' field found in 'tx'");
                    }
                } else {
                    error!("extract_swap_statuses: No 'tx' map found in top-level map");
                }

                Ok(statuses)
            }
            other => {
                error!("extract_swap_statuses: Data is not a map: {:?}", other);
                Err("Transfer transaction data must be a map".to_string())
            }
        }
    }

    fn extract_nfts(&self, data: &ICRC3Value) -> Result<Vec<Nft>, String> {
        let mut nfts = Vec::new();

        match data {
            ICRC3Value::Map(map) => {
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(nft_value) = tx.get("nft") {
                        match Nft::try_from(nft_value.clone()) {
                            Ok(nft) => nfts.push(nft),
                            Err(e) => return Err(format!("Failed to parse NFT: {}", e)),
                        }
                    }
                }
                Ok(nfts)
            }
            _ => Err("SwapTransaction must be a map".into()),
        }
    }
}

impl TransactionDataExtractor for ReverseSwapBlock {
    fn extract_accounts(&self, data: &ICRC3Value) -> Result<Vec<WrappedAccount>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut accounts = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(ICRC3Value::Text(to)) = tx.get("to") {
                        if let Ok(account) = WrappedAccount::from_str(to) {
                            accounts.push(account);
                        }
                    }
                }
                Ok(accounts)
            }
            _ => Err("Mint transaction data must be a map".to_string()),
        }
    }

    fn extract_swap_statuses(&self, data: &ICRC3Value) -> Result<Vec<SwapStatus>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut statuses = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(status) = tx.get("status") {
                        statuses.push(
                            SwapStatus::try_from(status).expect("Failed to parse swap status"),
                        );
                    }
                }
                Ok(statuses)
            }
            _ => Err("Transfer transaction data must be a map".to_string()),
        }
    }

    fn extract_nfts(&self, data: &ICRC3Value) -> Result<Vec<Nft>, String> {
        let mut nfts = Vec::new();

        match data {
            ICRC3Value::Map(map) => {
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(nft_value) = tx.get("nft") {
                        match Nft::try_from(nft_value.clone()) {
                            Ok(nft) => nfts.push(nft),
                            Err(e) => return Err(format!("Failed to parse NFT: {}", e)),
                        }
                    }
                }
                Ok(nfts)
            }
            _ => Err("SwapTransaction must be a map".into()),
        }
    }
}

impl TransactionDataExtractor for ForwardSwapOldBlock {
    fn extract_accounts(&self, data: &ICRC3Value) -> Result<Vec<WrappedAccount>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut accounts = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(ICRC3Value::Text(gldt_receiver)) = tx.get("gldt_receiver") {
                        if let Ok(account) = WrappedAccount::from_str(gldt_receiver) {
                            accounts.push(account);
                        }
                    }
                }
                Ok(accounts)
            }
            _ => Err("Burn transaction data must be a map".to_string()),
        }
    }

    fn extract_swap_statuses(&self, data: &ICRC3Value) -> Result<Vec<SwapStatus>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut statuses = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(status) = tx.get("status") {
                        statuses.push(
                            SwapStatus::try_from(status).expect("Failed to parse swap status"),
                        );
                    }
                }
                Ok(statuses)
            }
            _ => Err("Transfer transaction data must be a map".to_string()),
        }
    }

    fn extract_nfts(&self, data: &ICRC3Value) -> Result<Vec<Nft>, String> {
        let mut nfts = Vec::new();

        match data {
            ICRC3Value::Map(map) => {
                // try to find nft_id_string
                let nft_id = match map.get("nft_id_string") {
                    Some(ICRC3Value::Text(id)) => id.clone(),
                    Some(other) => {
                        return Err(format!("Expected Nat for nft_id_string, found {:?}", other))
                    }
                    None => return Err("Missing nft_id_string".into()),
                };

                let nft = parse_nft(&nft_id).ok_or("Such NFT was not found".to_string())?;

                nfts.push(nft);

                Ok(nfts)
            }
            _ => Err("SwapTransaction must be a map".into()),
        }
    }
}

impl TransactionDataExtractor for ReverseSwapOldBlock {
    fn extract_accounts(&self, data: &ICRC3Value) -> Result<Vec<WrappedAccount>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut accounts = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(ICRC3Value::Text(user)) = tx.get("user") {
                        if let Ok(principal) = Principal::from_text(user)
                            .map_err(|e| format!("Invalid principal: {}", e))
                        {
                            accounts.push(Account::from(principal).into());
                        }
                    }
                }
                Ok(accounts)
            }
            _ => Err("Burn transaction data must be a map".to_string()),
        }
    }

    fn extract_swap_statuses(&self, data: &ICRC3Value) -> Result<Vec<SwapStatus>, String> {
        match data {
            ICRC3Value::Map(map) => {
                let mut statuses = Vec::new();
                if let Some(ICRC3Value::Map(tx)) = map.get("tx") {
                    if let Some(status) = tx.get("status") {
                        statuses.push(
                            SwapStatus::try_from(status).expect("Failed to parse swap status"),
                        );
                    }
                }
                Ok(statuses)
            }
            _ => Err("Transfer transaction data must be a map".to_string()),
        }
    }

    fn extract_nfts(&self, data: &ICRC3Value) -> Result<Vec<Nft>, String> {
        let mut nfts = Vec::new();

        match data {
            ICRC3Value::Map(map) => {
                // try to find nft_id_string
                let nft_id = match map.get("nft_id_string") {
                    Some(ICRC3Value::Text(id)) => id.clone(),
                    Some(other) => {
                        return Err(format!("Expected Nat for nft_id_string, found {:?}", other))
                    }
                    None => return Err("Missing nft_id_string".into()),
                };

                let nft = parse_nft(&nft_id).ok_or("Such NFT was not found".to_string())?;

                nfts.push(nft);

                Ok(nfts)
            }
            _ => Err("SwapTransaction must be a map".into()),
        }
    }
}

pub async fn get_all_blocks(
    block_ids: Vec<u64>,
    sort_by: Option<SortBy>,
) -> Result<Vec<BlockWithId>, String> {
    if block_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut ret_blocks: Vec<BlockWithId> = Vec::new();
    let mut uncached_block_ids: Vec<u64> = Vec::new();

    // First, check cache for each block ID
    for &block_id in &block_ids {
        if let Some(cached_block) = try_get_value(block_id) {
            let block_with_id = BlockWithId {
                id: Nat::from(block_id),
                block: cached_block.0,
            };
            ret_blocks.push(block_with_id);
        } else {
            uncached_block_ids.push(block_id);
        }
    }

    // If all blocks were found in cache, return early
    if uncached_block_ids.is_empty() {
        match sort_by {
            Some(SortBy::Ascending) => {
                ret_blocks.sort_by(|a, b| a.id.cmp(&b.id));
            }
            Some(SortBy::Descending) => {
                ret_blocks.sort_by(|a, b| b.id.cmp(&a.id));
            }
            None => {
                ret_blocks.sort_by(|a, b| a.id.cmp(&b.id));
            }
        }
        return Ok(ret_blocks);
    }

    // Fetch only uncached blocks from network
    let mut sorted_block_ids = uncached_block_ids.clone();
    sorted_block_ids.sort();

    let chunks = group_consecutive_block_ids(&sorted_block_ids);

    let ledger_canister_id = read_state(|state| state.data.ledger_canister_id);

    for chunk in chunks {
        let start = chunk[0];
        let length = chunk.len() as u64;

        let blocks = icrc3_get_blocks(
            ledger_canister_id,
            vec![GetBlocksRequest {
                start: Nat::from(start),
                length: Nat::from(length),
            }],
        )
        .await
        .map_err(|e| e.to_string())?;

        for archive in blocks.archived_blocks {
            let archive_blocks = archive_get_blocks(archive.callback.canister_id, archive.args)
                .await
                .map_err(|e| e.to_string())?;

            for block in archive_blocks.blocks {
                if let Ok(block_id) = u64::try_from(&block.id.0) {
                    insert_value(
                        block_id,
                        gldt_swap_index_api_canister::wrapped_values::CustomValue(
                            block.block.clone(),
                        ),
                    );
                }
                ret_blocks.push(block);
            }
        }

        for block in blocks.blocks {
            if let Ok(block_id) = u64::try_from(&block.id.0) {
                insert_value(
                    block_id,
                    gldt_swap_index_api_canister::wrapped_values::CustomValue(block.block.clone()),
                );
            }
            ret_blocks.push(block);
        }
    }

    match sort_by {
        Some(SortBy::Ascending) => {
            ret_blocks.sort_by(|a, b| a.id.cmp(&b.id));
        }
        Some(SortBy::Descending) => {
            ret_blocks.sort_by(|a, b| b.id.cmp(&a.id));
        }
        None => {
            ret_blocks.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }

    Ok(ret_blocks)
}

/// Groups consecutive block IDs into chunks for efficient batch processing
///
/// # Arguments
/// * `block_ids` - A sorted vector of block IDs
///
/// # Returns
/// A vector of chunks, where each chunk contains consecutive block IDs
///
/// # Example
/// ```
/// use gldt_swap_index::blocks::group_consecutive_block_ids;
/// let block_ids = vec![1, 2, 3, 5, 6, 8, 9, 10];
/// let chunks = group_consecutive_block_ids(&block_ids);
/// // Result: [[1, 2, 3], [5, 6], [8, 9, 10]]
/// ```
pub fn group_consecutive_block_ids(block_ids: &[u64]) -> Vec<Vec<u64>> {
    if block_ids.is_empty() {
        return Vec::new();
    }

    let mut chunks: Vec<Vec<u64>> = Vec::new();
    let mut start_idx = 0;

    for i in 1..block_ids.len() {
        if block_ids[i] != block_ids[i - 1] + 1 {
            // Non-consecutive, create chunk from start_idx to i-1
            chunks.push(block_ids[start_idx..i].to_vec());
            start_idx = i;
        }
    }

    // Add the last chunk
    chunks.push(block_ids[start_idx..].to_vec());

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_consecutive_block_ids_empty() {
        let block_ids: Vec<u64> = vec![];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(chunks, Vec::<Vec<u64>>::new());
    }

    #[test]
    fn test_group_consecutive_block_ids_single() {
        let block_ids = vec![42];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(chunks, vec![vec![42]]);
    }

    #[test]
    fn test_group_consecutive_block_ids_all_consecutive() {
        let block_ids = vec![1, 2, 3, 4, 5];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(chunks, vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn test_group_consecutive_block_ids_no_consecutive() {
        let block_ids = vec![1, 3, 5, 7, 9];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(chunks, vec![vec![1], vec![3], vec![5], vec![7], vec![9]]);
    }

    #[test]
    fn test_group_consecutive_block_ids_mixed() {
        let block_ids = vec![1, 2, 3, 5, 6, 8, 9, 10];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![5, 6], vec![8, 9, 10]]);
    }

    #[test]
    fn test_group_consecutive_block_ids_gaps() {
        let block_ids = vec![1, 2, 5, 6, 7, 10, 11, 15];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(
            chunks,
            vec![vec![1, 2], vec![5, 6, 7], vec![10, 11], vec![15]]
        );
    }

    #[test]
    fn test_group_consecutive_block_ids_large_numbers() {
        let block_ids = vec![1000, 1001, 1002, 2000, 2001, 3000];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(
            chunks,
            vec![vec![1000, 1001, 1002], vec![2000, 2001], vec![3000]]
        );
    }

    #[test]
    fn test_group_consecutive_block_ids_edge_cases() {
        let block_ids = vec![0, 1, 2, 3];
        let chunks = group_consecutive_block_ids(&block_ids);
        assert_eq!(chunks, vec![vec![0, 1, 2, 3]]);
    }

    #[test]
    fn test_group_consecutive_block_ids_reverse_order() {
        // This test shows that the function expects sorted input
        let block_ids = vec![5, 4, 3, 2, 1];
        let chunks = group_consecutive_block_ids(&block_ids);
        // Since input is not sorted, this will create separate chunks
        assert_eq!(chunks, vec![vec![5], vec![4], vec![3], vec![2], vec![1]]);
    }
}
