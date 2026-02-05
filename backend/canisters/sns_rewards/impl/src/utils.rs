use std::str::FromStr;

use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::TransferArg,
};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::{Neuron, NeuronId};
use time::Date;
use time::UtcOffset;
use time::{OffsetDateTime, Time, Weekday};
use tracing::{debug, error, info, warn};
use types::TimestampMillis;

use crate::state::read_state;

use sns_governance_canister::types::get_neuron_response::Result::{
    Error as NeuronErrorResponse, Neuron as NeuronResponse,
};

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
                    debug!(
                        "Invalid Weekday set for distribution reward interval: {}",
                        e
                    );
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

pub async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat,
) -> Result<(), String> {
    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        ledger_id,
        &(TransferArg {
            from_subaccount: Some(from_sub_account),
            to: to_account,
            fee: None,
            created_at_time: None,
            amount,
            memo: None,
        }),
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

pub enum FetchNeuronDataByIdResponse {
    NeuronDoesNotExist,
    Ok(Neuron),
    InternalError(String),
}

pub async fn fetch_neuron_data_by_id(neuron_id: &NeuronId) -> FetchNeuronDataByIdResponse {
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let args = sns_governance_canister::get_neuron::Args {
        neuron_id: Some(neuron_id.clone()),
    };
    match sns_governance_canister_c2c_client::get_neuron(canister_id, &args).await {
        Ok(neuron_data) => match neuron_data.result {
            Some(neuron) => match neuron {
                NeuronResponse(n) => FetchNeuronDataByIdResponse::Ok(n),
                NeuronErrorResponse(_) => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
            },
            None => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
        },
        Err(e) => {
            debug!(
                "Error fetching neuron with id : {:?}, message : {:?}",
                neuron_id, e,
            );
            FetchNeuronDataByIdResponse::InternalError(e.to_string())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticateByHotkeyResponse {
    Ok(bool),
    NeuronHotKeyInvalid,
}

pub fn authenticate_by_hotkey(
    neuron_data: &Neuron,
    caller: &Principal,
) -> AuthenticateByHotkeyResponse {
    // Check if any of the permission principals contain an entry that matches the caller principal
    let matching_caller_hotkey = neuron_data
        .permissions
        .iter()
        .filter(|permission| permission.principal.as_ref() == Some(caller))
        .count();

    if matching_caller_hotkey >= 1 {
        AuthenticateByHotkeyResponse::Ok(true)
    } else {
        AuthenticateByHotkeyResponse::NeuronHotKeyInvalid
    }
}

pub fn next_first_wednesday_of_month_seconds(
    now_seconds: u64,
    hour: u8,
) -> Result<u64, NextDistributionError> {
    if hour > 23 {
        warn!(hour, "Validation failed: hour must be between 0 and 23");
        return Err(NextDistributionError::InvalidHour(hour));
    }

    let now = OffsetDateTime::from_unix_timestamp(now_seconds as i64).map_err(|e| {
        error!(error = ?e, "Failed to parse timestamp");
        NextDistributionError::InvalidTimestamp
    })?;

    let time = Time::MIDNIGHT
        .replace_hour(hour)
        .map_err(|_| NextDistributionError::InvalidHour(hour))?;

    let mut year = now.year();
    let mut month = now.month();

    info!(current_time = ?now, target_hour = hour, "Calculating next first Wednesday");

    loop {
        debug!(year, ?month, "Checking first Wednesday candidate for month");

        let mut candidate = OffsetDateTime::from_unix_timestamp(0)
            .map_err(|_| NextDistributionError::DateConstructionFailed)?
            .replace_year(year)
            .map_err(|_| NextDistributionError::DateConstructionFailed)?
            .replace_month(month)
            .map_err(|_| NextDistributionError::DateConstructionFailed)?
            .replace_day(1)
            .map_err(|_| NextDistributionError::DateConstructionFailed)?
            .replace_time(time);

        // Iterate to find the first Wednesday
        while candidate.weekday() != Weekday::Wednesday {
            candidate = candidate.saturating_add(time::Duration::days(1));
        }

        if candidate > now {
            let ts = candidate.unix_timestamp() as u64;
            info!(
                found_date = ?candidate,
                timestamp = ts,
                "Successfully found next first Wednesday"
            );
            return Ok(ts);
        }

        debug!(
            skipped_date = ?candidate,
            "First Wednesday of this month has passed, moving to next month"
        );

        month = match month {
            time::Month::December => {
                year += 1;
                time::Month::January
            }
            _ => month.next(),
        };
    }
}

pub fn next_14_feb_seconds(hour: u8) -> Result<u64, NextDistributionError> {
    use time::{Date, Month};

    let date = Date::from_calendar_date(2026, Month::February, 14)
        .map_err(|_| NextDistributionError::DateConstructionFailed)?;

    let time = Time::MIDNIGHT
        .replace_hour(hour)
        .map_err(|_| NextDistributionError::InvalidHour(hour))?;

    let datetime = date.with_time(time).assume_utc();
    Ok(datetime.unix_timestamp() as u64)
}

pub fn next_day_seconds(now_seconds: u64, hour: u8) -> Result<u64, NextDistributionError> {
    if hour > 23 {
        warn!(hour, "Attempted to calculate next day with invalid hour");
        return Err(NextDistributionError::InvalidHour(hour));
    }

    let now = OffsetDateTime::from_unix_timestamp(now_seconds as i64).map_err(|e| {
        error!(error = ?e, "Failed to parse now_seconds into OffsetDateTime");
        NextDistributionError::InvalidTimestamp
    })?;

    let time = Time::MIDNIGHT
        .replace_hour(hour)
        .map_err(|_| NextDistributionError::InvalidHour(hour))?;

    let mut candidate = now.replace_time(time);

    if candidate <= now {
        debug!(passed_time = ?candidate, "Target hour already passed for today; scheduling for tomorrow");
        candidate = candidate.saturating_add(time::Duration::days(1));
    } else {
        debug!(target_time = ?candidate, "Target hour is later today; scheduling for today");
    }

    let ts = candidate.unix_timestamp() as u64;
    info!(scheduled_for = ?candidate, timestamp = ts, "Next daily distribution calculated");

    Ok(ts)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextDistributionError {
    InvalidTimestamp,
    InvalidHour(u8),
    DateConstructionFailed,
}
#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_schedule_later_today() {
        // Now is 10:00 AM
        let now = datetime!(2024-06-18 10:00:00 UTC);
        // Target is 3:00 PM (15:00) today
        let result = next_day_seconds(now.unix_timestamp() as u64, 15).unwrap();

        let expected = datetime!(2024-06-18 15:00:00 UTC).unix_timestamp() as u64;
        assert_eq!(result, expected, "Should schedule for later the same day");
    }

    #[test]
    fn test_schedule_tomorrow_because_hour_passed() {
        // Now is 4:00 PM (16:00)
        let now = datetime!(2024-06-18 16:00:00 UTC);
        // Target is 9:00 AM today (which already passed)
        let result = next_day_seconds(now.unix_timestamp() as u64, 9).unwrap();

        // Should be 9:00 AM on June 19th
        let expected = datetime!(2024-06-19 09:00:00 UTC).unix_timestamp() as u64;
        assert_eq!(
            result, expected,
            "Should schedule for tomorrow if hour has passed"
        );
    }

    #[test]
    fn test_schedule_tomorrow_exact_match() {
        // Now is exactly 09:00:00
        let now = datetime!(2026-01-20 23:35:00 UTC);
        // Target is 09:00:00
        let result = next_day_seconds(now.unix_timestamp() as u64, 12).unwrap();
        let duration = result.saturating_sub(now.unix_timestamp() as u64);

        println!("Duration until next distribution: {} seconds", duration);
    }

    #[test]
    fn test_invalid_hour_error() {
        let now = datetime!(2024-06-18 10:00:00 UTC);
        let result = next_day_seconds(now.unix_timestamp() as u64, 25);
        assert!(matches!(
            result,
            Err(NextDistributionError::InvalidHour(25))
        ));
    }

    #[test]
    fn test_midnight_rollover() {
        // Now is 11:30 PM
        let now = datetime!(2024-12-31 23:30:00 UTC);
        // Target is 01:00 AM
        let result = next_day_seconds(now.unix_timestamp() as u64, 1).unwrap();

        // Should be 1:00 AM Jan 1st of the next year
        let expected = datetime!(2025-01-01 01:00:00 UTC).unix_timestamp() as u64;
        assert_eq!(result, expected, "Should handle year rollover correctly");
    }

    use super::authenticate_by_hotkey;
    use super::*;
    use crate::utils::{AuthenticateByHotkeyResponse, TimeInterval};
    use candid::Principal;
    use sns_governance_canister::types::{Neuron, NeuronId, NeuronPermission};

    #[test]
    fn test_authenticate_by_hotkey_with_correct_data() {
        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
        neuron.permissions.push(NeuronPermission {
            principal: Some(caller.clone()),
            permission_type: vec![3, 4],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::Ok(true));
    }

    #[test]
    fn test_authenticate_by_hotkey_with_no_hotkey_but_valid_caller() {
        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(caller.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::Ok(true));
    }

    #[test]
    fn test_authenticate_by_hotkey_with_no_hotkeys_invalid_caller() {
        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::NeuronHotKeyInvalid)
    }

    #[test]
    fn test_authenticate_by_hotkey_with_invalid_hotkey() {
        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
        neuron.permissions.push(NeuronPermission {
            principal: Some(Principal::from_text("tyyy3-4aaaa-aaaaq-aab7a-cai").unwrap()),
            permission_type: vec![3, 4],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::NeuronHotKeyInvalid)
    }

    #[test]
    fn test_weekly_distribution_interval() {
        let distribution_interval = TimeInterval {
            weekday: Some("Wednesday".to_string()),
            start_hour: 14,
            end_hour: 16,
        }; // wednesday between 14:00 and 16:00

        let time_now = 1718809200855; // UTC - wednesday Jun 19, 2024, 3:00:00 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            true
        );

        let time_now = 1718805600855; // UTC - wednesday Jun 19, 2024, 14:00:00 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            true
        );

        let time_now = 1718812799855; // UTC - wednesday Jun 19, 2024, 15:59:59 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            true
        );

        let time_now = 1718812800855; // UTC - wednesday Jun 19, 2024, 16:00:00 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            false
        );

        let time_now = 1718805599855; // UTC - wednesday Jun 19, 2024, 13:59:59 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            false
        );

        let time_now = 1718722800855; // UTC - Tuesday Jun 18, 2024, 15:00:00 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            false
        );

        let time_now = 1719430200000; // UTC - Wednesday Jun 26, 2024, 7:30:00 PM
        assert_eq!(
            distribution_interval.is_within_weekly_interval(time_now),
            false
        );
    }

    #[test]
    fn test_daily_distribution_interval() {
        let distribution_interval = TimeInterval {
            weekday: None,
            start_hour: 9,
            end_hour: 11,
        }; // any day between 9:00:00 and 11:00:00

        let time_now = 1718787600000; // UTC - wednesday Jun 19, 2024, 09:00:00
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            true
        );

        let time_now = 1718791200000; // UTC - wednesday Jun 19, 2024, 10:00:00
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            true
        );

        let time_now = 1718794800000; // UTC - wednesday Jun 19, 2024, 11:00:00
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            false
        );

        let time_now = 1718794801000; // UTC - wednesday Jun 19, 2024, 11:00:01
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            false
        );

        let time_now = 1718787599000; // UTC - wednesday Jun 19, 2024, 08:59:59
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            false
        );

        let time_now = 1718874000000; // UTC - Thursday Jun 20, 2024, 09:00:00
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            true
        );

        let time_now = 1718830800000; // UTC - Thursday Jun 20, 2024, 09:00:00
        assert_eq!(
            distribution_interval.is_within_daily_interval(time_now),
            false
        );
    }

    #[test]
    fn test_next_first_wednesday_basic() {
        let now = OffsetDateTime::parse(
            "2026-03-01T10:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        let res = next_first_wednesday_of_month_seconds(now, 0).unwrap();

        let expected = OffsetDateTime::parse(
            "2026-03-04T00:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        assert_eq!(res, expected);
    }

    #[test]
    fn test() {
        let now = OffsetDateTime::parse(
            "2024-06-18T08:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        let res = next_first_wednesday_of_month_seconds(now, 0).unwrap();

        println!("res: {}", res);
    }

    #[test]
    fn test_next_first_wednesday_rollover() {
        let now = OffsetDateTime::parse(
            "2026-03-06T00:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        let res = next_first_wednesday_of_month_seconds(now, 0).unwrap();

        let expected = OffsetDateTime::parse(
            "2026-04-01T00:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        assert_eq!(res, expected);
    }

    #[test]
    fn test_next_first_wednesday_year_boundary() {
        let now = OffsetDateTime::parse(
            "2026-12-31T23:59:59Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        let res = next_first_wednesday_of_month_seconds(now, 0).unwrap();

        let expected = OffsetDateTime::parse(
            "2027-01-06T00:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap()
        .unix_timestamp() as u64;

        assert_eq!(res, expected);
    }

    #[test]
    fn test_next_14_feb() {
        let result = next_14_feb_seconds(15).unwrap();

        let expected = datetime!(2026-02-14 15:00:00 UTC).unix_timestamp() as u64;
        assert_eq!(result, expected, "Should schedule for 14 Feb");
    }
}
