use crate::state::{mutate_state, read_state};
use candid::CandidType;
use gldt_stake_common::{
    stake_position::{StakePosition, StakePositionId},
    stake_position_event::UnstakeState,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use time::Weekday;
use tracing::debug;
use types::TimestampMillis;
use utils::env::Environment;

pub fn trace(msg: &str) {
    unsafe {
        ic0::debug_print(msg.as_ptr() as i32, msg.len() as i32);
    }
}

pub async fn commit_changes() {
    let this_canister_id = read_state(|s| s.env.canister_id());
    let _ = ic_cdk::call::<(), ()>(this_canister_id, "commit", ()).await;
}

// specifies a range that the reward interval can occur. e.g on a certain weekday and between a start hour and end hour
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TimeInterval {
    /// weekday - e.g  Monday, Tuesday, Wednesday = 2, Thursday = 3, Friday = 4, Saturday = 5, Sunday = 6
    pub weekday: Option<String>,
    /// 24 hour clock - 0 = 00, 14 = 14:00
    pub start_hour: u8,
    /// 24 hour clock - 0 = 00, 14 = 14:00
    pub end_hour: u8,
}

impl Default for TimeInterval {
    fn default() -> Self {
        Self {
            weekday: Some("Wednesday".to_string()),
            start_hour: 14, // 2pm
            end_hour: 16,   // 4pm
        }
    }
}

impl TimeInterval {
    pub fn is_within_weekly_interval(&self, timestamp_millis: TimestampMillis) -> bool {
        let timestamp_secs = timestamp_millis / 1000; // Convert milliseconds to seconds
                                                      // Create a DateTime equivalent using time crate
        let timestamp = match time::OffsetDateTime::from_unix_timestamp(timestamp_secs as i64) {
            Ok(t) => t,
            Err(_) => {
                return false;
            } // Invalid timestamp
        };

        if let Some(weekday_str) = &self.weekday {
            // Convert weekday index to time crate's Weekday enum
            let weekday = match Weekday::from_str(weekday_str) {
                Ok(w) => w,
                Err(e) => {
                    debug!("Invalid Weekday set for distribution reward interval {e:?}");
                    return false;
                } // Invalid weekday index
            };

            // Check if the given timestamp is on the specified weekday
            if timestamp.weekday() == weekday {
                // Check if the given timestamp is within the specified hour range
                let hour = timestamp.hour();
                if hour >= self.start_hour && hour < self.end_hour {
                    return true;
                }
            }
        } else {
            return false;
        }

        false
    }
    pub fn is_within_daily_interval(&self, timestamp_millis: TimestampMillis) -> bool {
        let timestamp_secs = timestamp_millis / 1000; // Convert milliseconds to seconds
                                                      // Create a DateTime equivalent using time crate
        let timestamp = match time::OffsetDateTime::from_unix_timestamp(timestamp_secs as i64) {
            Ok(t) => t,
            Err(_) => {
                return false;
            }
        };

        // Check if the given timestamp is within the specified hour range
        let hour = timestamp.hour();
        if hour >= self.start_hour && hour < self.end_hour {
            return true;
        }

        false
    }
}

pub fn set_unstake_state_of_position(
    stake_position_id: &StakePositionId,
    stake_position: &StakePosition,
    new_state: UnstakeState,
) {
    let mut updated_position = stake_position.clone();
    updated_position.unstake_state = new_state;
    mutate_state(|s| {
        s.data
            .stake_system
            .update_stake_position(stake_position_id, updated_position)
    });
}
