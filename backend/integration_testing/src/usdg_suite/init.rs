use candid::{Nat, Principal};
use gldt_swap_common::{gldt::GLDT_TX_FEE, nft::NftCanisterConf};
use icrc_ledger_canister::init::{InitArgs, LedgerArgument};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{PocketIc, PocketIcBuilder};
use std::{env, path::Path, time::SystemTime};
use types::{BuildVersion, CanisterId};
use usdg_minter_api::{
    lifecycle::{InitArgument, MinterArgument},
    USDG_TRANSFER_FEE,
};
// use ic_icrc1_ledger::{ ArchiveOptions, InitArgs as LedgerInitArgs, LedgerArgument };
use super::{CanisterIds, PrincipalIds, TestEnv};
use crate::{
    client::pocket::{create_canister, create_canister_with_id, install_canister},
    utils::random_principal,
    wasms,
};
use icrc_ledger_types::icrc3::archive::{GetArchivesArgs, GetArchivesResult};

pub static POCKET_IC_BIN: &str = "./pocket-ic";

pub fn default_setup() -> TestEnv {
    validate_pocketic_installation();

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

    let principal_ids: PrincipalIds = PrincipalIds {
        controller: random_principal(),
        user: random_principal(),
    };
    let canister_ids: CanisterIds =
        install_canisters(&mut pic, principal_ids.controller, principal_ids.user);

    TestEnv {
        pic: pic,
        canister_ids,
        principal_ids,
    }
}

fn install_canisters(pic: &mut PocketIc, controller: Principal, user: Principal) -> CanisterIds {
    let gldt_ledger = create_canister_with_id(pic, controller, "oh54a-baaaa-aaaap-abryq-cai");
    let usdg_ledger = create_canister(pic, controller);
    let usdg_minter = create_canister(pic, controller);

    // pic.add_cycles(gldt_swap_canister_id, 20_000_000_000_000);

    let minter_arg = MinterArgument::Init(InitArgument {
        usdg_ledger_id: usdg_ledger,
        gldt_ledger_id: gldt_ledger,
        gold_dao_governance_id: Principal::management_canister(),
        xrc_id: Principal::management_canister(),
    });
    install_canister(
        pic,
        controller,
        usdg_minter,
        wasms::USDG_MINTER.clone(),
        minter_arg,
    );

    let usdg_ledger_init_args = LedgerArgument::Init(InitArgs {
        fee_collector_account: None,
        minting_account: Account::from(usdg_minter),
        initial_balances: vec![(Account::from(controller), Nat::from(100_000_000_000 as u64))],
        archive_options: icrc_ledger_canister::init::ArchiveOptions {
            trigger_threshold: 2000,
            num_blocks_to_archive: 1000,
            controller_id: controller,
        },
        metadata: vec![],
        transfer_fee: Nat::from(USDG_TRANSFER_FEE),
        token_symbol: "USDG".to_string(),
        token_name: "USDG".to_string(),
    });
    install_canister(
        pic,
        controller,
        usdg_ledger,
        wasms::IC_ICRC2_LEDGER.clone(),
        usdg_ledger_init_args,
    );

    let gldt_ledger_init_args = LedgerArgument::Init(InitArgs {
        fee_collector_account: None,
        minting_account: Account::from(controller),
        initial_balances: vec![(Account::from(user), Nat::from(1_000_000_00_000_000 as u64))],
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
        gldt_ledger,
        wasms::IC_ICRC2_LEDGER.clone(),
        gldt_ledger_init_args,
    );

    pic.set_time(SystemTime::now());

    CanisterIds {
        gldt_ledger,
        usdg_ledger,
        usdg_minter,
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
