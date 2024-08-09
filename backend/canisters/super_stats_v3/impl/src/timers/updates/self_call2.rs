use ic_cdk::update;
use super_stats_v3_api::runtime::RUNTIME_STATE;
use crate::timers::timers::process_self_call2;

#[update]
async fn self_call2(){
    let self_id = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let caller_string = ic_cdk::caller().to_text();
    if caller_string != self_id {
        ic_cdk::trap("This method can only be called by this canister (self-call)");
    }
    process_self_call2().await
}