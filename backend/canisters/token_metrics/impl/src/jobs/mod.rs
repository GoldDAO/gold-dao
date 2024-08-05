pub mod sync_governance;
pub mod sync_supply_data;
pub mod update_goldnft_data;
pub mod update_goldprice;

pub(crate) fn start() {
    update_goldprice::start_job();
    update_goldnft_data::start_job();
    sync_governance::start_job();
}
