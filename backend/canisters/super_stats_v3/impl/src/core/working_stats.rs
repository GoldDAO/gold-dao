use super_stats_v3_api::runtime::RUNTIME_STATE;

pub fn count_error(){
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_errors()
    })
}

pub fn api_count(){
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_api()
    })
}