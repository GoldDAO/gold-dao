use crate::{ utils::random_principal, wasms };
use candid::CandidType;
use serde::{ Serialize, Deserialize };
use candid::Principal;
use pocket_ic::{ PocketIc, PocketIcBuilder };
use ic_ledger_types::Tokens;
use types::TokenInfo;
use candid::encode_one;
use types::BuildVersion;
use types::CanisterId;

const INIT_CYCLES_BALANCE: u128 = 1_000 * 1_000_000_000_000;

// Macro to reduce code duplication for creating and installing canisters
macro_rules! create_and_install_canister {
    ($pic:expr, $controller:expr, $subnet:expr, $wasm:expr, $args:expr) => {
        {
            let canister_id = $pic.create_canister_on_subnet(Some($controller.clone()), None, $subnet);
            $pic.add_cycles(canister_id, INIT_CYCLES_BALANCE);
            $pic.install_canister(canister_id, $wasm.clone(), encode_one($args).unwrap(), Some($controller.clone()));
            canister_id
        }
    };
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    test_mode: bool,
    version: BuildVersion,
    commit_hash: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Args {
    Init(InitArgs),
    Upgrade(UpgradeArgs),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeArgs {
    pub version: BuildVersion,
    pub commit_hash: String,
}

pub struct SNCTestEnv {
    pub controller: Principal,
    pub buyback_burn_canister_id: CanisterId,
    pub icp_neuron_canister_id: CanisterId,
    pub management_canister_id: CanisterId,
    pub sns_neuron_controller_canister_id: CanisterId,
    pub sns_rewards_canister_id: CanisterId,
    pub super_stats_v3_canister_id: CanisterId,
    pub token_metrics_canister_id: CanisterId,
    pub pic: PocketIc,
}
pub struct SNCTestEnvBuilder {
    controller: Principal,
}

impl Default for SNCTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
        }
    }
}

