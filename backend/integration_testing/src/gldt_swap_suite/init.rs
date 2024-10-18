use std::{ env, path::Path, time::SystemTime };
use candid::{ Nat, Principal };
use gldt_swap_common::{ gldt::{ GLDT_LEDGER_FEE_ACCOUNT, GLDT_TX_FEE }, nft::NftCanisterConf };
use gldt_swap_api_canister::lifecycle::Args as GldtSwapCanisterArgs;
use gldt_swap_api_canister::init::InitArgs as GldtSwapCanisterInitArgs;
use icrc_ledger_canister::init::{ InitArgs, LedgerArgument };
use pocket_ic::{ PocketIc, PocketIcBuilder };
use utils::consts::E8S_FEE_OGY;
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::ManageStorageRequestConfigureStorage;
use types::{ BuildVersion, CanisterId };

// use ic_icrc1_ledger::{ ArchiveOptions, InitArgs as LedgerInitArgs, LedgerArgument };
use icrc_ledger_types::icrc3::archive::{ GetArchivesArgs, GetArchivesResult };

use crate::{
    client::pocket::{ create_canister, create_canister_with_id, install_canister },
    utils::random_principal,
    wasms,
};

use super::{ nft_utils, CanisterIds, PrincipalIds, TestEnv };

pub static POCKET_IC_BIN: &str = "./pocket-ic";

pub fn init() -> TestEnv {
    validate_pocketic_installation();
    println!("install validate");

    // let mut pic: PocketIc = PocketIc::new();
    let mut pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_application_subnet()
        .with_sns_subnet()
        .with_fiduciary_subnet()
        .with_nns_subnet()
        .with_system_subnet()
        .build();

    let get_app_subnets = pic.topology().get_app_subnets()[1];

    println!("topology {:?}", pic.topology());
    println!("get_app_subnets {:?}", get_app_subnets.to_string());
    println!("pic set");

    // let mut pic: PocketIc = PocketIc::new();
    // println!("pic set");

    let principal_ids: PrincipalIds = PrincipalIds {
        net_principal: random_principal(),
        controller: random_principal(),
        originator: random_principal(),
        nft_owner: random_principal(),
    };
    let canister_ids: CanisterIds = install_canisters(
        &mut pic,
        principal_ids.controller,
        principal_ids.nft_owner
    );
    println!("origyn_nft: {:?}", canister_ids.origyn_nft.to_string());
    println!("ogy_ledger: {:?}", canister_ids.ogy_ledger.to_string());
    println!("gldt_ledger: {:?}", canister_ids.gldt_ledger.to_string());
    println!("gldt_swap: {:?}", canister_ids.gldt_swap.to_string());

    init_origyn_nft(
        &mut pic,
        canister_ids.origyn_nft,
        principal_ids.controller,
        principal_ids.originator,
        principal_ids.net_principal,
        canister_ids.ogy_ledger
    );
    TestEnv {
        pic: pic,
        canister_ids,
        principal_ids,
    }
}

fn init_origyn_nft(
    pic: &mut PocketIc,
    canister: CanisterId,
    controller: Principal,
    originator: Principal,
    net_principal: Principal,
    ogy_principal: Principal
) {
    let manage_storage_return: origyn_nft_reference::origyn_nft_reference_canister::ManageStorageResult = crate::client::origyn_nft_reference::client::manage_storage_nft_origyn(
        pic,
        canister,
        Some(controller),
        crate::client::origyn_nft_reference::manage_storage_nft_origyn::Args::ConfigureStorage(
            ManageStorageRequestConfigureStorage::Heap(Some(Nat::from(500000000 as u32)))
        )
    );

    println!("manage_storage_return: {:?}", manage_storage_return);

    let collection_update_return: origyn_nft_reference::origyn_nft_reference_canister::OrigynBoolResult = crate::client::origyn_nft_reference::client::collection_update_nft_origyn(
        pic,
        canister,
        Some(controller),
        crate::client::origyn_nft_reference::collection_update_nft_origyn::Args::UpdateOwner(
            net_principal
        )
    );
    println!("collection_update_return: {:?}", collection_update_return);

    let standard_collection_return = nft_utils::build_standard_collection(
        pic,
        canister.clone(),
        canister.clone(),
        originator.clone(),
        Nat::from(1024 as u32),
        net_principal.clone(),
        nft_utils::ICTokenSpec {
            canister: ogy_principal,
            fee: Some(Nat::from(E8S_FEE_OGY)),
            symbol: "OGY".to_string(),
            decimals: Nat::from(8 as u32),
            standard: nft_utils::TokenStandard::Ledger,
            id: None,
        }
    );
    println!("standard_collection_return: {:?}", standard_collection_return);
}

