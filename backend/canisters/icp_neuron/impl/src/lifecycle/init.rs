use crate::lifecycle::init_canister;
use crate::state::{ Data, RewardsRecipients, RuntimeState };
use candid::{ CandidType, Principal };
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use utils::env::{ CanisterEnv, Environment };

#[derive(Deserialize, CandidType, Debug)]
pub struct InitArgs {
    test_mode: bool,
    rewards_recipients: Vec<RewardsRecipients>,
}

#[init]
#[trace]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let rewards_recipients = args.rewards_recipients.clone();
    // validate_rewards_recipients(&rewards_recipients);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::new(rewards_recipients);

    if args.test_mode {
        data.authorized_principals.push(env.caller());
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}

fn validate_rewards_recipients(rewards_recipients: &Vec<RewardsRecipients>) {
    if rewards_recipients.is_empty() {
        ic_cdk::api::trap("Invalid rewards recipients: empty list.");
    }
    // expecting 4 recipients in the current design
    if rewards_recipients.len() > 5 {
        ic_cdk::api::trap("Invalid rewards recipients: too many recipients.");
    }
    for recipient in rewards_recipients {
        if recipient.account.owner == Principal::anonymous() {
            ic_cdk::api::trap("Invalid rewards recipient: account owner is anonymous.");
        }
        if recipient.reward_weight == 0 || recipient.reward_weight > 10000 {
            ic_cdk::api::trap(
                "Invalid rewards recipient: reward weight has to be between 1 and 10000."
            );
        }
    }
}
