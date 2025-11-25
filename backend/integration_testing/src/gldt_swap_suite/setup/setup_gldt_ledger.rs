use crate::client::pocket::create_canister_with_id;
use crate::client::pocket::install_canister;
use crate::wasms;
use candid::{Nat, Principal};
use gldt_swap_common::gldt::{GLDT_LEDGER_FEE_ACCOUNT, GLDT_TX_FEE};
use icrc_ledger_canister::init::{InitArgs, LedgerArgument};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;

pub fn setup_gldt_ledger(
    pic: &PocketIc,
    controller: Principal,
    gldt_ledger_canister_id: Principal,
    gldt_swap_canister_id: Principal,
) -> Principal {
    let gldt_ledger_canister_id =
        create_canister_with_id(pic, controller, &gldt_ledger_canister_id.to_text());
    pic.add_cycles(gldt_ledger_canister_id, 20_000_000_000_000);

    let gldt_ledger_canister_wasm: Vec<u8> = wasms::IC_ICRC2_LEDGER.clone();

    let gldt_ledger_init_args = LedgerArgument::Init(InitArgs {
        fee_collector_account: Some(Account {
            owner: gldt_swap_canister_id,
            subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
        }),
        minting_account: Account::from(gldt_swap_canister_id),
        // initial_balances: vec![(Account::from(controller), Nat::from(100_000_000_000 as u64))],
        initial_balances: vec![],
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
        gldt_ledger_init_args,
    );

    gldt_ledger_canister_id
}
