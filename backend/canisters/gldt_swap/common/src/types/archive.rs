use candid::{CandidType, Nat, Principal};
use serde::Serializer;
use serde::{Deserialize, Serialize}; // Ensure Serializer is imported
                                     // converts a principal to its text representation

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ArchiveCanister {
    #[serde(serialize_with = "custom_serialize_json_principal")]
    pub canister_id: Principal,
    #[serde(serialize_with = "custom_serialize_json_nat")]
    pub start_index: Nat,
    #[serde(serialize_with = "custom_serialize_option_nat")]
    pub end_index: Option<Nat>,
    pub active: bool,
}

fn custom_serialize_json_principal<S>(
    principal: &Principal,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_principal = principal.to_text();
    serializer.serialize_str(&string_principal)
}

fn custom_serialize_json_nat<S>(value: &Nat, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_representation = value.0.to_string(); // Convert Nat to string
    serializer.serialize_str(&string_representation)
}

fn custom_serialize_option_nat<S>(
    option_nat: &Option<Nat>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option_nat {
        Some(nat) => {
            let string_representation = nat.0.to_string(); // Convert Nat to string
            serializer.serialize_str(&string_representation)
        }
        None => serializer.serialize_none(),
    }
}
