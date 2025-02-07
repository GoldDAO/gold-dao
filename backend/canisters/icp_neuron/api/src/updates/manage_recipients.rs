use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::RewardsRecipient;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ManageRewardRecipientsResponse {
    Success,
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ManageRewardRecipientsRequest {
    pub list: Vec<RewardsRecipient>,
}
