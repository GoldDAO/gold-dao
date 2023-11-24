use candid::{ CandidType, Principal };

use icrc_ledger_types::icrc1::{ account::Account, transfer::{ NumTokens, BlockIndex } };
use std::collections::{ BTreeMap, btree_map };
use serde::ser::{ Serialize, Serializer, SerializeMap };
use serde::de::{ self, Deserialize, Deserializer, MapAccess, Visitor };
use serde::Serialize as Serialize_default;
use serde::Deserialize as Deserialize_default;
use std::fmt;
use std::marker::PhantomData;

use gldt_libs::types::NftSaleId;
use gldt_libs::error::Custom as CustomError;

use crate::Index;

/// The registry that keeps track of which royalties have been compensated.
#[derive(CandidType, Clone, Debug, Hash, Default)]
pub struct Registry {
    pub registry: BTreeMap<(Account, NftSaleId), FeeRegistryEntry>,
}

impl Serialize for Registry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(self.registry.len()))?;
        for (k, v) in self.registry.clone() {
            map.serialize_entry(&format!("{}|{}", k.0, k.1).clone(), &v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Registry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct RegistryVisitor {
            marker: PhantomData<fn() -> Registry>,
        }

        impl<'de> Visitor<'de> for RegistryVisitor {
            type Value = Registry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "Expecting example : \"registry\":{\"2vxsx-fae|tmp\":{\"amount\":[],\"block_height\":null,\"gld_nft_canister_id\":\"2vxsx-fae\",\"history_index\":[],\"previous_entry\":null,\"status\":\"Success\",\"timestamp\":0}}"
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<Registry, V::Error> where V: MapAccess<'de> {
                let mut my_map = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, FeeRegistryEntry>()? {
                    let parts: Vec<&str> = key.splitn(2, '|').collect();

                    if parts.len() != 2 {
                        return Err(
                            de::Error::invalid_value(
                                de::Unexpected::Str(&key),
                                &"a key with format 'account|nftSaleId'"
                            )
                        );
                    }

                    let account = parts[0].parse::<Account>().map_err(de::Error::custom)?;
                    let nft_sale_id = parts[1].to_owned();

                    let tuple = (account, nft_sale_id);
                    my_map.insert(tuple, value);
                }
                Ok(Registry { registry: my_map })
            }
        }

        deserializer.deserialize_map(RegistryVisitor { marker: PhantomData })
    }
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
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum Status {
    Success,
    Failed(CustomError),
    Ongoing,
}

/// Entry into the registry allows to keep a record of which fees have been compensated.
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
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
