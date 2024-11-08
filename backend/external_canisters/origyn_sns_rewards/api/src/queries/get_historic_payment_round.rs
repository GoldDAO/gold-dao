use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TokenSymbol;

use crate::payment_round::PaymentRound;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token: TokenSymbol,
    pub round_id: u16,
}

pub type Response = Vec<(u16, PaymentRound)>;
