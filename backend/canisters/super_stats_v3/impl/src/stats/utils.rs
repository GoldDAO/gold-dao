use super_stats_v3_api::custom_types::{ ProcessedTX, SmallTX };

pub fn remove_none_ptx_values(vec: Vec<Option<ProcessedTX>>) -> Vec<ProcessedTX> {
    let mut ret: Vec<ProcessedTX> = Vec::new();
    for tx in vec {
        match tx {
            Some(v) => {
                ret.push(v);
            }
            None => {}
        }
    }
    return ret;
}

pub fn remove_none_stx_values(vec: Vec<Option<SmallTX>>) -> Vec<SmallTX> {
    let mut ret: Vec<SmallTX> = Vec::new();
    for tx in vec {
        match tx {
            Some(v) => {
                ret.push(v);
            }
            None => {}
        }
    }
    return ret;
}

// used for setting specific timers.
pub fn nearest_past_hour(time_nano: u64) -> u64 {
    const NANO_PER_HOUR: u64 = 3600_000_000_000;
    let remainder = time_nano % NANO_PER_HOUR;
    let nearest_hour = time_nano - remainder;
    return nearest_hour;
}

pub fn nearest_day_start(time_nano: u64) -> u64 {
    const NANO_PER_DAY: u64 = 86_400_000_000_000;
    let remainder = time_nano % NANO_PER_DAY;
    let nearest_day_start = time_nano - remainder;
    return nearest_day_start;
}

pub fn principal_subaccount_to_string(principal: String, subaccount: String) -> String {
    return format!("{}.{}", principal, subaccount);
}

pub fn parse_icrc_account(input: &String) -> Option<(String, String)> {
    // -> Option<(principal, subaccount)>
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}
#[cfg(target_arch = "wasm32")]
pub fn timestamp_nanos() -> u64 {
    ic_cdk::api::time()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp_nanos() -> u64 {
    use std::time::SystemTime;

    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64
}
