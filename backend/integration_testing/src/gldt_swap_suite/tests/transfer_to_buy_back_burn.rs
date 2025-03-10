use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;

use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use canister_time::HOUR_IN_MS;
    use gldt_swap_common::gldt::{GLDT_LEDGER_FEE_ACCOUNT, GLDT_SWAP_FEE_ACCOUNT, GLDT_TX_FEE};

    use crate::{
        client::{
            gldt_swap::set_buy_back_canister,
            icrc1::{
                client::{balance_of, transfer},
                icrc1_total_supply,
            },
        },
        utils::random_principal,
    };

    use super::*;
    #[test]
    pub fn test_transferring_gldt_fees_to_buy_back_burn() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    gldt_swap,
                    gldt_ledger,
                    ..
                },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        let buyback_burn_account = Account {
            owner: random_principal(),
            subaccount: None,
        };
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive

        // transfer gldt to both ledger fee account and swap fee account ( under the threshold )
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
            5_000_000_000u128, // 50 GLDT ( not enough to trigger threshold )
        )
        .unwrap();

        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
            5_000_000_000u128, // 50 GLDT ( not enough to trigger threshold )
        )
        .unwrap();

        tick_n_blocks(pic, 2);
        let ledger_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        );
        // check transfer occured successfully
        assert_eq!(ledger_fee_account_balance, Nat::from(5_000_000_000u128));
        let swap_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
        );
        // check transfer occured successfully
        assert_eq!(swap_fee_account_balance, Nat::from(5_000_000_000u128));

        // wait for job  - nothing should happen becuase there is no buyback account set and we're under the 10 GLDT threshold
        pic.advance_time(Duration::from_millis(HOUR_IN_MS * 12));
        tick_n_blocks(pic, 2);
        let ledger_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        );
        assert_eq!(ledger_fee_account_balance, Nat::from(5_000_000_000u128));

        // set buyback account
        let res = set_buy_back_canister(
            pic,
            controller,
            gldt_swap,
            &Some(buyback_burn_account.clone()),
        );
        match res {
            gldt_swap_api_canister::set_buy_back_canister::Response::Success => {
                assert_eq!(true, true)
            }
            gldt_swap_api_canister::set_buy_back_canister::Response::InternalError(e) => {
                panic!("{e}");
            }
        }

        // nothing should happen because we haven't met the threshold
        pic.advance_time(Duration::from_millis(HOUR_IN_MS * 12));
        tick_n_blocks(pic, 2);
        let ledger_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        );
        assert_eq!(ledger_fee_account_balance, Nat::from(5_000_000_000u128));

        // transfer more gldt to the fee accounts such that they meet the 10 GLDT threshold
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
            5_000_000_000u128, // 5 GLDT ( not enough to trigger threshold )
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        let ledger_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        );
        // check transfer occured successfully
        assert_eq!(ledger_fee_account_balance, Nat::from(10_000_000_000u128));

        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
            5_000_000_000u128, // 5 GLDT ( not enough to trigger threshold )
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        let swap_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
        );
        // check transfer occured successfully
        assert_eq!(swap_fee_account_balance, Nat::from(10_000_000_000u128));

        let pre_transfer_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        pic.advance_time(Duration::from_millis(HOUR_IN_MS * 24));
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        tick_n_blocks(pic, 2);
        let ledger_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        );
        let swap_fee_account_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
        );
        assert_eq!(ledger_fee_account_balance, Nat::from(GLDT_TX_FEE));
        assert_eq!(swap_fee_account_balance, Nat::from(0u64));
        let post_transfer_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        assert_eq!(pre_transfer_supply, post_transfer_supply);

        let buy_back_burn_balance = balance_of(pic, gldt_ledger, buyback_burn_account);

        assert_eq!(
            buy_back_burn_balance,
            Nat::from(20_000_000_000u64 - GLDT_TX_FEE)
        );
    }

    #[test]
    pub fn test_setting_anon_account() {}
}
