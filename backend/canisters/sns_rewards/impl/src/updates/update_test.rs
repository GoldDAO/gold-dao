use ic_cdk_macros::update;
use tracing::info;

#[update]
fn update_info() -> String {
    info!("This is a test.");
    "This is a test.".to_string()
}
