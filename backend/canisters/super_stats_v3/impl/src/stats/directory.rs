use super_stats_v3_api::stable_memory::STABLE_STATE;

pub fn add_to_directory(account: &String) -> Option<u64> {
    STABLE_STATE.with(|s| {
        s.borrow_mut().as_mut().unwrap().directory_data.add_id(account.clone())
    })
}

pub fn lookup_directory(ref_id: u64) -> Option<String> {
    STABLE_STATE.with(|s| {
        s.borrow_mut().as_mut().unwrap().directory_data.get_id(&ref_id)
    })
}