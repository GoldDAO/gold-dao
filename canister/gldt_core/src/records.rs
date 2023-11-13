use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;
use std::collections::{ BTreeMap, HashMap };
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::BlockIndex };

use gldt_libs::types::{ NftId, GldtNumTokens, NftWeight };

#[cfg(not(test))]
pub const MAX_NUMBER_OF_RECORDS: usize = 64000;
#[cfg(test)]
pub const MAX_NUMBER_OF_RECORDS: usize = 64;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Records {
    pub entries: BTreeMap<BlockIndex, GldtRecord>,
    pub entries_by_user: HashMap<Principal, Vec<BlockIndex>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum RecordType {
    Mint,
    Burn,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum RecordStatus {
    Success,
    Failed,
    Ongoing,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct RecordStatusInfo {
    pub status: RecordStatus,
    pub message: Option<String>,
}

/// Record of successful minting or burning of GLDT for GLD NFTs
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct GldtRecord {
    /// The type of transaction
    record_type: RecordType,
    /// Timestamp of the record entry
    timestamp: u64,
    /// The account who is swapping the NFT for GLDT or vice versa.
    counterparty: Account,
    /// The canister ID of the Origyn NFT canister that manages this NFT.
    gld_nft_canister_id: Principal,
    /// The id of the NFT that was locked up
    nft_id: NftId,
    /// The escrow account where the GLDT tokens are sent to for the trade.
    escrow_subaccount: Subaccount,
    /// The sale id of the NFT listing in the GLD NFT canister
    nft_sale_id: String,
    /// The number of grams that this NFT is reported to have.
    grams: NftWeight,
    /// The amount of tokens minted.
    num_tokens: GldtNumTokens,
    /// The block index on the GLDT ledger when the GLDT were minted or burned.
    block_height: BlockIndex,
    /// The status of the record
    status: RecordStatusInfo,
}

impl GldtRecord {
    pub fn new(
        record_type: RecordType,
        timestamp: u64,
        counterparty: Account,
        gld_nft_canister_id: Principal,
        nft_id: NftId,
        escrow_subaccount: Subaccount,
        nft_sale_id: String,
        grams: NftWeight,
        num_tokens: GldtNumTokens,
        block_height: BlockIndex,
        status: RecordStatusInfo
    ) -> Self {
        Self {
            record_type,
            timestamp,
            counterparty,
            gld_nft_canister_id,
            nft_id,
            escrow_subaccount,
            nft_sale_id,
            grams,
            num_tokens,
            block_height,
            status,
        }
    }
}

impl Records {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
