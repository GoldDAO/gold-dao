use ic_cdk_timers::TimerId;
use std::cell::RefCell;

thread_local! {
    pub static TIMER_STATE: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}
