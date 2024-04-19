use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{ PocketIc, PocketIcBuilder };

use crate::{ client::icrc1::client::transfer, CanisterIds };

use super::{
    setup_ledger::setup_ledgers,
    setup_rewards::setup_rewards_canister,
    setup_sns::{ setup_sns_by_week, SNSTestEnv },
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
    let sns = setup_sns_by_week(&mut pic, 1, None);
    let rewards = setup_rewards_canister(&mut pic, &token_ledgers, &sns.sns_gov_id);

    setup_reward_pools(&mut pic, controller, rewards, token_ledgers, 100_000_000_000);
    TestEnv {
        pic,
        controller,
        token_ledgers,
        sns,
        rewards,
    }
}

pub fn setup_reward_pools(
    mut pic: &mut PocketIc,
    minting_account: Principal,
    reward_canister_id: Principal,
    canister_ids: CanisterIds,
    amount: u64
) {
    let reward_account = Account {
        owner: reward_canister_id,
        subaccount: Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]),
    };

    for canister_id in canister_ids.into_iter() {
        transfer(
            &mut pic,
            minting_account,
            canister_id,
            None,
            reward_account,
            amount.into()
        ).unwrap();
    }
}
