use candid::Principal;
use crate::token_data::GovernanceStats;

pub type Args = Option<Principal>;
pub type Response = GovernanceStats;
