use candid::Principal;
use canister_time::DAY_IN_MS;
use pocket_ic::PocketIc;
use rand::{ RngCore, thread_rng };
use types::TimestampMillis;

pub fn random_principal() -> Principal {
    let mut bytes = [0u8; 29];
    thread_rng().fill_bytes(&mut bytes);
    Principal::from_slice(&bytes)
}

pub fn tick_n_blocks(pic: &PocketIc, times: u32) {
    for _ in 0..times {
        pic.tick();
    }
}

pub fn is_interval_more_than_7_days(
    previous_time: TimestampMillis,
    now_time: TimestampMillis
) -> bool {
    // convert the milliseconds to the number of days since UNIX Epoch.
    // integer division means partial days will be truncated down or effectively rounded down. e.g 245.5 becomes 245
    let previous_in_days = previous_time / DAY_IN_MS;
    let current_in_days = now_time / DAY_IN_MS;
    // never allow distributions to happen twice i.e if the last run distribution in days since UNIX epoch is the same as the current time in days since the last UNIX Epoch then return early.
    current_in_days >= previous_in_days + 7
}
