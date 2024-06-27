use ic_cdk::update;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::updates::force_payment_round_to_fail::{
    Args as ForcePaymentRoundToFailArgs,
    Response as ForcePaymentRoundToFailResponse,
};

use sns_rewards_api_canister::payment_round::PaymentStatus;

use crate::{
    guards::caller_is_governance_principal,
    jobs::distribute_rewards::create_new_payment_rounds,
    state::mutate_state,
};

// only to be used for integration testing
#[cfg(feature = "inttest")]
#[update(guard = "caller_is_governance_principal", hidden = true)]
pub async fn force_payment_round_to_fail(
    args: ForcePaymentRoundToFailArgs
) -> ForcePaymentRoundToFailResponse {
    _force_payment_round_to_fail_impl(args).await
}

async fn _force_payment_round_to_fail_impl(neurons: Vec<NeuronId>) {
    create_new_payment_rounds().await;
    mutate_state(|s| {
        let rounds = s.data.payment_processor.get_active_rounds();
        for payment_round in rounds {
            let symbol = payment_round.token;
            for neuron_id in neurons.clone() {
                s.data.payment_processor.set_active_payment_status(
                    &symbol,
                    &neuron_id,
                    PaymentStatus::Failed("Fake testing failure".to_string())
                );
            }
        }
    });
}
