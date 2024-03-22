pub mod update_goldprice;
pub mod update_goldnft_data;

pub(crate) fn start() {
    update_goldprice::start_job();
}
