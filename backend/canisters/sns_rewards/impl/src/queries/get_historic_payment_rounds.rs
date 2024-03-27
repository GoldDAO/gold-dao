use ic_cdk_macros::query;
use types::TokenSymbol;

use crate::{ model::payment_processor::PaymentRound, state::read_state };

// no real use for this, mainly for testing. Remove later
#[query]
fn get_historic_payment_round(token: TokenSymbol, round_id: u16) -> Vec<(u16, PaymentRound)> {
    read_state(|state| { state.data.payment_processor.get_payment_round_history(token, round_id) })
}
