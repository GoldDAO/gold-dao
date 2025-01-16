use crate::guard::TimerGuard;
use crate::lifecycle::tasks::{pop_if_ready, schedule_after, schedule_now, TaskType};
use crate::logs::INFO;
use crate::state::audit::process_event;
use crate::state::event::EventType;
use crate::state::mutate_state;
use crate::transfer::process_pending_transfer;
use crate::vault::check_vaults;
use crate::xrc::fetch_gold_price;
use ic_canister_log::log;
use scopeguard::guard;
use std::time::Duration;

pub fn setup_timers() {
    schedule_after(Duration::from_secs(24 * 60 * 60), TaskType::ChargeFees);
    schedule_after(Duration::from_secs(60), TaskType::ProcessLogic);
    #[cfg(not(feature = "inttest"))]
    schedule_after(Duration::from_secs(60), TaskType::FetchGoldPrice);
}

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

                let _enqueue_followup_guard = guard((), |_| {
                    log!(INFO, "[timer] ProcessLogic panicked",);
                    schedule_after(Duration::from_secs(10), TaskType::ProcessLogic);
                });

                mutate_state(|s| check_vaults(s));
                schedule_after(Duration::from_secs(60), TaskType::FetchGoldPrice);
                scopeguard::ScopeGuard::into_inner(_enqueue_followup_guard);
            }),
            TaskType::FetchGoldPrice => {
                ic_cdk::spawn(async {
                    let _guard = match TimerGuard::new(task_type) {
                        Ok(guard) => guard,
                        Err(_) => {
                            log!(INFO, "[timer] Already processing FetchGoldPrice",);
                            return;
                        }
                    };

                    if fetch_gold_price().await {
                        schedule_after(Duration::from_secs(5 * 60), TaskType::FetchGoldPrice);
                        schedule_now(TaskType::ProcessLogic);
                    } else {
                        schedule_after(DEFAULT_RETRY_DELAY, TaskType::FetchGoldPrice);
                    }
                });
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
            TaskType::ChargeFees => {
                ic_cdk::spawn(async {
                    let _guard = match TimerGuard::new(task_type) {
                        Ok(guard) => guard,
                        Err(_) => {
                            log!(INFO, "[timer] Already processing ChargeFees",);
                            return;
                        }
                    };

                    mutate_state(|s| process_event(s, EventType::ChargeFee));
                    schedule_after(Duration::from_secs(24 * 60 * 60), TaskType::ChargeFees);
                });
            }
        }
    }
}
