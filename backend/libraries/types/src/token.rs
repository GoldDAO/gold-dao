use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};

use candid::Decode;
use serde::{Deserialize, Serialize};

/// Compact enum for token identity (for keys, matching, etc.)
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    CandidType,
    minicbor::Encode,
    minicbor::Decode,
)]
pub enum TokenSymbol {
    #[n(0)]
    ICP,
    #[n(1)]
    OGY,
    #[n(2)]
    GOLDAO,
    #[n(3)]
    WTN,
    #[n(4)]
    GLDT,
}

impl std::fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            TokenSymbol::ICP => "ICP",
            TokenSymbol::OGY => "OGY",
            TokenSymbol::GOLDAO => "GOLDAO",
            TokenSymbol::WTN => "WTN",
            TokenSymbol::GLDT => "GLDT",
        };
        write!(f, "{symbol}")
    }
}

impl TokenSymbol {
    /// Return the display symbol for a token (can be renamed here)
    pub fn symbol(&self) -> &'static str {
        match self {
            TokenSymbol::ICP => "ICP",
            TokenSymbol::OGY => "OGY",
            TokenSymbol::GOLDAO => "GOLDAO",
            TokenSymbol::WTN => "WTN",
            TokenSymbol::GLDT => "GLDT",
        }
    }

    pub fn parse(symbol: &str) -> Result<Self, TokenSymbolParseError> {
        match symbol {
            "ICP" => Ok(TokenSymbol::ICP),
            "OGY" => Ok(TokenSymbol::OGY),
            "GOLDAO" | "GLDGov" => Ok(TokenSymbol::GOLDAO),
            "WTN" => Ok(TokenSymbol::WTN),
            _ => Err(TokenSymbolParseError::InvalidTokenSymbol),
        }
    }

    pub fn get_prod_token_info(self) -> TokenInfo {
        TokenInfo {
            ledger_id: self.ledger_id(false),
            fee: match self {
                TokenSymbol::ICP => 10_000,
                TokenSymbol::OGY => 200_000,
                TokenSymbol::GOLDAO => 100_000,
                TokenSymbol::WTN => 1_000_000,
                TokenSymbol::GLDT => 10_000_000,
            },
            decimals: 8,
        }
    }

    pub fn get_token_info(self, is_test_mode: bool) -> TokenInfo {
        TokenInfo {
            ledger_id: self.ledger_id(is_test_mode),
            fee: match self {
                TokenSymbol::ICP => 10_000,
                TokenSymbol::OGY => 200_000,
                TokenSymbol::GOLDAO => 100_000,
                TokenSymbol::WTN => 1_000_000,
                TokenSymbol::GLDT => 10_000_000,
            },
            decimals: 8,
        }
    }

    pub fn ledger_id(&self, test_mode: bool) -> Principal {
        match (self, test_mode) {
            (TokenSymbol::ICP, false) => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")
                .expect("Invalid ICP ledger principal"),
            (TokenSymbol::ICP, true) => Principal::from_text("te3q-rqaaa-aaaal-qdlva-cai")
                .expect("Invalid test ICP ledger principal"),

            (TokenSymbol::OGY, false) => Principal::from_text("lkwrt-vyaaa-aaaaq-aadhq-cai")
                .expect("Invalid OGY ledger principal"),
            (TokenSymbol::OGY, true) => Principal::from_text("j5naj-nqaaa-aaaal-ajc7q-cai")
                .expect("Invalid test OGY ledger principal"),

            (TokenSymbol::GOLDAO, false) => Principal::from_text("tyyy3-4aaaa-aaaaq-aab7a-cai")
                .expect("Invalid GLDGov ledger principal"),
            (TokenSymbol::GOLDAO, true) => Principal::from_text("irhm6-5yaaa-aaaap-ab24q-cai")
                .expect("Invalid test GLDGov ledger principal"),

            (TokenSymbol::WTN, false) => Principal::from_text("jcmow-hyaaa-aaaaq-aadlq-cai")
                .expect("Invalid WTN ledger principal"),
            (TokenSymbol::WTN, true) => Principal::from_text("jcmow-hyaaa-aaaaq-aadlq-cai")
                .expect("Invalid test WTN ledger principal"),

            (TokenSymbol::GLDT, false) => Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai")
                .expect("Invalid GLDT ledger principal"),
            (TokenSymbol::GLDT, true) => Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai")
                .expect("Invalid test GLDT ledger principal"),
        }
    }

    pub fn is_valid(symbol: &str) -> bool {
        TokenSymbol::parse(symbol).is_ok()
    }
}

