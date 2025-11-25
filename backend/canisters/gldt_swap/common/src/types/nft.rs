use crate::general_error::GeneralError;
use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use minicbor::{Decode, Encode};
use origyn_nft_canister_c2c_client::icrc7_owner_of;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::CanisterId;

#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    PartialOrd,
    Ord,
    Encode,
    Decode,
)]
pub struct Nft {
    #[cbor(n(0), with = "cbor::nat")]
    pub id: Nat,
    #[cbor(n(1), with = "cbor::principal")]
    pub canister_id: CanisterId,
}

impl From<Nft> for ICRC3Value {
    fn from(val: Nft) -> Self {
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), ICRC3Value::Nat(val.id));
        map.insert(
            "canister_id".to_string(),
            ICRC3Value::Text(val.canister_id.to_text()),
        );
        ICRC3Value::Map(map)
    }
}

impl TryFrom<ICRC3Value> for Nft {
    type Error = String;

    fn try_from(value: ICRC3Value) -> Result<Self, Self::Error> {
        match value {
            ICRC3Value::Map(map) => {
                // Extract "id"
                let id = match map.get("id") {
                    Some(ICRC3Value::Nat(n)) => n.clone(),
                    other => return Err(format!("Expected Nat for key 'id', got {:?}", other)),
                };

                // Extract "canister_id"
                let canister_id = match map.get("canister_id") {
                    Some(ICRC3Value::Text(s)) => {
                        Principal::from_text(s).map_err(|e| format!("Invalid principal: {}", e))?
                    }
                    other => {
                        return Err(format!(
                            "Expected Text for key 'canister_id', got {:?}",
                            other
                        ))
                    }
                };

                Ok(Nft { id, canister_id })
            }
            other => Err(format!("Expected ICRC3Value::Map, got {:?}", other)),
        }
    }
}

impl Nft {
    pub async fn validate_nft_owner(self, user: Principal) -> Result<(), GeneralError> {
        let result = icrc7_owner_of(self.canister_id, vec![self.id]).await;

        let owners = result.map_err(|e| GeneralError::CallError(format!("{e:?}")))?;

        match owners.first().and_then(|opt| *opt) {
            Some(owner) if owner == user.into() => Ok(()),
            _ => Err(GeneralError::UserIsNotNftOwner(
                "User is not NFT owner".to_string(),
            )),
        }
    }
}
