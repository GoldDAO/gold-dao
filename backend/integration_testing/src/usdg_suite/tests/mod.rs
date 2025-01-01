use crate::client::icrc1_icrc2_token::icrc1_balance_of;
use crate::client::icrc1_icrc2_token::icrc2_approve;
use crate::client::usdg::{
    add_margin_to_vault, borrow_from_vault, deposit_liquidity, get_lp_position,
    get_vaults_by_account, open_vault, withdraw_liquidity,
};
use crate::usdg_suite::init;
use assert_matches::assert_matches;
use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types_ic_main_repo::icrc1::account::Account as ICAccount;
use std::time::Duration;
use usdg_minter_api::queries::get_lp_position::LiquidationPoolPosition;
use usdg_minter_api::updates::add_margin_to_vault::AddMarginArg;
use usdg_minter_api::updates::borrow_from_vault::BorrowArg;
use usdg_minter_api::updates::deposit_liquidity::DepositArg;
use usdg_minter_api::updates::open_vault::{OpenVaultArg, OpenVaultSuccess};
use usdg_minter_api::updates::withdraw_liquidity::WithdrawArg;
use usdg_minter_api::{ApiFeeBucket, ApiVault};

const E8S: u64 = 100_000_000;

#[test]
fn usdg_should_open_vault() {
    let mut env = init::default_setup();

    let arg: Option<ICAccount> = None;
    let vault = get_vaults_by_account(
        &env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &arg,
    );
    assert!(vault.is_empty());

    icrc2_approve(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.gldt_ledger,
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: env.canister_ids.usdg_minter,
                subaccount: None,
            },
            amount: Nat::from(2_000 * E8S),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );

    let open_vault_arg = OpenVaultArg {
        margin_amount: 1_000 * E8S,
        borrowed_amount: 0,
        fee_bucket: ApiFeeBucket::Low,
        maybe_subaccount: None,
    };
    let open_result = open_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &open_vault_arg,
    );
    assert_matches!(
        open_result,
        Ok(OpenVaultSuccess {
            block_index: 2,
            vault_id: 0,
        })
    );

    let arg: Option<ICAccount> = None;
    let vault = get_vaults_by_account(
        &env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &arg,
    );
    assert_eq!(vault.len(), 1);
    assert_eq!(
        vault[0],
        ApiVault {
            vault_id: 0,
            owner: ICAccount {
                owner: env.principal_ids.user,
                subaccount: None
            },
            borrowed_amount: 0,
            margin_amount: 1_000 * E8S,
            fee_bucket: ApiFeeBucket::Low,
        }
    );
}

#[test]
fn usdg_should_add_margin_to_vault() {
    let mut env = init::default_setup();

    icrc2_approve(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.gldt_ledger,
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: env.canister_ids.usdg_minter,
                subaccount: None,
            },
            amount: Nat::from(2_000 * E8S),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );

    let open_vault_arg = OpenVaultArg {
        margin_amount: 1_000 * E8S,
        borrowed_amount: 0,
        fee_bucket: ApiFeeBucket::Low,
        maybe_subaccount: None,
    };
    let open_result = open_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &open_vault_arg,
    );
    assert_matches!(
        open_result,
        Ok(OpenVaultSuccess {
            block_index: 2,
            vault_id: 0,
        })
    );

    let add_margin_arg = AddMarginArg {
        vault_id: 0,
        margin_amount: 100 * E8S,
    };
    let add_margin_result = add_margin_to_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &add_margin_arg,
    );
    assert_matches!(add_margin_result, Ok(3));

    let arg: Option<ICAccount> = None;
    let vault = get_vaults_by_account(
        &env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &arg,
    );
    assert_eq!(vault.len(), 1);
    assert_eq!(
        vault[0],
        ApiVault {
            vault_id: 0,
            owner: ICAccount {
                owner: env.principal_ids.user,
                subaccount: None
            },
            borrowed_amount: 0,
            margin_amount: 1_100 * E8S,
            fee_bucket: ApiFeeBucket::Low,
        }
    );
}

#[test]
fn usdg_should_borrow_from_vault() {
    let mut env = init::default_setup();

    icrc2_approve(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.gldt_ledger,
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: env.canister_ids.usdg_minter,
                subaccount: None,
            },
            amount: Nat::from(2_000 * E8S),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );

    let open_vault_arg = OpenVaultArg {
        margin_amount: 1_000 * E8S,
        borrowed_amount: 0,
        fee_bucket: ApiFeeBucket::Low,
        maybe_subaccount: None,
    };
    let open_result = open_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &open_vault_arg,
    );
    assert_matches!(
        open_result,
        Ok(OpenVaultSuccess {
            block_index: 2,
            vault_id: 0,
        })
    );

    let borrow_arg = BorrowArg {
        vault_id: 0,
        borrowed_amount: 100 * E8S,
    };
    let borrow_result = borrow_from_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &borrow_arg,
    );
    assert_matches!(borrow_result, Ok(1));

    let usdg_balance = icrc1_balance_of(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_ledger,
        &Account {
            owner: env.principal_ids.user,
            subaccount: None,
        },
    );
    assert_eq!(usdg_balance, Nat::from(100 * E8S));
}

