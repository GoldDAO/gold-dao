use crate::gldt_swap_suite::setup::setup_gldt_ledger::setup_gldt_ledger;
use crate::gldt_swap_suite::setup::setup_gldt_swap::setup_gldt_swap_canister;
use crate::origyn_nft_test_env::origyn_nft_test_env::OrigynNftTestEnv;
use crate::origyn_nft_test_env::origyn_nft_test_env::OrigynNftTestEnvBuilder;
use crate::utils::random_principal;
use bity_ic_icrc3::config::ICRC3Config;
use bity_ic_icrc3::config::ICRC3Properties;
use bity_ic_types::BuildVersion;
use candid::Nat;
use candid::Principal;
use gldt_swap_api_canister::init::InitArgs;
use gldt_swap_common::swap_canister_config::FractionalizationConfig;
use gldt_swap_common::swap_canister_config::GeneralFractionalizationConfig;
use gldt_swap_common::swap_canister_config::SwapCanisterConfig;
use icrc_ledger_types::icrc3::blocks::SupportedBlockType;
use origyn_nft_canister::InitApprovalsArg;
use origyn_nft_canister::Permission;
use origyn_nft_canister::PermissionManager;
use pocket_ic::{PocketIc, PocketIcBuilder};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

pub struct TestEnv {
    pub pic: Rc<RefCell<PocketIc>>,
    pub origyn_nft_test_env: OrigynNftTestEnv,
    pub gldt_ledger_canister_id: Principal,
    pub gldt_swap_canister_id: Principal,
    pub owner_1: Principal,
    pub owner_2: Principal,
}

pub struct TestEnvBuilder {
    pub controller: Principal,
    gldt_ledger_canister_id: Principal,
    gldt_swap_canister_id: Principal,
}

impl Default for TestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            gldt_ledger_canister_id: Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai").unwrap(),
            gldt_swap_canister_id: Principal::from_text("6f6ua-hqaaa-aaaar-qairq-cai").unwrap(),
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
        println!("Start building TestEnv");

        let pic_ref = Rc::new(RefCell::new(
            PocketIcBuilder::new()
                .with_application_subnet()
                .with_application_subnet()
                .with_sns_subnet()
                .with_fiduciary_subnet()
                .with_nns_subnet()
                .with_system_subnet()
                .build(),
        ));
        let pic = pic_ref.borrow();

        let gldt_ledger_canister_id = setup_gldt_ledger(
            &pic,
            self.controller.clone(),
            self.gldt_ledger_canister_id,
            self.gldt_swap_canister_id,
        );

        let mut user_permissions = HashMap::new();
        user_permissions.insert(
            self.controller,
            vec![
                Permission::Minting,
                Permission::ManageAuthorities,
                Permission::UpdateMetadata,
                Permission::UpdateCollectionMetadata,
                Permission::ReadUploads,
                Permission::UpdateUploads,
            ],
        );

        let origyn_nft_init_args = origyn_nft_canister::InitArgs {
            test_mode: true,
            version: bity_ic_types::BuildVersion::min(),
            commit_hash: "commit_hash".to_string(),
            permissions: PermissionManager::new(user_permissions),
            description: None,
            symbol: "MC".to_string(),
            name: "MyCollection".to_string(),
            logo: None,
            supply_cap: Some(Nat::from(10u64)),
            max_query_batch_size: None,
            max_update_batch_size: None,
            max_take_value: None,
            default_take_value: None,
            max_memo_size: None,
            atomic_batch_transfers: None,
            tx_window: None,
            permitted_drift: None,
            max_canister_storage_threshold: None,
            collection_metadata: HashMap::new(),
            approval_init: InitApprovalsArg {
                max_approvals_per_token_or_collection: Some(Nat::from(10u64)),
                max_revoke_approvals: Some(Nat::from(10u64)),
            },
        };
        let origyn_nft_test_env =
            OrigynNftTestEnvBuilder::new(&pic_ref, self.controller).build(origyn_nft_init_args);

        let swap_config = SwapCanisterConfig {
            icrc7_canister_id: origyn_nft_test_env.collection_canister_id,
            fractionalization_config: FractionalizationConfig::General(
                GeneralFractionalizationConfig {
                    division: 100_000_000_000_000,
                    swap_fee: Nat::from(100_000_000_u64),
                    ledger_id: self.gldt_ledger_canister_id,
                },
            ),
        };

        // INIT ICRC3
        let mut constants = ICRC3Properties::default();
        // constants.max_memory_size_bytes = 1000;
        constants.max_memory_size_bytes = 60000;
        constants.tx_window = Duration::from_millis(500);
        constants.max_transactions_in_window = 100;
        constants.max_blocks_per_response = 100;
        constants.max_transactions_to_purge = 5;
        constants.initial_cycles = 5_000_000_000_000;
        constants.reserved_cycles = 5_000_000_000_000;
        // INIT ICRC3

        let gldt_swap_init_args = gldt_swap_api_canister::Args::Init(InitArgs {
            test_mode: true,
            version: BuildVersion::min(),
            commit_hash: "Test".to_string(),
            swap_configs: vec![swap_config],
            authorized_principals: vec![self.controller],
            buyback_burn_canister: None,
            icrc3_config: ICRC3Config {
                supported_blocks: vec![SupportedBlockType {
                    block_type: "reverse_swap".to_string(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-3/README.md#supported-block-types".to_string(),
                },SupportedBlockType {
                    block_type: "forward_swap".to_string(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-3/README.md#supported-block-types".to_string(),
                }],
                constants,
            },
        });
        let gldt_swap_canister_id = setup_gldt_swap_canister(
            &pic,
            self.controller,
            self.gldt_swap_canister_id,
            gldt_swap_init_args,
        );

        TestEnv {
            pic: Rc::clone(&pic_ref),
            origyn_nft_test_env,
            gldt_ledger_canister_id,
            gldt_swap_canister_id,
            owner_1: Principal::from_text("54vkq-taaaa-aaaap-ahqra-cai").unwrap(),
            owner_2: Principal::from_text("s2ryu-oyaaa-aaaap-qhq2q-cai").unwrap(),
        }
    }
}
