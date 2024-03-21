use ic_cdk_macros::query;

use crate::{ model::payment_processor::PaymentRound, state::read_state };

// no real use for this, mainly for testing. Remove later
#[query]
fn get_payment_rounds() -> Vec<(u16, PaymentRound)> {
    read_state(|state| { state.data.payment_processor.get_payment_rounds() })
}
