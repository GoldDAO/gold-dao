use super_stats_v3_api::{ core::constants::D1_AS_NANOS, stable_memory::STABLE_STATE };

pub fn get_count_of_unique_accounts(stx_ref_vec: Vec<Option<u64>>) -> u64 {
    if stx_ref_vec.len() == 0 {
        return 0_u64;
    }
    let mut u64_values: Vec<u64> = Vec::new();
    // Remove none values and get u64
    for stx_ref in stx_ref_vec {
        match stx_ref {
            None => {}
            Some(v) => {
                u64_values.push(v);
            }
        }
    }
    // sort and get unique values
    u64_values.sort();
    let mut count: u64 = 1; // first index is counted as loop starts at 1 not 0
    for i in 1..u64_values.len() {
        if u64_values[i] != u64_values[i - 1] {
            count += 1;
        }
    }
    count
}

pub fn push_activity_snapshot() -> (u64, u64) {
    let accounts_count = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().account_data.accounts.len()
    });
    let principals_count = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().principal_data.accounts.len()
    });
    let new_times = STABLE_STATE.with(|s| {
        s.borrow_mut().as_mut().unwrap().activity_stats.take_activity_snapshot(accounts_count, principals_count)
    });
    new_times
}

pub fn push_padding_snapshot(start: u64, end: u64) -> (u64, u64) {
    let accounts_count = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().account_data.accounts.len()
    });
    let principals_count = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().principal_data.accounts.len()
    });
    let new_times = STABLE_STATE.with(|s| {
        s.borrow_mut().as_mut().unwrap().activity_stats.take_activity_snapshot(accounts_count, principals_count)
    });
    new_times
}

pub fn nearest_past_day_start(time_nano: u64) -> u64 {
    let remainder = time_nano % D1_AS_NANOS;
    let nearest_day_start = time_nano - remainder;
    return nearest_day_start;
}

pub fn next_midnight_time(time_now: u64) -> u64 {
    let last_midnight: u64 = nearest_past_day_start(time_now);
    let next_midnight: u64 = last_midnight + D1_AS_NANOS;
    return next_midnight;
}

pub fn init_activity_stats(first_block_time: u64) -> u64 {
    let end_time = next_midnight_time(first_block_time.clone());
    STABLE_STATE.with(|s| {
        s.borrow_mut().as_mut().unwrap().activity_stats.init(first_block_time, end_time.clone())
    });
    end_time
}
