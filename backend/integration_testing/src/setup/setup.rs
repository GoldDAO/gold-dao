use candid::Principal;
use pocket_ic::{ PocketIc, PocketIcBuilder };

use crate::{ utils::random_principal, CanisterIds };

use super::{
    ledger::setup_ledgers,
    rewards::setup_rewards_canister,
    sns::{ setup_sns_by_week, SNSTestEnv },
};

pub struct TestEnv {
    pub pic: PocketIc,
    pub controller: Principal,
    pub token_ledgers: CanisterIds,
    pub sns: SNSTestEnv,
    pub rewards: Principal,
}

pub static POCKET_IC_BIN: &str = "./pocket-ic";

pub fn init() -> TestEnv {
    let mut pic = PocketIcBuilder::new().with_sns_subnet().with_application_subnet().build();

    let controller = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let token_ledgers = setup_ledgers(&mut pic, controller);
    let sns = setup_sns_by_week(&mut pic, controller, 1, None);
    let rewards = setup_rewards_canister(&mut pic, &token_ledgers, &sns.sns_gov_id);
    TestEnv {
        pic,
        controller,
        token_ledgers,
        sns,
        rewards,
    }
}