fn install_canisters(
    pic: &mut PocketIc,
    controller: Principal,
    nft_owner: Principal
) -> CanisterIds {
    let origyn_nft_canister_id: Principal = create_canister(pic, controller);
    let ogy_ledger_canister_id: Principal = create_canister_with_id(
        pic,
        controller,
        "lkwrt-vyaaa-aaaaq-aadhq-cai"
    );
    let icp_ledger_canister_id: Principal = create_canister_with_id(
        pic,
        controller,
        "ryjl3-tyaaa-aaaaa-aaaba-cai"
    );
    let gldt_ledger_canister_id: Principal = create_canister(pic, controller);
    let gldt_swap_canister_id: Principal = create_canister(pic, controller);
    pic.add_cycles(gldt_swap_canister_id, 20_000_000_000_000);

    let origyn_nft_canister_wasm: Vec<u8> = wasms::ORIGYN_NFT.clone();
    let ogy_ledger_canister_wasm: Vec<u8> = wasms::IC_ICRC2_LEDGER.clone();
    let gldt_ledger_canister_wasm: Vec<u8> = wasms::IC_ICRC2_LEDGER.clone();
    let icp_ledger_canister_wasm: Vec<u8> = wasms::IC_ICRC2_LEDGER.clone();
    let gldt_swap_canister_wasm: Vec<u8> = wasms::GLDT_SWAP.clone();

    install_canister(pic, controller, origyn_nft_canister_id, origyn_nft_canister_wasm, {});

    let ogy_ledger_init_args: icrc_ledger_canister::init::LedgerArgument = icrc_ledger_canister::init::LedgerArgument::Init(
        icrc_ledger_canister::init::InitArgs {
            minting_account: Account::from(controller),
            initial_balances: vec![
                (Account::from(controller), Nat::from(18_446_744_073_709 as u64)),
                (Account::from(origyn_nft_canister_id), Nat::from(18_446_744_073_709 as u64)),
                (Account::from(nft_owner), Nat::from(18_446_744_073_709 as u64)),
                (Account::from(gldt_swap_canister_id), Nat::from(100_000_000_000_000 as u64))
            ],
            archive_options: icrc_ledger_canister::init::ArchiveOptions {
                trigger_threshold: 2000,
                num_blocks_to_archive: 1000,
                controller_id: controller,
            },
            metadata: vec![],
            transfer_fee: Nat::from(E8S_FEE_OGY),
            token_symbol: "OGY".into(),
            token_name: "Origyn".into(),
            fee_collector_account: None,
        }
    );

    println!("ogy_ledger_canister_id {:?}", ogy_ledger_canister_id);

    install_canister(
        pic,
        controller,
        ogy_ledger_canister_id,
        ogy_ledger_canister_wasm,
        ogy_ledger_init_args
    );
    let gldt_ledger_init_args = LedgerArgument::Init(InitArgs {
        fee_collector_account: Some(Account {
            owner: gldt_swap_canister_id,
            subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
        }),
        minting_account: Account::from(gldt_swap_canister_id),
        initial_balances: vec![(Account::from(controller), Nat::from(100_000_000_000 as u64))],
        archive_options: icrc_ledger_canister::init::ArchiveOptions {
            trigger_threshold: 2000,
            num_blocks_to_archive: 1000,
            controller_id: controller,
        },
        metadata: vec![],
        transfer_fee: Nat::from(GLDT_TX_FEE),
        token_symbol: "GLDT".to_string(),
        token_name: "GLDT".to_string(),
    });

    install_canister(
        pic,
        controller,
        gldt_ledger_canister_id,
        gldt_ledger_canister_wasm,
        gldt_ledger_init_args
    );

    let gldt_swap_init_args: GldtSwapCanisterArgs = GldtSwapCanisterArgs::Init(
        GldtSwapCanisterInitArgs {
            commit_hash: "abcdefgh".to_string(),
            version: BuildVersion::min(),
            test_mode: true,
            gldt_ledger_id: gldt_ledger_canister_id.clone(),
            gldnft_canisters: vec![(origyn_nft_canister_id, NftCanisterConf { grams: 1u16 })],
            ogy_ledger_id: ogy_ledger_canister_id,
            authorized_principals: vec![controller.clone()],
        }
    );

    pic.set_time(SystemTime::now());

    install_canister(
        pic,
        controller,
        gldt_swap_canister_id,
        gldt_swap_canister_wasm,
        gldt_swap_init_args
    );

    let icp_ledger_init_args: icrc_ledger_canister::init::LedgerArgument = icrc_ledger_canister::init::LedgerArgument::Init(
        icrc_ledger_canister::init::InitArgs {
            minting_account: Account::from(controller),
            initial_balances: vec![
                (Account::from(controller), Nat::from(18_446_744_073_709 as u64)),
                (Account::from(origyn_nft_canister_id), Nat::from(18_446_744_073_709 as u64)),
                (Account::from(nft_owner), Nat::from(18_446_744_073_709 as u64))
                // (Account::from(gldt_swap_canister_id), Nat::from(100_000_000_000_000 as u64))
            ],
            archive_options: icrc_ledger_canister::init::ArchiveOptions {
                trigger_threshold: 2000,
                num_blocks_to_archive: 1000,
                controller_id: controller,
            },
            metadata: vec![],
            transfer_fee: Nat::from(E8S_FEE_OGY),
            token_symbol: "ICP".into(),
            token_name: "ICP".into(),
            fee_collector_account: None,
        }
    );

    println!("ogy_ledger_canister_id {:?}", icp_ledger_canister_id);

    install_canister(
        pic,
        controller,
        icp_ledger_canister_id,
        icp_ledger_canister_wasm,
        icp_ledger_init_args
    );

    CanisterIds {
        origyn_nft: origyn_nft_canister_id,
        ogy_ledger: ogy_ledger_canister_id,
        gldt_ledger: gldt_ledger_canister_id,
        gldt_swap: gldt_swap_canister_id,
        icp_ledger: icp_ledger_canister_id,
    }
}

pub fn validate_pocketic_installation() {
    let path = POCKET_IC_BIN;

    if !Path::new(&path).exists() {
        println!(
            "
        Could not find the PocketIC binary to run canister integration tests.

        I looked for it at {:?}. You can specify another path with the environment variable POCKET_IC_BIN (note that I run from {:?}).
        ",
            &path,
            &env
                ::current_dir()
                .map(|x| x.display().to_string())
                .unwrap_or_else(|_| "an unknown directory".to_string())
        );
    }
}
