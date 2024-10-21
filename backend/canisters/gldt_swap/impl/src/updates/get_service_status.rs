pub use gldt_swap_api_canister::get_service_status::{
    Args as GetServiceStatusArgs,
    Response as GetServiceStatusResponse,
};
use ic_cdk::update;

use crate::service_status::check_service_status;

async fn get_service_status(_: GetServiceStatusArgs) -> GetServiceStatusResponse {
    check_service_status().await
}
