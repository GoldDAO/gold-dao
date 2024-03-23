// use sns_governance_canister::types::ListNeuronsResponse;
use crate::jobs::distribute_rewards::run_retry_distribution;
use ic_cdk::update;

// Only for development, remove after
#[update]
async fn retry_failed_payment_rounds() {
    run_retry_distribution()
}
