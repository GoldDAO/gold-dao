use candid::CandidType;
use serde::{ Deserialize, Serialize };

#[derive(CandidType, Deserialize, Serialize, Clone, Default, Debug)]
pub struct GetVotingParticipationHistoryArgs {
    pub days: u64,
}
pub type Args = GetVotingParticipationHistoryArgs;
pub type Response = Vec<(u64, u64)>;
