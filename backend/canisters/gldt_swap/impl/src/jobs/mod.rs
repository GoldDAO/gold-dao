// mod transfer_fees;
mod manage_archive_cycles;
mod manage_archives;
mod manage_ogy_swap_fees;
mod manage_service_status;
mod manage_stale_swaps;

pub(crate) fn start() {
    // transfer_fees::start_job();
    manage_archives::start_job();
    manage_stale_swaps::start_job();
    manage_archive_cycles::start_job();
    manage_service_status::start_job();
    manage_ogy_swap_fees::start_job();
}
