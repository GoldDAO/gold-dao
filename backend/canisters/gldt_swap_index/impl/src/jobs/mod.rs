pub mod cleanup_cache;
pub mod update_index;

pub(crate) fn start() {
    cleanup_cache::start_job();
    update_index::start_job();
}
