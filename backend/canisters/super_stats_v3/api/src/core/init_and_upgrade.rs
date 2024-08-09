use candid::CandidType;
use serde::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct InitArgs {
    pub admin: String,
    pub test_mode: bool,
}
