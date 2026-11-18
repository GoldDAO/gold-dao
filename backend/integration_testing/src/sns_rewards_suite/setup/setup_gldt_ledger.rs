use crate::client::pocket::create_canister_with_id;
use crate::client::pocket::install_canister;
use crate::wasms;
use candid::{Nat, Principal};
use icrc_ledger_canister::init::{InitArgs, LedgerArgument};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;

const GLDT_TX_FEE: u64 = 10_000_000u64;
const GLDT_LEDGER_FEE_ACCOUNT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

pub fn setup_gldt_ledger(
    pic: &PocketIc,
    controller: Principal,
    gldt_ledger_canister_id: Principal,
    minting_account_owner: Principal,
) -> Principal {
    let gldt_ledger_canister_id =
        create_canister_with_id(pic, controller, &gldt_ledger_canister_id.to_text());
    pic.add_cycles(gldt_ledger_canister_id, 20_000_000_000_000);

    let gldt_ledger_canister_wasm: Vec<u8> = wasms::IC_ICRC2_LEDGER.clone();

    let gldt_ledger_init_args = LedgerArgument::Init(InitArgs {
        fee_collector_account: Some(Account {
            owner: minting_account_owner,
            subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
        }),
        minting_account: Account::from(minting_account_owner),
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
        gldt_ledger_init_args,
    );

    gldt_ledger_canister_id
}
