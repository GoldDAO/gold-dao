use candid::CandidType;
use candid::Principal;
use serde::Deserialize;
use serde::Serialize;
use types::TimestampMillis;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub stake_position_user: Principal,
    pub dissovle_event_id: u8,
    pub new_dissolve_timestamp: TimestampMillis,
}
pub type Response = Result<(), String>;
