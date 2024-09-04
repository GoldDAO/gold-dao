pub mod burn_tokens;
pub mod swap_tokens;
pub mod init_swap_clinets;

pub(crate) fn start() {
    init_swap_clinets::start_job();
    swap_tokens::start_job();
    // burn_tokens::start_job();
}
