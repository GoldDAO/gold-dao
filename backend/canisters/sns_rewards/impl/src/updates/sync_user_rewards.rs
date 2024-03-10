// use sns_governance_canister::types::ListNeuronsResponse;
use crate::jobs::distribute_rewards::run;
use ic_cdk::update;

// Only for development, remove after
#[update]
async fn sync_user_rewards() {
    run()
}
