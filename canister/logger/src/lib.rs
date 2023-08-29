use ic_cdk::{api, storage};
use ic_cdk_macros::{export_candid, query, update};

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    canistergeek_ic_rust::logger::log_message(format!("executing pre_upgrade"));

    // canister geek data
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();

    storage::stable_save((monitor_stable_data, logger_stable_data)).unwrap();
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    canistergeek_ic_rust::logger::log_message(format!("executing post_upgrade"));

    let stable_data: Result<
        (
            canistergeek_ic_rust::monitor::PostUpgradeStableData,
            canistergeek_ic_rust::logger::PostUpgradeStableData,
        ),
        String,
    > = storage::stable_restore();
    match stable_data {
        Ok((monitor_stable_data, logger_stable_data)) => {
            canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
            canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
        }
        Err(_) => {}
    }
}

#[update]
fn log_message(message: String) {
    let caller = api::caller();
    validate_caller();
    canistergeek_ic_rust::monitor::collect_metrics();
    canistergeek_ic_rust::logger::log_message(format!("{} :: {}", caller, message));
}

#[query(name = "getCanistergeekInformation")]
async fn get_canistergeek_information(
    request: canistergeek_ic_rust::api_type::GetInformationRequest,
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    canistergeek_ic_rust::get_information(request)
}

fn validate_caller() -> () {}

export_candid!();