#[test]
fn usdg_should_repay_and_close() {
    let mut env = init::default_setup();

    icrc2_approve(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.gldt_ledger,
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: env.canister_ids.usdg_minter,
                subaccount: None,
            },
            amount: Nat::from(2_000 * E8S),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );

    let open_vault_arg = OpenVaultArg {
        margin_amount: 1_000 * E8S,
        borrowed_amount: 300 * E8S,
        fee_bucket: ApiFeeBucket::Low,
        maybe_subaccount: None,
    };
    let open_result = open_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &open_vault_arg,
    );
    assert_matches!(
        open_result,
        Ok(OpenVaultSuccess {
            block_index: 2,
            vault_id: 0,
        })
    );

    env.pic.advance_time(Duration::from_secs(5));
    env.pic.tick();

    let usdg_balance = icrc1_balance_of(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_ledger,
        &Account {
            owner: env.principal_ids.user,
            subaccount: None,
        },
    );
    assert_eq!(usdg_balance, Nat::from(300 * E8S));
}

#[test]
fn usdg_should_deposit_and_withdraw_liquidity() {
    let mut env = init::default_setup();

    icrc2_approve(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.gldt_ledger,
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: env.canister_ids.usdg_minter,
                subaccount: None,
            },
            amount: Nat::from(2_000 * E8S),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );

    let open_vault_arg = OpenVaultArg {
        margin_amount: 1_000 * E8S,
        borrowed_amount: 0,
        fee_bucket: ApiFeeBucket::Low,
        maybe_subaccount: None,
    };
    let open_result = open_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &open_vault_arg,
    );
    assert_matches!(
        open_result,
        Ok(OpenVaultSuccess {
            block_index: 2,
            vault_id: 0,
        })
    );

    let borrow_arg = BorrowArg {
        vault_id: 0,
        borrowed_amount: 100 * E8S,
    };
    let borrow_result = borrow_from_vault(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_minter,
        &borrow_arg,
    );
    assert_matches!(borrow_result, Ok(1));

    icrc2_approve(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_ledger,
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: env.canister_ids.usdg_minter,
                subaccount: None,
            },
            amount: Nat::from(2_000 * E8S),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );

    let initial_balance = icrc1_balance_of(
        &mut env.pic,
        env.principal_ids.user,
        env.canister_ids.usdg_ledger,
        &Account {
            owner: env.principal_ids.user,
            subaccount: None,
        },
    );

    assert_matches!(
        deposit_liquidity(
            &mut env.pic,
            env.principal_ids.user,
            env.canister_ids.usdg_minter,
            &DepositArg {
                deposited_amount: 50 * E8S,
                maybe_subaccount: None,
            },
        ),
        Ok(3)
    );

    assert_eq!(
        icrc1_balance_of(
            &mut env.pic,
            env.principal_ids.user,
            env.canister_ids.usdg_ledger,
            &Account {
                owner: env.principal_ids.user,
                subaccount: None
            }
        ),
        initial_balance.clone() - 50 * E8S
    );

    assert_eq!(
        get_lp_position(
            &env.pic,
            env.principal_ids.user,
            env.canister_ids.usdg_minter,
            &None,
        ),
        LiquidationPoolPosition {
            gldt_returns: 0_u64,
            usdg_available: 50 * E8S,
        }
    );

    assert_matches!(
        withdraw_liquidity(
            &mut env.pic,
            env.principal_ids.user,
            env.canister_ids.usdg_minter,
            &WithdrawArg {
                amount: 50 * E8S,
                maybe_subaccount: None,
            },
        ),
        Ok(4)
    );

    assert_eq!(
        get_lp_position(
            &env.pic,
            env.principal_ids.user,
            env.canister_ids.usdg_minter,
            &None,
        ),
        LiquidationPoolPosition {
            gldt_returns: 0_u64,
            usdg_available: 0,
        }
    );

    assert_eq!(
        icrc1_balance_of(
            &mut env.pic,
            env.principal_ids.user,
            env.canister_ids.usdg_ledger,
            &Account {
                owner: env.principal_ids.user,
                subaccount: None
            }
        ),
        initial_balance
    );
}
