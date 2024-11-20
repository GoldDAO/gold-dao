use crate::numeric::{GLDT, USDG};
use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use usdg_minter_api::ApiFeeBucket;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vault {
    // The vault unique Id.
    pub vault_id: u64,
    // The owner of the vault.
    pub owner: Account,
    // The amount of USDG currently borrowed from this vault.
    // Represented using e8s.
    pub borrowed_amount: USDG,
    // The GLDT margin of this vault.
    // Represented using e8s.
    pub margin_amount: GLDT,
    // The bucket which determines the fee charged over time.
    pub fee_bucket: FeeBucket,
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeeBucket {
    Low,
    Medium,
    High,
}

impl From<ApiFeeBucket> for FeeBucket {
    fn from(api_bucket: ApiFeeBucket) -> FeeBucket {
        match api_bucket {
            ApiFeeBucket::Low => FeeBucket::Low,
            ApiFeeBucket::Medium => FeeBucket::Medium,
            ApiFeeBucket::High => FeeBucket::High,
        }
    }
}
