use crate::lifecycle::init_canister;
use crate::state::{ Data, RuntimeState };
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use types::{ RewardsRecipient, RewardsRecipientList };
use utils::consts::STAGING_SNS_GOVERNANCE_CANISTER_ID;
use utils::env::{ CanisterEnv, Environment };

#[derive(Deserialize, CandidType, Debug)]
pub struct InitArgs {
    test_mode: bool,
    rewards_recipients: Vec<RewardsRecipient>,
}

#[init]
#[trace]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let rewards_recipients = RewardsRecipientList::new(args.rewards_recipients.clone()).unwrap();

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::new(rewards_recipients);

    if args.test_mode {
        data.authorized_principals.push(env.caller());
        data.authorized_principals.push(STAGING_SNS_GOVERNANCE_CANISTER_ID);
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