#[macro_export]
macro_rules! ledger_id {
    ($symbol:ident) => {{
        let is_test_mode = crate::state::read_state(|s| s.env.is_test_mode());
        types::TokenSymbol::$symbol.ledger_id(is_test_mode)
    }};
}

#[macro_export]
macro_rules! token_info {
    ($symbol:ident) => {{
        let is_test_mode = crate::state::read_state(|s| s.env.is_test_mode());
        types::TokenSymbol::$symbol.get_token_info(is_test_mode)
    }};
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenSymbolParseError {
    InvalidTokenSymbol,
}

impl Display for TokenSymbolParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTokenSymbol => write!(f, "InvalidTokenSymbol"),
        }
    }
}

const MAX_VALUE_SIZE: u32 = 20;

impl Storable for TokenSymbol {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        minicbor::encode(self, &mut buf).expect("token symbol encoding should always succeed");
        Cow::Owned(buf)
    }

    // fn into_bytes(self) -> std::vec::Vec<u8> {
    //     let mut buf = vec![];
    //     minicbor::encode(self, &mut buf).expect("token symbol encoding should always succeed");
    //     buf
    // }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        minicbor::decode(bytes.as_ref()).unwrap_or_else(|e| {
            panic!(
                "failed to decode token symbol bytes {}: {e}",
                hex::encode(bytes)
            )
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Debug, Serialize, Clone, Deserialize, CandidType, PartialEq, Eq, Hash, Copy)]
pub struct TokenInfo {
    pub ledger_id: Principal,
    pub fee: u64,
    pub decimals: u64,
}

impl TokenInfo {
    pub fn validate(self) -> Result<(), String> {
        if self.ledger_id == Principal::anonymous() {
            return Err("Invalid ledger_id: cannot be anonymous".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::token::MAX_VALUE_SIZE;
    use crate::TokenSymbol;
    use ic_stable_structures::Storable;

    #[test]
    fn test_token_symbol_encode_decode() {
        let symbols = [
            TokenSymbol::ICP,
            TokenSymbol::OGY,
            TokenSymbol::GOLDAO,
            TokenSymbol::WTN,
        ];
        for &symbol in &symbols {
            let mut buf = Vec::new();
            minicbor::encode(&symbol, &mut buf).unwrap();
            println!("Encoded {:?} to bytes: {:?}", symbol, buf);

            let decoded: TokenSymbol = minicbor::decode(&buf).unwrap();
            assert_eq!(decoded, symbol);
        }
    }

    #[test]
    fn test_token_symbol_encoded_size_within_limit() {
        let symbols = [
            TokenSymbol::ICP,
            TokenSymbol::OGY,
            TokenSymbol::GOLDAO,
            TokenSymbol::WTN,
        ];
        for &symbol in &symbols {
            let encoded = symbol.to_bytes();
            let size = encoded.len() as u32;
            println!("Encoded size for {:?}: {}", symbol, size);
            assert!(
                size <= MAX_VALUE_SIZE,
                "Encoded size of {:?} ({}) exceeds MAX_VALUE_SIZE ({})",
                symbol,
                size,
                MAX_VALUE_SIZE
            );
        }
    }

    #[test]
    fn test_token_symbol_parse() {
        use crate::TokenSymbol;

        // Valid symbols
        assert_eq!(TokenSymbol::parse("ICP"), Ok(TokenSymbol::ICP));
        assert_eq!(TokenSymbol::parse("OGY"), Ok(TokenSymbol::OGY));
        assert_eq!(TokenSymbol::parse("GOLDAO"), Ok(TokenSymbol::GOLDAO));
        assert_eq!(TokenSymbol::parse("GLDGov"), Ok(TokenSymbol::GOLDAO)); // alias
        assert_eq!(TokenSymbol::parse("WTN"), Ok(TokenSymbol::WTN));

        // Invalid symbols
        assert!(TokenSymbol::parse("icp").is_err()); // case-sensitive
        assert!(TokenSymbol::parse("goldao").is_err());
        assert!(TokenSymbol::parse("GLD").is_err());
        assert!(TokenSymbol::parse("").is_err());
        assert!(TokenSymbol::parse("UNKNOWN").is_err());
    }
}
