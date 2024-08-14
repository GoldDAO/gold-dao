pub mod burn_tokens;
pub mod swap_tokens;

pub(crate) fn start() {
    burn_tokens::start_job();
    swap_tokens::start_job();
}
