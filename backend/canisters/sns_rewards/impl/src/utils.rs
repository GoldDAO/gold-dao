use std::collections::HashMap;
use std::str::FromStr;

use candid::{ CandidType, Nat, Principal };
use time::macros::datetime;
use time::{ error::ComponentRange, Weekday };
use time;
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::TransferArg };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::{ Neuron, NeuronId };
use tracing::debug;
use types::{ TimestampMillis, TokenSymbol };

use crate::state::read_state;

use sns_governance_canister::types::get_neuron_response::Result::{
    Neuron as NeuronResponse,
    Error as NeuronErrorResponse,
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
            end_hour: 16, // 4pm
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
                    debug!("Invalid Weekday set for distribution reward interval");
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
    amount: Nat
) -> Result<(), String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_id,
            &(TransferArg {
                from_subaccount: Some(from_sub_account),
                to: to_account,
                fee: None,
                created_at_time: None,
                amount: amount,
                memo: None,
            })
        ).await
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
        Ok(neuron_data) => {
            match neuron_data.result {
                Some(neuron) => {
                    match neuron {
                        NeuronResponse(n) => FetchNeuronDataByIdResponse::Ok(n),
                        NeuronErrorResponse(_) => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
                    }
                }
                None => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
            }
        }
        Err(e) => {
            debug!(
                "Error fetching neuron with id : {:?}, error code : {:?}, message : {:?}",
                neuron_id,
                e.0,
                e.1
            );
            FetchNeuronDataByIdResponse::InternalError(e.1)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticateByHotkeyResponse {
    NeuronHotKeyAbsent,
    Ok(bool),
    NeuronHotKeyInvalid,
}

pub fn authenticate_by_hotkey(
    neuron_data: &Neuron,
    caller: &Principal
) -> AuthenticateByHotkeyResponse {
    // first is always the nns owner principal so if less than or equal to 1 then no hotkeys have been added.
    if neuron_data.permissions.len() <= 1 {
        return AuthenticateByHotkeyResponse::NeuronHotKeyAbsent;
    }

    // Check if any of the permission principals contain an entry that matches the caller principal
    let matching_caller_hotkey = neuron_data.permissions
        .iter()
        .skip(1)
        .filter(|permission| permission.principal.as_ref() == Some(caller))
        .count();

    if matching_caller_hotkey >= 1 {
        AuthenticateByHotkeyResponse::Ok(true)
    } else {
        AuthenticateByHotkeyResponse::NeuronHotKeyInvalid
    }
}

pub fn validate_set_reserve_transfer_amounts_payload(
    args: &HashMap<TokenSymbol, Nat>
) -> Result<(), String> {
    if args.len() < (1 as usize) {
        return Err("Should contain at least 1 token symbol and amount to update".to_string());
    }

    for (token_symbol, amount) in args {
        // Check the amount is above 0.
        if amount == &Nat::from(0u64) {
            return Err(
                format!("ERROR : The amount for token : {:?} must be more than 0", token_symbol)
            );
        }
    }
    Ok(())
}

pub fn validate_set_daily_gldgov_burn_rate_payload(amount: &Nat) -> Result<(), String> {
    if amount == &Nat::from(0u64) {
        return Err(format!("ERROR : The amount for burning must be more than 0"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use sns_governance_canister::types::{ Neuron, NeuronId, NeuronPermission };
    use types::TimestampMillis;

    use crate::utils::{ AuthenticateByHotkeyResponse, TimeInterval };
    use super::authenticate_by_hotkey;

    #[test]
    fn test_authenticate_by_hotkey_with_correct_data() {
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

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
    fn test_authenticate_by_hotkey_with_no_hotkeys() {
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::NeuronHotKeyAbsent)
    }

    #[test]
    fn test_authenticate_by_hotkey_with_invalid_hotkey() {
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

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
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), true);

        let time_now = 1718805600855; // UTC - wednesday Jun 19, 2024, 14:00:00 PM
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), true);

        let time_now = 1718812799855; // UTC - wednesday Jun 19, 2024, 15:59:59 PM
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), true);

        let time_now = 1718812800855; // UTC - wednesday Jun 19, 2024, 16:00:00 PM
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), false);

        let time_now = 1718805599855; // UTC - wednesday Jun 19, 2024, 13:59:59 PM
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), false);

        let time_now = 1718722800855; // UTC - Tuesday Jun 18, 2024, 15:00:00 PM
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), false);

        let time_now = 1719430200000; // UTC - Wednesday Jun 26, 2024, 7:30:00 PM
        assert_eq!(distribution_interval.is_within_weekly_interval(time_now), false);
    }

    #[test]
    fn test_daily_distribution_interval() {
        let distribution_interval = TimeInterval {
            weekday: None,
            start_hour: 14,
            end_hour: 16,
        }; // wednesday between 14:00 and 16:00

        let time_now = 1718809200855; // UTC - wednesday Jun 19, 2024, 3:00:00 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), true);

        let time_now = 1718805600855; // UTC - wednesday Jun 19, 2024, 14:00:00 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), true);

        let time_now = 1718812799855; // UTC - wednesday Jun 19, 2024, 15:59:59 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), true);

        let time_now = 1718812800855; // UTC - wednesday Jun 19, 2024, 16:00:00 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), false);

        let time_now = 1718805599855; // UTC - wednesday Jun 19, 2024, 13:59:59 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), false);

        let time_now = 1718722800855; // UTC - Tuesday Jun 18, 2024, 15:00:00 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), true);

        let time_now = 1719430200000; // UTC - Wednesday Jun 26, 2024, 7:30:00 PM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), false);

        let time_now = 1721290471006; // UTC - Thursday Jul 18, 2024, 8:14:31 AM
        assert_eq!(distribution_interval.is_within_daily_interval(time_now), false);
    }
}
