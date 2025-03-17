use candid::{CandidType, Principal};
use canister_time::MINUTE_IN_MS;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use types::Milliseconds;

pub const MANAGE_ARCHIVE_CYCLE_INTERVAL: Milliseconds = MINUTE_IN_MS * 10;
pub const MANAGE_NEW_ARCHIVES_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_SERVICE_STATUS_INTERVAL: Milliseconds = MINUTE_IN_MS;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ArchiveCanister {
    #[serde(serialize_with = "custom_serialize_json_principal")]
    pub canister_id: Principal,
    pub active: bool,
}

fn custom_serialize_json_principal<S>(
    principal: &Principal,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_principal = principal.to_text();
    serializer.serialize_str(&string_principal)
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveStatus {
    Up,
    Down(ArchiveDownReason),
    Upgrading,
    Initializing,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveDownReason {
    NewArchiveError(NewArchiveError),
    Upgrading,
    UpgradingArchivesFailed(String),
    NoArchiveCanisters(String),
    LowOrigynToken(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum NewArchiveError {
    FailedToSerializeInitArgs(String),
    CreateCanisterError(String),
    InstallCodeError(String),
    CantFindControllers(String),
}
