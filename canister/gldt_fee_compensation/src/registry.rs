use candid::{ CandidType, Deserialize, Principal };

use icrc_ledger_types::icrc1::{ account::Account, transfer::{ NumTokens, BlockIndex } };
use serde::Serialize;
use std::collections::{ BTreeMap, btree_map };

use gldt_libs::types::NftSaleId;

use crate::error::CustomError;
use crate::Index;

/// The registry that keeps track of which royalties have been compensated.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct Registry {
    registry: BTreeMap<(Account, NftSaleId), FeeRegistryEntry>,
}

impl Registry {
    pub fn init_entry(
        &mut self,
        key: (Account, NftSaleId),
        entry: FeeRegistryEntry
    ) -> Result<(), String> {
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(v) => {
                v.insert(entry);
                Ok(())
            }
            btree_map::Entry::Occupied(mut o) => {
                // If there is already an entry, only continue if it's a previous failed
                // entry. Otherwise, a double redemption try may have happened.
                if o.get().did_fail() {
                    o.insert(FeeRegistryEntry {
                        previous_entry: Some(Box::new(o.get().clone())),
                        ..entry
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
    pub fn update_failed(&mut self, key: (Account, NftSaleId), message: CustomError) {
        match self.registry.get_mut(&key) {
            Some(entry) => {
                entry.status = RegistryStatus::Failed(message);
            }
            None => {}
        }
    }
    pub fn update_completed(&mut self, key: (Account, NftSaleId), block_height: BlockIndex) {
        match self.registry.get_mut(&key) {
            Some(entry) => {
                entry.status = RegistryStatus::Success;
                entry.block_height = Some(block_height);
            }
            None => {}
        }
    }
}

/// The status of the registry entry to avoid double compensation.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
enum RegistryStatus {
    Success,
    Failed(CustomError),
    Ongoing,
}

/// Entry into the registry allows to keep a record of which fees have been compensated.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct FeeRegistryEntry {
    /// The amount of GLDT compensated
    amount: NumTokens,
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

impl FeeRegistryEntry {
    pub fn new(
        amount: NumTokens,
        gld_nft_canister_id: Principal,
        timestamp: u64,
        history_index: Index,
        block_height: Option<BlockIndex>
    ) -> Self {
        Self {
            amount,
            block_height,
            gld_nft_canister_id,
            history_index,
            status: RegistryStatus::Ongoing,
            timestamp,
            previous_entry: None,
        }
    }

    pub fn get_amount(&self) -> NumTokens {
        self.amount.clone()
    }

    pub fn did_fail(&self) -> bool {
        match self.status {
            RegistryStatus::Failed(_) => true,
            _ => false,
        }
    }
}
