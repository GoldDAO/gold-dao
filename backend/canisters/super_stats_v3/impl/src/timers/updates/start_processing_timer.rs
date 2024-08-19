use ic_cdk::update;
use super_stats_v3_api::runtime::RUNTIME_STATE;
use crate::{core::utils::log, timers::timers::start_processing_time_impl};

#[update]
fn start_processing_timer(secs: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    let ret;

    // check target canister set
    let init_done = RUNTIME_STATE.with(|s|{s.borrow().data.target_ledger_locked});
    if init_done == false {
        return String::from("Cannot start timer - No target ledger set. Use init method before starting timer")
    }

    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
     if is_running == true {
        ret = String::from("Processing timer is already running");
    } else { 
        start_processing_time_impl(secs);
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.update_timer(true)
        });
        ret = String::from("Processing timer has been started");
        log("[][] ---- Starting Processing Timer ---- [][]");
    }
    return ret;
}
