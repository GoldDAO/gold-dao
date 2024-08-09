use ic_cdk::update;
use ic_cdk_timers::TimerId;
use super_stats_v3_api::runtime::RUNTIME_STATE;
use crate::{core::utils::log, timers::state::TIMER_STATE};

#[update]
fn stop_all_timers() -> String {
    // check admin
    RUNTIME_STATE.with(|s|{s.borrow().data.check_admin(ic_cdk::caller().to_text())});

    // clear timers
    TIMER_STATE.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });

    //update working stats
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.update_timer(false)
    });   

    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}