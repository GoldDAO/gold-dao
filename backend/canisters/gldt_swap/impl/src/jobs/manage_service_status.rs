/*!

- checks and sets the service status

*/

use crate::service_status::check_service_status;
use canister_time::{run_now_then_interval, MINUTE_IN_MS};
use std::time::Duration;
use types::Milliseconds;

const MANAGE_SERVICE_STATUS_INTERVAL: Milliseconds = MINUTE_IN_MS; // 1000 years

pub fn start_job() {
    run_now_then_interval(
        Duration::from_millis(MANAGE_SERVICE_STATUS_INTERVAL),
        spawn_transfer_job,
    );
}

pub fn spawn_transfer_job() {
    ic_cdk::spawn(manage_service_status())
}

async fn manage_service_status() {
    check_service_status().await;
}
