use candid::{ CandidType, Deserialize, Principal };

use icrc_ledger_types::icrc1::{ account::Account, transfer::{ NumTokens, BlockIndex } };
use serde::Serialize;
use std::collections::{ BTreeMap, btree_map };

use gldt_libs::types::NftSaleId;

use crate::error::Custom as CustomError;
use crate::Index;

/// The registry that keeps track of which royalties have been compensated.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct Registry {
    registry: BTreeMap<(Account, NftSaleId), FeeRegistryEntry>,
}

impl Registry {
    pub fn init_entry(
        &mut self,
        key: &(Account, NftSaleId),
        entry: &FeeRegistryEntry
    ) -> Result<(), String> {
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(v) => {
                v.insert(entry.clone());
                Ok(())
            }
            btree_map::Entry::Occupied(mut o) => {
                // If there is already an entry, only continue if it's a previous failed
                // entry. Otherwise, a double redemption try may have happened.
                if o.get().did_fail() {
                    o.insert(FeeRegistryEntry {
                        previous_entry: Some(Box::new(o.get().clone())),
                        ..entry.clone()
                    });
                    Ok(())
                } else {
                    Err(
                        format!(
                            "There is already an active entry for sale_id {} of user {}. Canceling compensation of tokens.",
                            key.1,
                            key.0
                        )
                    )
                }
            }
        }
    }
    pub fn update_failed(&mut self, key: &(Account, NftSaleId), message: CustomError) {
        if let Some(entry) = self.registry.get_mut(key) {
            entry.status = Status::Failed(message);
        }
    }
    pub fn update_completed(&mut self, key: &(Account, NftSaleId), block_height: BlockIndex) {
        if let Some(entry) = self.registry.get_mut(key) {
            entry.status = Status::Success;
            entry.block_height = Some(block_height);
        }
    }
}

/// The status of the registry entry to avoid double compensation.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum Status {
    Success,
    Failed(CustomError),
    Ongoing,
}

/// Entry into the registry allows to keep a record of which fees have been compensated.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct FeeRegistryEntry {
    /// The amount of GLDT compensated
    pub amount: NumTokens,
    /// The block height when the compensation was done
    pub block_height: Option<BlockIndex>,
    /// The canister from where this payment was extracted
    pub gld_nft_canister_id: Principal,
    /// The index in the history_nft_origyn list where the payment was extracted from.
    pub history_index: Index,
    /// The status of the compensation
    pub status: Status,
    /// The timestamp of this entry
    pub timestamp: u64,
    /// Keeping track in case of previous entries that failed
    pub previous_entry: Option<Box<FeeRegistryEntry>>,
}

impl FeeRegistryEntry {
    pub fn did_fail(&self) -> bool {
        matches!(self.status, Status::Failed(_))
    }
}
