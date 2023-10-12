use candid::{ CandidType, Deserialize, Principal };

use icrc_ledger_types::icrc1::{ account::Account, transfer::BlockIndex };
use serde::Serialize;
use std::collections::BTreeMap;

use gldt_libs::types::{ GldtNumTokens, NftSaleId };

use crate::error::CustomError;
use crate::Index;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct Registry {
    registry: BTreeMap<(Account, NftSaleId), FeeRegistryEntry>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
enum RegistryStatus {
    Success,
    Failed(Option<CustomError>),
    Ongoing,
}

/// Entry into the registry allows to keep a record of which fees have been compensated.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct FeeRegistryEntry {
    /// The amount of GLDT compensated
    amount: GldtNumTokens,
    /// The block height when the compensation was done
    block_height: Option<BlockIndex>,
    /// The canister from where this payment was extracted
    gld_nft_canister_id: Principal,
    /// The index in the history_nft_origyn list where the payment was extracted from.
    history_index: Index,
    /// The status of the compensation
    status: RegistryStatus,
    /// The timestamp of this entry
    timestamp: u64,
    /// Keeping track in case of previous entries that failed
    previous_entry: Option<Box<FeeRegistryEntry>>,
}
