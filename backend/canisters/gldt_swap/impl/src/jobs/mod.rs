mod manage_archive_cycles;
mod manage_archives;
mod manage_gldt_supply;
mod manage_ogy_fee_accounts;
mod manage_service_status;
mod manage_stale_swaps;
mod transfer_gldt_fees;

pub(crate) fn start() {
    manage_archives::start_job();
    manage_stale_swaps::start_job();
    manage_archive_cycles::start_job();
    manage_service_status::start_job();
    manage_ogy_fee_accounts::start_job();
    manage_gldt_supply::start_job();
    transfer_gldt_fees::start_job();
}
