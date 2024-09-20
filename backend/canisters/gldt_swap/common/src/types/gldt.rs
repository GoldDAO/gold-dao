use candid::{ CandidType, Nat, Principal };
use origyn_nft_reference::origyn_nft_reference_canister::{
    IcTokenSpec,
    IcTokenSpecStandard,
    TokenSpec,
};
use serde::{ Deserialize, Serialize };

use crate::nft::NftWeight;

pub const GLDT_SUBDIVIDABLE_BY: u64 = 100_000_000;
pub const GLDT_DECIMALS: u8 = 8;
pub const GLDT_PRICE_RATIO: u8 = 100;
pub const GLDT_TX_FEE: u64 = 1_000_000u64;
pub const GLDT_SWAP_FEE_ACCOUNT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
];
pub const GLDT_LEDGER_FEE_ACCOUNT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Default, Eq)]
pub struct GldtNumTokens {
    value: Nat,
    value_with_fee: Nat,
}

impl GldtNumTokens {
    pub fn new(initial_value: Nat) -> Result<Self, String> {
        if !Self::is_valid(initial_value.clone()) {
            return Err(format!("Invalid initial value for GldtNumTokens: {initial_value}"));
        }
        let with_fee = initial_value.clone() + Nat::from(GLDT_TX_FEE * 2);

        Ok(GldtNumTokens {
            value: initial_value,
            value_with_fee: with_fee,
        })
    }

    pub fn new_from_weight(weight: NftWeight) -> Result<Self, String> {
        let value = Nat::from(weight) * Nat::from(GLDT_PRICE_RATIO) * GLDT_SUBDIVIDABLE_BY;
        Self::new(value)
    }

    pub fn get(&self) -> Nat {
        self.value.clone()
    }

    pub fn get_with_fee(&self) -> Nat {
        self.value_with_fee.clone()
    }

    fn is_valid(val: Nat) -> bool {
        val % (GLDT_SUBDIVIDABLE_BY * Nat::from(GLDT_PRICE_RATIO)) == 0u8
    }

    pub fn invalid() -> Self {
        Self {
            value: Nat::from(0u64),
            value_with_fee: Nat::from(0u64),
        }
    }
}

pub struct GldtTokenSpec {
    id: Option<Nat>,
    pub fee: Option<Nat>,
    decimals: Nat,
    canister: Principal,
    standard: IcTokenSpecStandard,
    symbol: String,
}
impl GldtTokenSpec {
    pub fn new(canister_id_ledger: Principal) -> Self {
        GldtTokenSpec {
            id: None,
            fee: Some(Nat::from(GLDT_TX_FEE)),
            decimals: Nat::from(GLDT_DECIMALS),
            canister: canister_id_ledger,
            standard: IcTokenSpecStandard::Ledger,
            symbol: String::from("GLDT"),
        }
    }

    pub fn get_token_spec(&self) -> TokenSpec {
        TokenSpec::Ic(IcTokenSpec {
            id: self.id.clone(),
            fee: self.fee.clone(),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        })
    }
    pub fn get_token_spec_with_no_fee(&self) -> TokenSpec {
        TokenSpec::Ic(IcTokenSpec {
            id: self.id.clone(),
            fee: Some(Nat::from(0u64)),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        })
    }

    pub fn get_ic_token_spec(&self) -> IcTokenSpec {
        IcTokenSpec {
            id: self.id.clone(),
            fee: self.fee.clone(),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        }
    }
}

pub struct OGYTokenSpec {
    id: Option<Nat>,
    pub fee: Option<Nat>,
    decimals: Nat,
    canister: Principal,
    standard: IcTokenSpecStandard,
    symbol: String,
}
impl OGYTokenSpec {
    pub fn new(canister_id_ledger: Principal) -> Self {
        OGYTokenSpec {
            id: None,
            fee: Some(Nat::from(200_000u64)),
            decimals: Nat::from(8u64),
            canister: canister_id_ledger,
            standard: IcTokenSpecStandard::Ledger,
            symbol: String::from("OGY"),
        }
    }

    pub fn get_token_spec(&self) -> TokenSpec {
        TokenSpec::Ic(IcTokenSpec {
            id: self.id.clone(),
            fee: self.fee.clone(),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        })
    }
    pub fn get_token_spec_with_no_fee(&self) -> TokenSpec {
        TokenSpec::Ic(IcTokenSpec {
            id: self.id.clone(),
            fee: Some(Nat::from(0u64)),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        })
    }

    pub fn get_ic_token_spec(&self) -> IcTokenSpec {
        IcTokenSpec {
            id: self.id.clone(),
            fee: self.fee.clone(),
            decimals: self.decimals.clone(),
            canister: self.canister,
            standard: self.standard.clone(),
            symbol: self.symbol.clone(),
        }
    }
}
