use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{TokenInfo, TokenSymbol};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token_list: Vec<(TokenSymbol, TokenInfo)>,
}

pub type Response = Result<String, String>;
