use crate::buyback_burn_suite::setup::setup_buyback_burn::setup_buyback_burn_canister;
use crate::utils::random_principal;
use crate::utils::tick_n_blocks;
use bity_ic_canister_time::HOUR_IN_MS;
use bity_ic_types::BuildVersion;
use buyback_burn_api::exchange_job_config::ExchangeJobConfig;
use buyback_burn_api::icpswap::ICPSwapConfig;
use buyback_burn_api::swap_config::ExchangeConfig;
use buyback_burn_api::Args;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use ic_ledger_types::Tokens;
use pocket_ic::{PocketIc, PocketIcBuilder};
use std::time::Duration;
use types::CanisterId;
use types::TokenSymbol;

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
            exchange_configs: vec![ExchangeJobConfig {
                token_to_sell: TokenSymbol::ICP,
                token_to_buy: TokenSymbol::GLDT,
                exchange: ExchangeConfig::ICPSwap(ICPSwapConfig {
                    swap_canister_id: Principal::from_text("k46ek-4qaaa-aaaag-qcyzq-cai").unwrap(),
                    zero_for_one: true,
                }),
                rate_per_interval: 714_286,
                job_interval_ms: 0,
                source_subaccount: None,
                min_amount: Tokens::from_e8s(0),
                max_amount: Some(Tokens::from_e8s(0)),
                destination_account: None,
            }],
            icp_swap_canister_id: Principal::from_text("7eikv-2iaaa-aaaag-qdgwa-cai").unwrap(),
            commit_hash: "".to_string(),
            authorized_principals: vec![self.controller.clone()],
        });

        let buyback_burn_canister_id = setup_buyback_burn_canister(
            &mut pic,
            self.buyback_burn_id,
            buyback_burn_init_args,
            self.controller,
        );

        // Tuesday Jun 18, 2024, 9:00:00 AM
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        tick_n_blocks(&pic, 5);

        TestEnv {
            controller: self.controller,
            buyback_burn_id: buyback_burn_canister_id,
            pic,
        }
    }
}
