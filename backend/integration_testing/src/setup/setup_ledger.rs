use candid::{ encode_one, Nat, Principal };
use icrc_ledger_canister::init::{ ArchiveOptions as ArchiveOptionsIcrc, InitArgs, LedgerArgument };
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;

use crate::{ wasms, CanisterIds };

pub fn setup_ledgers(pic: &PocketIc, controller: Principal) -> CanisterIds {
    let app_subnet_id = pic.topology().get_app_subnets()[0];
    let icp_ledger_id = pic.create_canister_on_subnet(None, None, app_subnet_id);
    let ogy_ledger_id = pic.create_canister_on_subnet(None, None, app_subnet_id);
    let gldgov_ledger_id = pic.create_canister_on_subnet(None, None, app_subnet_id);

    pic.add_cycles(icp_ledger_id, 10_000_000_000_000);
    pic.add_cycles(ogy_ledger_id, 10_000_000_000_000);
    pic.add_cycles(gldgov_ledger_id, 10_000_000_000_000);

    let icrc1_ledger_wasm = wasms::IC_ICRC1_LEDGER.clone();

    pic.install_canister(
        icp_ledger_id,
        icrc1_ledger_wasm.clone(),
        encode_one(generate_ledger_canister_init_args("ICP", controller)).unwrap(),
        None
    );
    pic.install_canister(
        ogy_ledger_id,
        icrc1_ledger_wasm.clone(),
        encode_one(generate_ledger_canister_init_args("OGY", controller)).unwrap(),
        None
    );
    pic.install_canister(
        gldgov_ledger_id,
        icrc1_ledger_wasm.clone(),
        encode_one(generate_ledger_canister_init_args("GLDGov", controller)).unwrap(),
        None
    );
    CanisterIds {
        icp_ledger_id,
        ogy_ledger_id,
        gldgov_ledger_id,
    }
}

pub fn generate_ledger_canister_init_args(token: &str, controller: Principal) -> LedgerArgument {
    let payload = LedgerArgument::Init(InitArgs {
        minting_account: Account::from(controller),
        initial_balances: vec![(Account::from(controller), Nat::from(1_000_000_000_000_000u64))],
        transfer_fee: Nat::from(10000u64),
        token_name: token.into(),
        token_symbol: token.into(),
        metadata: Vec::new(),
        archive_options: ArchiveOptionsIcrc {
            trigger_threshold: 1000,
            num_blocks_to_archive: 1000,
            controller_id: controller,
        },
    });

    payload
}
