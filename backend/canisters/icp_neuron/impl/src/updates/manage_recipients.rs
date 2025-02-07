use crate::{guards::caller_is_governance_principal, state::mutate_state};
use canister_tracing_macros::trace;
use ic_cdk::{query, update};
pub use icp_neuron_api_canister::manage_recipients::{
    ManageRewardRecipientsRequest, ManageRewardRecipientsResponse,
};
use types::{RewardsRecipient, RewardsRecipientList};

// method to add / remove recipients of reward distribution
#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn manage_reward_recipients(
    args: ManageRewardRecipientsRequest,
) -> ManageRewardRecipientsResponse {
    match manage_reward_recipients_impl(args.list).await {
        Ok(_) => ManageRewardRecipientsResponse::Success,
        Err(err) => ManageRewardRecipientsResponse::InternalError(err),
    }
}

pub(crate) async fn manage_reward_recipients_impl(
    list: Vec<RewardsRecipient>,
) -> Result<(), String> {
    mutate_state(|s| -> Result<(), String> {
        match s.data.rewards_recipients.set(list) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    })
}

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manage_reward_recipients_validate(
    args: ManageRewardRecipientsRequest,
) -> Result<String, String> {
    // test initialising rewards list
    RewardsRecipientList::validate(&args.list)?;
    // return prettified string
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}
