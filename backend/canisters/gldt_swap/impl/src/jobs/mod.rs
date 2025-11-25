pub mod manage_token_supply;
pub mod transfer_fees;

pub(crate) fn start() {
    manage_token_supply::start_job();
    transfer_fees::start_job();
}
