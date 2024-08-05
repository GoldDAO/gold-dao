pub mod check_journal;
pub mod create_proposal;
pub mod swap_tokens;

pub(crate) fn start() {
    create_proposal::start_job();
    check_journal::start_job();
}
