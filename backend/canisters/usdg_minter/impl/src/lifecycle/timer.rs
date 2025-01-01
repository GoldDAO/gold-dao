use crate::guard::TimerGuard;
use crate::lifecycle::tasks::schedule_after;
use crate::lifecycle::tasks::{pop_if_ready, TaskType};
use crate::logs::INFO;
use crate::transfer::process_pending_transfer;
use ic_canister_log::log;
use std::time::Duration;

#[export_name = "canister_global_timer"]
fn timer() {
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
