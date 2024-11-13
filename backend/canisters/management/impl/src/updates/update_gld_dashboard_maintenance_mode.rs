use crate::guards::caller_is_authorized;
use ic_cdk::update;
pub use management_api_canister::update_gld_dashboard_maintenance_mode::{
    Args as UpdateGLDDashboardMaintenanceModeArgs,
    Response as UpdateGLDDashboardMaintenanceModeResponse,
};

use crate::state::mutate_state;

#[update(guard = "caller_is_authorized")]
async fn update_gld_dashboard_maintenance_mode(
    value: UpdateGLDDashboardMaintenanceModeArgs,
) -> UpdateGLDDashboardMaintenanceModeResponse {
    mutate_state(|s| {
        s.data.gld_dashbaord_maintenance_mode = value;
    })
}
