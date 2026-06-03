pub mod burn_tokens;
pub mod stake_icp;
pub mod swap_tokens;

pub(crate) fn start() {
    swap_tokens::start_job();
    burn_tokens::start_job();
}