impl SNCTestEnvBuilder {
    pub fn new() -> Self {
        SNCTestEnvBuilder::default()
    }

    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn build(&mut self) -> SNCTestEnv {
        let pic = PocketIcBuilder::new().with_sns_subnet().with_application_subnet().build();
        let sns_subnet = pic.topology().get_sns().unwrap();

        let commit_hash = "TestCommitHash1".to_string();
        let test_mode = true;
        let controller = self.controller.clone();

        // Canister IDs
        let authorized_principals = vec![self.controller.clone()];
        let ogy_sns_governance_canister_id = Principal::anonymous();
        let ogy_sns_ledger_canister_id = Principal::anonymous();
        let ogy_sns_rewards_canister_id = Principal::anonymous();
        let icp_swap_canister_id = Principal::anonymous();
        let icp_ledger_canister_id = Principal::anonymous();
        let sns_ledger_canister_id = Principal::anonymous();
        let ogy_ledger_canister_id = Principal::anonymous();
        let sns_gov_canister_id = Principal::anonymous();
        let sns_governance_canister_id = Principal::anonymous();
        let super_stats_canister_id = Principal::anonymous();
        let ogy_new_ledger_canister_id = Principal::anonymous();
        let sns_rewards_canister_id = Principal::anonymous();

        let buyback_burn_args = buyback_burn_api::Args::Init(buyback_burn_api::init::InitArgs {
            test_mode,
            version: BuildVersion::min(),
            commit_hash: commit_hash.clone(),
            authorized_principals: authorized_principals.clone(),
            gldgov_token_info: TokenInfo {
                ledger_id: ogy_sns_ledger_canister_id,
                fee: 10000,
                decimals: 8,
            },
            tokens: vec![],
            buyback_burn_interval_in_secs: 1000,
            icp_swap_canister_id,
            burn_rate: 33,
            min_burn_amount: Tokens::from_e8s(100000),
        });

        let icp_neuron_args = Args::Init(InitArgs {
            test_mode,
            version: BuildVersion::min(),
            commit_hash: commit_hash.clone(),
        });

        let management_args = management_api_canister::Args::Init(
            management_api_canister::init::InitArgs {
                test_mode,
                version: BuildVersion::min(),
                commit_hash: commit_hash.clone(),
                authorized_principals: authorized_principals.clone(),
            }
        );

        let snc_init_args = sns_neuron_controller_api_canister::Args::Init(
            sns_neuron_controller_api_canister::init::InitArgs {
                test_mode,
                version: BuildVersion::min(),
                commit_hash: commit_hash.clone(),
                authorized_principals: authorized_principals.clone(),
                sns_rewards_canister_id,
                ogy_sns_governance_canister_id,
                ogy_sns_ledger_canister_id,
                ogy_sns_rewards_canister_id,
            }
        );

        let sns_rewards_init_args = sns_rewards_api_canister::Args::Init(
            sns_rewards_api_canister::init::InitArgs {
                test_mode: true,
                version: BuildVersion::min(),
                commit_hash: commit_hash.clone(),
                icp_ledger_canister_id,
                sns_ledger_canister_id,
                ogy_ledger_canister_id,
                sns_gov_canister_id,
            }
        );

        let super_stats_init_args = super_stats_v3_api::init_and_upgrade::InitArgs {
            test_mode: true,
            admin: "Admin".to_string(),
        };

        let token_metrics_init_args = token_metrics_api::Args::Init(
            token_metrics_api::init::InitArgs {
                test_mode: true,
                version: BuildVersion::min(),
                commit_hash: commit_hash.clone(),
                sns_governance_canister_id,
                super_stats_canister_id,
                ogy_new_ledger_canister_id,
                sns_rewards_canister_id,
                treasury_account: "Treasuty account".to_string(),
                foundation_accounts: vec!["Foundation account".to_string()],
            }
        );

        // let buyback_burn_canister_id = create_and_install_canister!(
        //     pic,
        //     controller,
        //     sns_subnet,
        //     wasms::BUYBACK_BURN,
        //     buyback_burn_args
        // );

        let buyback_burn_canister_id = pic.create_canister_on_subnet(
            Some(self.controller.clone()),
            None,
            sns_subnet
        );
        pic.add_cycles(buyback_burn_canister_id, INIT_CYCLES_BALANCE);
        pic.install_canister(
            buyback_burn_canister_id,
            wasms::BUYBACK_BURN.clone(),
            encode_one(buyback_burn_args).unwrap(),
            Some(controller.clone())
        );

        println!("buyback_burn_canister was installed");

        let icp_neuron_canister_id = create_and_install_canister!(
            pic,
            controller,
            sns_subnet,
            wasms::ICP_NEURON,
            icp_neuron_args
        );
        println!("icp_neuron_canister was installed");
        let management_canister_id = create_and_install_canister!(
            pic,
            controller,
            sns_subnet,
            wasms::MANAGEMENT,
            management_args
        );
        println!("management_canister_id was installed");
        let sns_neuron_controller_canister_id = create_and_install_canister!(
            pic,
            controller,
            sns_subnet,
            wasms::SNS_NEURON_CONTROLLER,
            snc_init_args
        );
        println!("sns_neuron_controller_canister_id was installed");
        let sns_rewards_canister_id = create_and_install_canister!(
            pic,
            controller,
            sns_subnet,
            wasms::REWARDS,
            sns_rewards_init_args
        );
        println!("sns_rewards_canister_id was installed");
        let super_stats_v3_canister_id = create_and_install_canister!(
            pic,
            controller,
            sns_subnet,
            wasms::SUPER_STATS,
            super_stats_init_args
        );
        println!("super_stats_v3_canister_id was installed");
        let token_metrics_canister_id = create_and_install_canister!(
            pic,
            controller,
            sns_subnet,
            wasms::TOKEN_METRICS,
            token_metrics_init_args
        );
        println!("token_metrics_canister_id was installed");

        SNCTestEnv {
            controller: self.controller,
            buyback_burn_canister_id,
            icp_neuron_canister_id,
            management_canister_id,
            sns_neuron_controller_canister_id,
            sns_rewards_canister_id,
            super_stats_v3_canister_id,
            token_metrics_canister_id,
            pic,
        }
    }
}

// env.create_canister_on_subnet(Some(controller), None, canister_id)
// .expect("Create canister with ID failed");

// pub fn install_canister_with_id()

// pub fn create_canister_on_subnet(env: &mut PocketIc, controller: Principal, canister_id: &str) -> CanisterId {
//     let canister_id = canister_id.try_into().expect("Invalid canister ID");
//     env.create_canister_on_subnet(Some(controller), None, canister_id)
//         .expect("Create canister with ID failed");
//     env.add_cycles(canister_id, INIT_CYCLES_BALANCE);
//     canister_id
// }
