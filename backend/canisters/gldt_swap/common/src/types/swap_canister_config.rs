use crate::general_error::GeneralError;
use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, CandidType, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SwapCanisterConfig {
    pub icrc7_canister_id: Principal, // ICRC-7 canister
    pub fractionalization_config: FractionalizationConfig, // define division per each token
}

#[derive(Deserialize, Serialize, CandidType, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FractionalizationConfig {
    General(GeneralFractionalizationConfig),
    Custom(CustomFractionalizationConfig),
}

impl FractionalizationConfig {
    pub fn get_ledger_id(&self) -> Result<Principal, GeneralError> {
        match self {
            FractionalizationConfig::General(gen_cfg) => Ok(gen_cfg.ledger_id),
            FractionalizationConfig::Custom(custom_cfg) => {
                if let Some(first_cfg) = custom_cfg.per_token_config.values().next() {
                    Ok(first_cfg.ledger_id)
                } else {
                    Err(GeneralError::ConfigNotFound(
                        "Custom fractionalization config has no tokens".to_string(),
                    ))
                }
            }
        }
    }

    pub fn division_for_token(&self, token_id: &Nat) -> Result<Nat, GeneralError> {
        match self {
            FractionalizationConfig::General(gen_cfg) => Ok(Nat::from(gen_cfg.division)),
            FractionalizationConfig::Custom(custom_cfg) => {
                if let Some(gen_cfg) = custom_cfg.per_token_config.get(token_id) {
                    Ok(Nat::from(gen_cfg.division))
                } else {
                    Err(GeneralError::ConfigNotFound(format!(
                        "No custom fractionalization config for token {token_id}"
                    )))
                }
            }
        }
    }

    pub fn swap_fee_for_token(&self, token_id: &Nat) -> Result<Nat, GeneralError> {
        match self {
            FractionalizationConfig::General(gen_cfg) => Ok(gen_cfg.swap_fee.clone()),
            FractionalizationConfig::Custom(custom_cfg) => {
                if let Some(gen_cfg) = custom_cfg.per_token_config.get(token_id) {
                    Ok(gen_cfg.swap_fee.clone())
                } else {
                    Err(GeneralError::ConfigNotFound(format!(
                        "No custom fractionalization config for token {token_id}"
                    )))
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CustomFractionalizationConfig {
    pub per_token_config: BTreeMap<Nat, GeneralFractionalizationConfig>, // define division per each token
}

#[derive(Deserialize, Serialize, CandidType, Debug, Clone, Hash, Eq, PartialEq)]
pub struct GeneralFractionalizationConfig {
    pub division: u64, // amount of tokens for one NFT, e.g., 1000 GLDT per NFT (for uniform NFTs like gold bars)
    pub swap_fee: Nat, // NOTE: fee to be taken for each swap, but minus the ledger fee, so that the value per swap is pretty
    pub ledger_id: Principal, // ICRC-1 ledger canister
}

impl From<GeneralFractionalizationConfig> for ICRC3Value {
    fn from(val: GeneralFractionalizationConfig) -> Self {
        let mut map = BTreeMap::new();
        map.insert(
            "division".to_string(),
            ICRC3Value::Nat(Nat::from(val.division)),
        );
        map.insert("swap_fee".to_string(), ICRC3Value::Nat(val.swap_fee));
        map.insert(
            "ledger_id".to_string(),
            ICRC3Value::Text(val.ledger_id.to_text()),
        );
        ICRC3Value::Map(map)
    }
}
