use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Clone, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub struct TokenSymbol(String);

#[derive(Debug)]
pub enum TokenSymbolParseError {
    InvalidTokenSymbol,
}

impl TokenSymbol {
    pub fn parse(symbol: &str) -> Result<TokenSymbol, TokenSymbolParseError> {
        let allowed_tokens = ["ICP", "OGY", "GLDGov"];
        let valid_token = allowed_tokens.contains(&symbol);
        if valid_token {
            Ok(TokenSymbol(symbol.to_string()))
        } else {
            Err(TokenSymbolParseError::InvalidTokenSymbol)
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub struct TokenInfo {
    pub ledger_id: Principal,
    pub fee: u64,
    pub decimals: u64,
}
