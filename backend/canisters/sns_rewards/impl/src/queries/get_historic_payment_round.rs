use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_historic_payment_round::{
    Args as GetHistoricPaymentRoundsArgs,
    Response as GetHistoricPaymentRoundsResponse,
};

use crate::state::read_state;

#[query(hidden = true)]
fn get_historic_payment_round(
    args: GetHistoricPaymentRoundsArgs
) -> GetHistoricPaymentRoundsResponse {
    read_state(|state| {
        state.data.payment_processor.get_payment_round_history(args.token, args.round_id)
    })
}
