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

pub struct RealDataTestEnv {
    pub pic: Rc<RefCell<PocketIc>>,
    pub gold_1g_nft_test_env: OrigynNftTestEnv,
    pub gold_10g_nft_test_env: OrigynNftTestEnv,
    pub gold_100g_nft_test_env: OrigynNftTestEnv,
    pub gold_1000g_nft_test_env: OrigynNftTestEnv,
    pub gldt_ledger_canister_id: Principal,
    pub gldt_swap_canister_id: Principal,
    pub owner_1: Principal,
    pub owner_2: Principal,
}

pub struct RealDataTestEnvBuilder {
    pub controller: Principal,
    gldt_ledger_canister_id: Principal,
    gldt_swap_canister_id: Principal,
}

impl Default for RealDataTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            gldt_ledger_canister_id: Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai").unwrap(),
            gldt_swap_canister_id: Principal::from_text("6f6ua-hqaaa-aaaar-qairq-cai").unwrap(),
        }
    }
}

impl RealDataTestEnvBuilder {
    pub fn new() -> Self {
        RealDataTestEnvBuilder::default()
    }

    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn build(&mut self) -> RealDataTestEnv {
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

        let gold_nft_1g_init_args = make_gold_nft_init_args(1, &user_permissions);
        let gold_nft_10g_init_args = make_gold_nft_init_args(10, &user_permissions);
        let gold_nft_100g_init_args = make_gold_nft_init_args(100, &user_permissions);
        let gold_nft_1000g_init_args = make_gold_nft_init_args(1000, &user_permissions);

        let gold_1g_nft_test_env =
            OrigynNftTestEnvBuilder::new(&pic_ref, self.controller).build(gold_nft_1g_init_args);
        let gold_10g_nft_test_env =
            OrigynNftTestEnvBuilder::new(&pic_ref, self.controller).build(gold_nft_10g_init_args);
        let gold_100g_nft_test_env =
            OrigynNftTestEnvBuilder::new(&pic_ref, self.controller).build(gold_nft_100g_init_args);
        let gold_1000g_nft_test_env =
            OrigynNftTestEnvBuilder::new(&pic_ref, self.controller).build(gold_nft_1000g_init_args);

        let swap_config_1g = make_swap_config(
            gold_1g_nft_test_env.collection_canister_id,
            1,
            self.gldt_ledger_canister_id,
        );
        let swap_config_10g = make_swap_config(
            gold_10g_nft_test_env.collection_canister_id,
            10,
            self.gldt_ledger_canister_id,
        );
        let swap_config_100g = make_swap_config(
            gold_100g_nft_test_env.collection_canister_id,
            100,
            self.gldt_ledger_canister_id,
        );
        let swap_config_1000g = make_swap_config(
            gold_1000g_nft_test_env.collection_canister_id,
            1000,
            self.gldt_ledger_canister_id,
        );

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
            swap_configs: vec![swap_config_1g, swap_config_10g, swap_config_100g, swap_config_1000g],
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

        RealDataTestEnv {
            pic: Rc::clone(&pic_ref),
            gold_1g_nft_test_env,
            gold_10g_nft_test_env,
            gold_100g_nft_test_env,
            gold_1000g_nft_test_env,
            gldt_ledger_canister_id,
            gldt_swap_canister_id,
            owner_1: Principal::from_text("54vkq-taaaa-aaaap-ahqra-cai").unwrap(),
            owner_2: Principal::from_text("s2ryu-oyaaa-aaaap-qhq2q-cai").unwrap(),
        }
    }
}

fn make_gold_nft_init_args(
    grams: u64,
    user_permissions: &HashMap<Principal, Vec<Permission>>,
) -> origyn_nft_canister::InitArgs {
    origyn_nft_canister::InitArgs {
        test_mode: true,
        version: bity_ic_types::BuildVersion::min(),
        commit_hash: "commit_hash".to_string(),
        permissions: PermissionManager::new(user_permissions.clone()),
        description: None,
        symbol: format!("GldNft{}g", grams),
        name: format!("Gold {}g collection", grams),
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
    }
}

fn make_swap_config(
    icrc7_canister_id: Principal,
    grams: u64,
    ledger_id: Principal,
) -> SwapCanisterConfig {
    SwapCanisterConfig {
        icrc7_canister_id,
        fractionalization_config: FractionalizationConfig::General(
            GeneralFractionalizationConfig {
                division: grams * 100_000_000, // scale factor
                swap_fee: Nat::from(100_000_000_u64 - 10_000_000_u64),
                ledger_id,
            },
        ),
    }
}
