use candid::{ CandidType, Deserialize, Nat, Principal };
use crate::gld_nft::{ ICTokenSpec, ICTokenSpec_standard, TokenSpec };

use icrc_ledger_types::icrc1::transfer::NumTokens;
use serde::Serialize;
use std::hash::Hash;

use crate::constants::*;

/// An NFT is identified by a string.
pub type NftId = String;

/// An NFT has a certain weight <65535
pub type NftWeight = u16;

/// The sale id of an NFT.
pub type NftSaleId = String;

/// The number of tokens that are minted. Always needs to be a multiple of
/// GLDT_PRICE_RATIO (100) * GLDT_SUBDIVIDABLE_BY (10**8)
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Default)]
pub struct GldtNumTokens {
    value: NumTokens,
}

impl GldtNumTokens {
    pub fn new(initial_value: NumTokens) -> Result<Self, String> {
        if !Self::is_valid(initial_value.clone()) {
            return Err(format!("Invalid initial value for GldtNumTokens: {}", initial_value));
        }
        Ok(GldtNumTokens {
            value: initial_value,
        })
    }

    pub fn new_from_weight(weight: NftWeight) -> Result<Self, String> {
        let value = Nat::from(weight) * (GLDT_PRICE_RATIO as u64) * GLDT_SUBDIVIDABLE_BY;
        Self::new(value)
    }

    pub fn get(&self) -> NumTokens {
        self.value.clone()
    }

    fn is_valid(val: NumTokens) -> bool {
        val % (GLDT_SUBDIVIDABLE_BY * (GLDT_PRICE_RATIO as u64)) == 0
    }
}

/// The token specifications for GLDT
pub struct GldtTokenSpec {
    id: Option<Nat>,
    fee: Option<Nat>,
    decimals: Nat,
    canister: Principal,
    standard: ICTokenSpec_standard,
    symbol: String,
}

impl GldtTokenSpec {
    pub fn new(canister_id_ledger: Principal) -> Self {
        GldtTokenSpec {
            id: None,
            fee: Some(Nat::from(GLDT_TX_FEE)),
            decimals: Nat::from(GLDT_DECIMALS),
            canister: canister_id_ledger,
            standard: ICTokenSpec_standard::ICRC1,
            symbol: String::from("GLDT"),
        }
    }

    pub fn get(&self) -> TokenSpec {
        TokenSpec::ic(ICTokenSpec {
            id: self.id.clone(),
            fee: self.fee.clone(),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        })
    }
}

pub fn calculate_tokens_from_weight(grams: NftWeight) -> Result<GldtNumTokens, String> {
    GldtNumTokens::new(Nat::from((grams as u64) * (GLDT_PRICE_RATIO as u64) * GLDT_SUBDIVIDABLE_BY))
}
