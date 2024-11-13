// use std::time::Duration;

#[export_name = "canister_global_timer"]
fn timer() {
    use crate::tasks::{pop_if_ready, TaskType};

    // const INTERVAL_PROCESSING: Duration = Duration::from_secs(5);

    let task = match pop_if_ready() {
        Some(task) => task,
        None => return,
    };

    match task.task_type {
        TaskType::ProcessLogic => todo!(),
        TaskType::FetchGoldPrice => todo!(),
    }
}
