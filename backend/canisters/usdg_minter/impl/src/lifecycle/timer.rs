use crate::guard::TimerGuard;
use crate::lifecycle::tasks::schedule_after;
use crate::lifecycle::tasks::{pop_if_ready, TaskType};
use crate::logs::INFO;
use crate::transfer::process_pending_transfer;
use ic_canister_log::log;
use std::time::Duration;

pub fn setup_timers() {}

#[cfg(feature = "inttest")]
fn ok_or_die(result: Result<(), String>) {
    if let Err(msg) = result {
        ic_cdk::println!("{}", msg);
        ic_cdk::trap(&msg);
    }
}

/// Checks that the canister state is internally consistent.
#[cfg(feature = "inttest")]
fn check_invariants() -> Result<(), String> {
    crate::state::read_state(|s| {
        let recovered_state = crate::state::audit::replay_events();

        s.check_semantically_eq(&recovered_state)?;

        Ok(())
    })
}

pub fn check_postcondition<T>(t: T) -> T {
    #[cfg(feature = "inttest")]
    ok_or_die(check_invariants());
    t
}

#[export_name = "canister_global_timer"]
fn timer() {
    #[cfg(feature = "inttest")]
    ok_or_die(check_invariants());

    const DEFAULT_RETRY_DELAY: Duration = Duration::from_secs(5);

    if let Some(task) = pop_if_ready() {
        let task_type = task.task_type;
        match task_type {
            TaskType::ProcessLogic => ic_cdk::spawn(async {
                let _guard = match TimerGuard::new(task_type) {
                    Ok(guard) => guard,
                    Err(_) => {
                        log!(INFO, "[timer] Already processing ProcessLogic",);
                        return;
                    }
                };
            }),
            TaskType::FetchGoldPrice => {
                let _guard = match TimerGuard::new(task_type) {
                    Ok(guard) => guard,
                    Err(_) => {
                        log!(INFO, "[timer] Already processing FetchGoldPrice",);
                        return;
                    }
                };
            }
            TaskType::ProcessPendingTransfer => {
                ic_cdk::spawn(async {
                    let _guard = match TimerGuard::new(task_type) {
                        Ok(guard) => guard,
                        Err(_) => {
                            log!(INFO, "[timer] Already processing ProcessPendingTransfer",);
                            return;
                        }
                    };

                    if process_pending_transfer().await > 0 {
                        schedule_after(DEFAULT_RETRY_DELAY, TaskType::ProcessPendingTransfer);
                    }
                });
            }
        }
    }
}
