use candid::{ CandidType, Principal };
use std::collections::{ BTreeMap, HashMap };
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::BlockIndex };
use serde::ser::{ Serialize, Serializer, SerializeStruct };
use serde::de::{ self, Deserialize, Deserializer, MapAccess, Visitor };
use serde::Serialize as Serialize_default;
use serde::Deserialize as Deserialize_default;

use std::fmt;
use std::marker::PhantomData;

use gldt_libs::types::{ NftId, GldtNumTokens, NftWeight };

#[cfg(not(test))]
pub const MAX_NUMBER_OF_RECORDS: usize = 64000;
#[cfg(test)]
pub const MAX_NUMBER_OF_RECORDS: usize = 64;

#[derive(CandidType, Clone, Debug, Default)]
pub struct Records {
    pub entries: BTreeMap<BlockIndex, GldtRecord>,
    pub entries_by_user: HashMap<Principal, Vec<BlockIndex>>,
}

impl Serialize for Records {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Records", 2)?;

        let entries_as_strings: HashMap<String, &GldtRecord> = self.entries
            .iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        state.serialize_field("entries", &entries_as_strings)?;

        let entries_by_user_as_strings: HashMap<String, &Vec<BlockIndex>> = self.entries_by_user
            .iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        state.serialize_field("entries_by_user", &entries_by_user_as_strings)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for Records {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct RecordsVisitor {
            marker: PhantomData<fn() -> Records>,
        }

        impl<'de> Visitor<'de> for RecordsVisitor {
            type Value = Records;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A map representing entries and entries_by_user")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Records, V::Error> where V: MapAccess<'de> {
                let mut entries = None;
                let mut entries_by_user = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "entries" => {
                            entries = Some(map.next_value::<BTreeMap<BlockIndex, GldtRecord>>()?);
                        }
                        "entries_by_user" => {
                            entries_by_user = Some(
                                map.next_value::<HashMap<Principal, Vec<BlockIndex>>>()?
                            );
                        }
                        _ => {
                            return Err(de::Error::unknown_field(&key, FIELDS));
                        }
                    }
                }

                let entries = entries.ok_or_else(|| de::Error::missing_field("entries"))?;
                let entries_by_user = entries_by_user.ok_or_else(||
                    de::Error::missing_field("entries_by_user")
                )?;

                Ok(Records {
                    entries,
                    entries_by_user,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["entries", "entries_by_user"];
        deserializer.deserialize_map(RecordsVisitor { marker: PhantomData })
    }
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum RecordType {
    Mint,
    Burn,
}
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum RecordStatus {
    Success,
    Failed,
    Ongoing,
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub struct RecordStatusInfo {
    pub status: RecordStatus,
    pub message: Option<String>,
}

/// Record of successful minting or burning of GLDT for GLD NFTs
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
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
