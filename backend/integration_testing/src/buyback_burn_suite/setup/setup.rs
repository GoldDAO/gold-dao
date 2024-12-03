use crate::buyback_burn_suite::setup::setup_buyback_burn::setup_buyback_burn_canister;
use crate::utils::random_principal;
use buyback_burn_api::Args;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use ic_ledger_types::Tokens;
use pocket_ic::{PocketIc, PocketIcBuilder};
use types::BuildVersion;
use types::CanisterId;
use types::TokenInfo;

#[derive(CandidType, Deserialize, Debug)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

pub struct TestEnv {
    pub controller: Principal,
    pub buyback_burn_id: CanisterId,
    pub pic: PocketIc,
}

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
impl Debug for TestEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestEnv")
            .field("buyback_burn_id", &self.buyback_burn_id.to_text())
            .finish()
    }
}
pub struct TestEnvBuilder {
    controller: Principal,
    buyback_burn_id: CanisterId,
}

impl Default for TestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            buyback_burn_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        }
    }
}

impl TestEnvBuilder {
    pub fn new() -> Self {
        TestEnvBuilder::default()
    }

    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn build(&mut self) -> TestEnv {
        let mut pic = PocketIcBuilder::new()
            .with_sns_subnet()
            .with_application_subnet()
            .build();

        let sns_subnet = pic.topology().get_sns().unwrap();

        self.buyback_burn_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);

        let buyback_burn_init_args = Args::Init(buyback_burn_api::init::InitArgs {
            test_mode: true,
            version: BuildVersion::min(),
            gldgov_token_info: TokenInfo {
                ledger_id: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
                fee: 10000,
                decimals: 8,
            },
            tokens: vec![],
            buyback_interval_in_secs: 100000,
            icp_swap_canister_id: Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            burn_rate: 33,
            min_burn_amount: Tokens::from_e8s(100_000_000),
            commit_hash: "".to_string(),
            authorized_principals: vec![],
        });

        let buyback_burn_canister_id = setup_buyback_burn_canister(
            &mut pic,
            self.buyback_burn_id,
            buyback_burn_init_args,
            self.controller,
        );

        TestEnv {
            controller: self.controller,
            buyback_burn_id: buyback_burn_canister_id,
            pic,
        }
    }
}
