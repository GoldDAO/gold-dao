use crate::state::event::{Event, EventType};
use crate::state::{Account, GoldPrice, State, GLDT, USDG};
use crate::transfer::PendingTransfer;
use crate::transfer::Unit;
use crate::vault::check_vaults;
use crate::vault::{FeeBucket, Vault};
use crate::{Factor, DEFAULT_MEDIUM_RATE, MAXIUM_INTEREST_RATE, MINIMUM_INTEREST_RATE};
use assert_matches::assert_matches;
use candid::Principal;
use proptest::array::uniform32;
use proptest::collection::vec as pvec;
use proptest::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use usdg_minter_api::lifecycle::InitArgument;
use usdg_minter_api::VaultError::BorrowedAmountTooBig;

fn default_state() -> State {
    State::new(InitArgument {
        usdg_ledger_id: Principal::management_canister(),
        gldt_ledger_id: Principal::management_canister(),
        gold_dao_governance_id: Principal::management_canister(),
        xrc_id: Principal::management_canister(),
    })
}

fn default_account() -> Account {
    Account {
        owner: Principal::from_text(
            "5lo5n-u62y5-bemys-zhepa-tz63u-7qe47-wlsa6-5f7ek-rfbwz-xb5re-bae",
        )
        .unwrap(),
        subaccount: None,
    }
}

fn default_account_2() -> Account {
    Account {
        owner: Principal::from_text("jmod6-4iaaa-aaaaq-aadkq-cai").unwrap(),
        subaccount: Some([2_u8; 32]),
    }
}

fn default_account_with_sub(sub: u8) -> Account {
    Account {
        owner: Principal::from_text("jmod6-4iaaa-aaaaq-aadkq-cai").unwrap(),
        subaccount: Some([sub; 32]),
    }
}

#[test]
fn should_dispatch_fees() {
    let mut state = default_state();

    let owner = default_account();
    let margin_amount = GLDT::from_unscaled(100_000);
    let borrowed_amount = USDG::from_e8s(10_000_00_100_001);

    let _ = state.record_vault_creation(owner, borrowed_amount, margin_amount, FeeBucket::Medium);

    state.deposit_liquidity(default_account_2(), USDG::from_unscaled(100));
    state.deposit_liquidity(default_account(), USDG::from_unscaled(100));

    state.charge_fee();

    let lp_balance_2 = state
        .liquidation_pool
        .get(&default_account_2())
        .unwrap_or(&USDG::ZERO)
        .clone();

    assert_eq!(lp_balance_2, USDG::from_e8s(100_51_369_869));

    let lp_balance = state
        .liquidation_pool
        .get(&default_account())
        .unwrap_or(&USDG::ZERO)
        .clone();

    assert_eq!(lp_balance, USDG::from_e8s(100_51_369_868));

    let total_fee_occured = lp_balance_2
        .checked_add(lp_balance)
        .unwrap()
        .checked_sub(USDG::from_unscaled(200))
        .unwrap()
        .checked_add(state.reserve_usdg)
        .unwrap();

    assert_eq!(
        total_fee_occured,
        state
            .get_vault(0)
            .unwrap()
            .borrowed_amount
            .checked_sub(borrowed_amount)
            .unwrap()
    );
}

#[test]
fn should_update_vault() {
    let mut state = default_state();

    let owner = default_account();
    let margin_amount = GLDT::from_unscaled(500);
    let borrowed_amount = USDG::from_unscaled(100);

    let _ = state.record_vault_creation(owner, borrowed_amount, margin_amount, FeeBucket::Medium);

    state.record_update_vault(0, None, Some(FeeBucket::Low));
    assert_eq!(
        state.get_vault(0).unwrap(),
        Vault {
            vault_id: 0,
            owner,
            borrowed_amount,
            margin_amount,
            fee_bucket: FeeBucket::Low,
        }
    );
    state.record_update_vault(0, Some(default_account_2()), Some(FeeBucket::Low));
    assert_eq!(
        state.get_vault(0).unwrap(),
        Vault {
            vault_id: 0,
            owner: default_account_2(),
            borrowed_amount,
            margin_amount,
            fee_bucket: FeeBucket::Low,
        }
    );
    state.record_update_vault(0, None, None);
    assert_eq!(
        state.get_vault(0).unwrap(),
        Vault {
            vault_id: 0,
            owner: default_account_2(),
            borrowed_amount,
            margin_amount,
            fee_bucket: FeeBucket::Low,
        }
    );
}

#[test]
fn should_redeem() {
    let mut state = default_state();

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);

    let _ = state.record_vault_creation(owner, USDG::from_unscaled(100), margin, FeeBucket::Medium);
    let _ = state.record_vault_creation(owner, USDG::from_unscaled(200), margin, FeeBucket::Medium);
    let _ = state.record_vault_creation(owner, USDG::from_unscaled(300), margin, FeeBucket::High);
    let _ = state.record_vault_creation(owner, USDG::from_unscaled(200), margin, FeeBucket::Low);

    assert_eq!(
        state.record_redemption(
            default_account_2(),
            USDG::from_unscaled(100),
            state.one_centigram_of_gold_price
        ),
        GLDT::from_e8s(114_45_783_133)
    );
    assert_eq!(
        state.vault_id_to_vault.get(&3).unwrap().borrowed_amount,
        USDG::from_unscaled(105)
    );
    assert_eq!(state.reserve_usdg, USDG::from_unscaled(5));

    assert_eq!(
        state.record_redemption(
            default_account_2(),
            USDG::from_unscaled(200),
            state.one_centigram_of_gold_price
        ),
        GLDT::from_e8s(228_91_566_265)
    );
    assert_eq!(
        state.vault_id_to_vault.get(&3).unwrap().borrowed_amount,
        USDG::ZERO
    );
    assert_eq!(
        state.vault_id_to_vault.get(&1).unwrap().borrowed_amount,
        USDG::from_unscaled(115)
    );
    assert_eq!(
        state.pending_transfers,
        BTreeMap::from([
            (
                0,
                PendingTransfer {
                    transfer_id: 0,
                    amount: 100_00_000_000,
                    receiver: owner,
                    unit: Unit::USDG,
                }
            ),
            (
                1,
                PendingTransfer {
                    transfer_id: 1,
                    amount: 200_00_000_000,
                    receiver: owner,
                    unit: Unit::USDG,
                }
            ),
            (
                2,
                PendingTransfer {
                    transfer_id: 2,
                    amount: 300_00_000_000,
                    receiver: owner,
                    unit: Unit::USDG,
                }
            ),
            (
                3,
                PendingTransfer {
                    transfer_id: 3,
                    amount: 200_00_000_000,
                    receiver: owner,
                    unit: Unit::USDG,
                }
            ),
            (
                4,
                PendingTransfer {
                    transfer_id: 4,
                    amount: 114_45_783_133,
                    receiver: default_account_2(),
                    unit: Unit::GLDT,
                }
            ),
            (
                5,
                PendingTransfer {
                    transfer_id: 5,
                    amount: 228_91_566_265,
                    receiver: default_account_2(),
                    unit: Unit::GLDT,
                }
            ),
        ])
    );
    assert_eq!(state.reserve_usdg, USDG::from_unscaled(15));
}

#[test]
fn should_order_vaults_as_expected() {
    let mut state = default_state();

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);

    let _ = state.record_vault_creation(owner, USDG::from_unscaled(100), margin, FeeBucket::Medium);
    let _ = state.record_vault_creation(owner, USDG::from_unscaled(200), margin, FeeBucket::Medium);
    let _ = state.record_vault_creation(owner, USDG::from_unscaled(300), margin, FeeBucket::Medium);

    assert_eq!(
        state.get_ordered_vault_ids(state.one_centigram_of_gold_price),
        vec![2, 1, 0]
    );

    let _ = state.record_vault_creation(owner, USDG::from_unscaled(300), margin, FeeBucket::Low);
    let _ = state.record_vault_creation(owner, USDG::from_unscaled(200), margin, FeeBucket::Low);

    assert_eq!(
        state.get_ordered_vault_ids(state.one_centigram_of_gold_price),
        vec![3, 4, 2, 1, 0]
    );

    let _ = state.record_vault_creation(owner, USDG::from_unscaled(300), margin, FeeBucket::High);
    let _ = state.record_vault_creation(owner, USDG::ZERO, margin, FeeBucket::High);

    assert_eq!(
        state.get_ordered_vault_ids(state.one_centigram_of_gold_price),
        vec![3, 4, 2, 1, 0, 5, 6]
    );
}

#[test]
fn should_create_vault() {
    let mut state = default_state();

    assert_eq!(state.vault_id_to_vault.len(), 0);

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    assert_eq!(state.pending_transfers.len(), 1);
    assert_eq!(state.vault_id_to_vault.len(), 1);

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        1
    );

    assert_eq!(state.vault_id_to_vault.len(), 2);
    assert_eq!(state.pending_transfers.len(), 2);
}

#[test]
#[should_panic]
fn should_not_borrow_more_than_max_vault_creation() {
    let mut state = default_state();
    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(1_000);
    let fee_bucket = FeeBucket::Medium;

    state.record_vault_creation(owner, borrowed, margin, fee_bucket);
}

#[test]
fn should_borrow_from_vault() {
    let mut state = default_state();
    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    state.record_borrow_from_vault(0, borrowed);

    assert_eq!(
        state.get_vault(0).unwrap(),
        Vault {
            vault_id: 0,
            owner,
            borrowed_amount: borrowed.checked_mul(Factor::from_unscaled(2)).unwrap(),
            margin_amount: margin,
            fee_bucket,
        }
    );
    assert_eq!(state.pending_transfers.len(), 1);
    assert_eq!(
        state.pending_transfers[&0],
        PendingTransfer {
            transfer_id: 0,
            amount: borrowed.0,
            receiver: owner,
            unit: Unit::USDG,
        }
    );
}

#[test]
fn should_add_margin_to_vault() {
    let mut state = default_state();
    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    state.record_add_margin_to_vault(0, margin);

    assert_eq!(
        state.get_vault(0).unwrap().margin_amount,
        GLDT::from_unscaled(1000)
    );
}

#[test]
#[should_panic]
fn should_not_borrow_more_than_max() {
    let mut state = default_state();
    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    state.record_borrow_from_vault(0, USDG::from_unscaled(500));
}

#[test]
fn should_reject_unvalid_open_vault() {
    let state = default_state();

    let margin = GLDT::from_unscaled(100); // 1g of gold

    let usdg_borrowed = USDG::from_e8s(7_904_761_906);
    assert_matches!(
        state.check_max_borrowable_amount(margin, usdg_borrowed),
        Err(BorrowedAmountTooBig {
            maximum_borrowable_amount: 7_904_761_905,
        },)
    );

    let usdg_borrowed = USDG::from_e8s(7_904_761_905);
    assert_matches!(
        state.check_max_borrowable_amount(margin, usdg_borrowed),
        Ok(())
    );
}

#[test]
fn should_deposit_liquidity() {
    let mut state = default_state();

    let owner = default_account();
    let usdg_borrowed = USDG::from_unscaled(2_000);

    state.deposit_liquidity(owner, usdg_borrowed);
    assert_eq!(
        state
            .liquidation_pool
            .get(&owner)
            .unwrap_or(&USDG::ZERO)
            .clone(),
        usdg_borrowed
    );

    state.deposit_liquidity(owner, usdg_borrowed);
    assert_eq!(
        state
            .liquidation_pool
            .get(&owner)
            .unwrap_or(&USDG::ZERO)
            .clone(),
        USDG::from_unscaled(4_000)
    );

    state.withdraw_liquidity(USDG::from_unscaled(1_000), owner);
    assert_eq!(
        state
            .liquidation_pool
            .get(&owner)
            .unwrap_or(&USDG::ZERO)
            .clone(),
        USDG::from_unscaled(3_000)
    );

    state.withdraw_liquidity(USDG::from_unscaled(3_000), owner);
    assert!(state.liquidation_pool.get(&owner).is_none());
}

#[test]
fn should_claim_returns() {
    let mut state = default_state();

    let owner = default_account_2();
    state
        .liquidation_return
        .insert(owner, GLDT::from_unscaled(100));
    state.record_claimed_returns(owner, GLDT::from_unscaled(50));
    assert_eq!(
        state.liquidation_return.get(&owner).unwrap(),
        &GLDT::from_unscaled(50)
    );
    state.record_claimed_returns(owner, GLDT::from_unscaled(50));
    assert!(state.liquidation_return.get(&owner).is_none());
}

#[test]
fn should_add_and_remove_vault() {
    let mut state = default_state();

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::from([0_u64])
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::from([0_u64])
    );
    assert_eq!(
        state.vault_id_to_vault.get(&0).unwrap(),
        &Vault {
            vault_id: 0,
            owner,
            borrowed_amount: borrowed,
            margin_amount: margin,
            fee_bucket,
        }
    );

    state.remove_vault(0);

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::default()
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::default()
    );
    assert!(state.vault_id_to_vault.get(&0).is_none(),);

    assert_eq!(
        state.pending_transfers,
        BTreeMap::from([(
            0,
            PendingTransfer {
                transfer_id: 0,
                amount: 100_00_000_000,
                receiver: owner,
                unit: Unit::USDG,
            }
        )])
    );
}

#[test]
fn should_close_vault() {
    let mut state = default_state();

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::from([0_u64])
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::from([0_u64])
    );
    assert_eq!(
        state.vault_id_to_vault.get(&0).unwrap(),
        &Vault {
            vault_id: 0,
            owner,
            borrowed_amount: borrowed,
            margin_amount: margin,
            fee_bucket,
        }
    );

    state.record_close_vault(0);

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::default()
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::default()
    );
    assert!(state.vault_id_to_vault.get(&0).is_none(),);

    assert_eq!(
        state.pending_transfers,
        BTreeMap::from([
            (
                0,
                PendingTransfer {
                    transfer_id: 0,
                    amount: 100_00_000_000,
                    receiver: owner,
                    unit: Unit::USDG,
                }
            ),
            (
                1,
                PendingTransfer {
                    transfer_id: 1,
                    amount: 500_00_000_000,
                    receiver: owner,
                    unit: Unit::GLDT,
                }
            ),
        ])
    );
}

#[test]
fn should_liquidate_vault() {
    let mut state = default_state();

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(395);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    assert_eq!(state.total_gldt_margin(), GLDT::from_unscaled(500));
    assert_eq!(state.total_usdg_debt(), USDG::from_unscaled(395));

    check_vaults(&mut state);

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::from([0_u64])
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::from([0_u64])
    );
    assert_eq!(
        state.vault_id_to_vault.get(&0).unwrap(),
        &Vault {
            vault_id: 0,
            owner,
            borrowed_amount: borrowed,
            margin_amount: margin,
            fee_bucket,
        }
    );

    state.deposit_liquidity(owner, USDG::from_e8s(1_470_00_000_012));
    state.deposit_liquidity(default_account_2(), USDG::from_e8s(529_99_999_988));

    assert_eq!(
        state.total_usdg_in_liquidation_pool(),
        USDG::from_unscaled(2_000)
    );

    state.one_centigram_of_gold_price = GoldPrice::from_e8s(43_000_000);

    check_vaults(&mut state);

    assert_eq!(
        state.total_usdg_in_liquidation_pool(),
        USDG::from_unscaled(1_605)
    );
    assert_eq!(state.total_gldt_in_returns(), GLDT::from_unscaled(500));
    assert_eq!(state.total_gldt_margin(), GLDT::ZERO);
    assert_eq!(state.total_usdg_debt(), USDG::ZERO);

    assert_eq!(
        state.liquidation_pool,
        BTreeMap::from([
            (default_account_2(), USDG::from_e8s(425_32_499_988_u64)),
            (default_account(), USDG::from_e8s(1_179_67_500_012_u64))
        ])
    );

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::default()
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::default()
    );
    assert!(state.vault_id_to_vault.get(&0).is_none(),);

    assert_eq!(
        state.pending_transfers,
        BTreeMap::from([(
            0,
            PendingTransfer {
                transfer_id: 0,
                amount: 395_00_000_000,
                receiver: owner,
                unit: Unit::USDG,
            }
        )])
    );
}

#[test]
fn should_redistribute_vault() {
    let mut state = default_state();

    let owner = default_account();
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(333);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    assert_eq!(
        state.record_vault_creation(owner, borrowed, GLDT::from_unscaled(1_000), fee_bucket),
        1
    );
    assert_eq!(
        state.record_vault_creation(owner, borrowed, GLDT::from_unscaled(1_000), fee_bucket),
        2
    );
    assert_eq!(
        state.record_vault_creation(owner, borrowed, GLDT::from_unscaled(1_000), fee_bucket),
        3
    );

    check_vaults(&mut state);

    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::from([0_u64, 1_u64, 2_u64, 3_u64])
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::from([0_u64, 1_u64, 2_u64, 3_u64])
    );
    assert_eq!(
        state.vault_id_to_vault,
        BTreeMap::from([
            (
                0_u64,
                Vault {
                    vault_id: 0,
                    owner,
                    borrowed_amount: borrowed,
                    margin_amount: margin,
                    fee_bucket,
                }
            ),
            (
                1_u64,
                Vault {
                    vault_id: 1,
                    owner,
                    borrowed_amount: borrowed,
                    margin_amount: GLDT::from_unscaled(1_000),
                    fee_bucket,
                }
            ),
            (
                2_u64,
                Vault {
                    vault_id: 2,
                    owner,
                    borrowed_amount: borrowed,
                    margin_amount: GLDT::from_unscaled(1_000),
                    fee_bucket,
                }
            ),
            (
                3_u64,
                Vault {
                    vault_id: 3,
                    owner,
                    borrowed_amount: borrowed,
                    margin_amount: GLDT::from_unscaled(1_000),
                    fee_bucket,
                }
            ),
        ])
    );

    state.one_centigram_of_gold_price = GoldPrice::from_e8s(63_000_000);

    assert_eq!(state.total_gldt_margin(), GLDT::from_unscaled(3_500));
    assert_eq!(state.total_usdg_debt(), USDG::from_unscaled(1_332));

    check_vaults(&mut state);
    assert_eq!(
        state.fee_bucket_to_vault_ids.get(&fee_bucket).unwrap(),
        &BTreeSet::from([1_u64, 2_u64, 3_u64])
    );
    assert_eq!(
        state.account_to_vault_ids.get(&owner).unwrap(),
        &BTreeSet::from([1_u64, 2_u64, 3_u64])
    );

    assert_eq!(state.total_gldt_margin(), GLDT::from_unscaled(3_500));
    assert_eq!(state.total_usdg_debt(), USDG::from_unscaled(1_332));

    assert_eq!(
        state.vault_id_to_vault,
        BTreeMap::from([
            (
                1_u64,
                Vault {
                    vault_id: 1,
                    owner,
                    borrowed_amount: USDG::from_e8s(443_99_999_889),
                    margin_amount: GLDT::from_e8s(1_166_66_666_500),
                    fee_bucket,
                }
            ),
            (
                2_u64,
                Vault {
                    vault_id: 2,
                    owner,
                    borrowed_amount: USDG::from_e8s(443_99_999_889),
                    margin_amount: GLDT::from_e8s(1_166_66_666_500),
                    fee_bucket,
                }
            ),
            (
                3_u64,
                Vault {
                    vault_id: 3,
                    owner,
                    borrowed_amount: USDG::from_e8s(444_00_000_222),
                    margin_amount: GLDT::from_e8s(1_166_66_667_000),
                    fee_bucket,
                }
            ),
        ])
    );
}

#[test]
fn should_adjust_interest_scenario_0() {
    let mut state = default_state();
    state
        .interest_rates
        .insert(FeeBucket::High, MAXIUM_INTEREST_RATE);
    state
        .interest_rates
        .insert(FeeBucket::Low, MINIMUM_INTEREST_RATE);

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        MINIMUM_INTEREST_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
        DEFAULT_MEDIUM_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        MAXIUM_INTEREST_RATE
    );

    let owner = default_account();
    let margin = GLDT::from_unscaled(100_000);
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(10_000), margin, FeeBucket::Low),
        0
    );
    assert_eq!(
        state.record_vault_creation(
            owner,
            USDG::from_unscaled(24_000),
            margin,
            FeeBucket::Medium
        ),
        1
    );
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(50_000), margin, FeeBucket::High),
        2
    );

    assert_eq!(state.get_pull_factor(), 0.47619047619047616);

    for _ in 0..100 {
        state.update_interest_rate();
    }

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.04840829192465135
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
        DEFAULT_MEDIUM_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.9999999999999999
    );

    state.charge_fee();

    assert_eq!(
        state.vault_id_to_vault,
        BTreeMap::from([
            (
                0,
                Vault {
                    vault_id: 0,
                    owner,
                    borrowed_amount: USDG::from_e8s(10_001_32_625_457),
                    margin_amount: margin,
                    fee_bucket: FeeBucket::Low,
                }
            ),
            (
                1,
                Vault {
                    vault_id: 1,
                    owner,
                    borrowed_amount: USDG::from_e8s(24_003_28_767_123),
                    margin_amount: margin,
                    fee_bucket: FeeBucket::Medium,
                }
            ),
            (
                2,
                Vault {
                    vault_id: 2,
                    owner,
                    borrowed_amount: USDG::from_e8s(50_136_98_630_136),
                    margin_amount: margin,
                    fee_bucket: FeeBucket::High,
                }
            ),
        ])
    );

    assert_eq!(state.reserve_usdg, 141_60_022_716_u64.into());
}

#[test]
fn should_adjust_interest_scenario_1() {
    let mut state = default_state();

    state
        .interest_rates
        .insert(FeeBucket::High, MAXIUM_INTEREST_RATE);
    state
        .interest_rates
        .insert(FeeBucket::Low, MINIMUM_INTEREST_RATE);

    let owner = default_account();
    let margin = GLDT::from_unscaled(100_000);
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(50_000), margin, FeeBucket::Low),
        0
    );
    assert_eq!(
        state.record_vault_creation(
            owner,
            USDG::from_unscaled(24_000),
            margin,
            FeeBucket::Medium
        ),
        1
    );
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(10_000), margin, FeeBucket::High),
        2
    );

    assert_eq!(state.get_pull_factor(), -0.47619047619047616);

    state.update_interest_rate();

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.010000000000000004
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
        DEFAULT_MEDIUM_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.09523809523809544
    );

    for _ in 0..100 {
        state.update_interest_rate();
    }

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.010000000000000004
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
        DEFAULT_MEDIUM_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.050175464160250705
    );
}

#[test]
fn should_adjust_interest_scenario_2() {
    let mut state = default_state();

    state
        .interest_rates
        .insert(FeeBucket::High, DEFAULT_MEDIUM_RATE);
    state
        .interest_rates
        .insert(FeeBucket::Low, MINIMUM_INTEREST_RATE);

    let owner = default_account();
    let margin = GLDT::from_unscaled(100_000);
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(10_000), margin, FeeBucket::Low),
        0
    );
    assert_eq!(
        state.record_vault_creation(
            owner,
            USDG::from_unscaled(24_000),
            margin,
            FeeBucket::Medium
        ),
        1
    );
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(50_000), margin, FeeBucket::High),
        2
    );

    assert_eq!(state.get_pull_factor(), 0.47619047619047616);

    for _ in 0..100 {
        state.update_interest_rate();
    }

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.04840829192465135
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
        DEFAULT_MEDIUM_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.8575332998211325
    );
}

#[test]
fn should_adjust_interest_scenario_3() {
    let mut state = default_state();

    state
        .interest_rates
        .insert(FeeBucket::High, DEFAULT_MEDIUM_RATE);
    state
        .interest_rates
        .insert(FeeBucket::Low, MINIMUM_INTEREST_RATE);

    let owner = default_account();
    let margin = GLDT::from_unscaled(100_000);
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(10_000), margin, FeeBucket::Low),
        0
    );
    assert_eq!(
        state.record_vault_creation(
            owner,
            USDG::from_unscaled(24_000),
            margin,
            FeeBucket::Medium
        ),
        1
    );
    assert_eq!(
        state.record_vault_creation(owner, USDG::from_unscaled(50_000), margin, FeeBucket::High),
        2
    );

    assert_eq!(state.get_pull_factor(), 0.47619047619047616);

    for _ in 0..24 {
        state.update_interest_rate();
    }

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.02184522811328816
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
        DEFAULT_MEDIUM_RATE
    );
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.13901719125399048
    );

    state.interest_rates.insert(FeeBucket::Medium, 0.15);

    state.update_interest_rate();

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.06729296266655824
    );
    assert_eq!(*state.interest_rates.get(&FeeBucket::Medium).unwrap(), 0.15);
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.4341503468748059
    );

    for _ in 0..50 {
        state.update_interest_rate();
    }

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.13502052660942984
    );
    assert_eq!(*state.interest_rates.get(&FeeBucket::Medium).unwrap(), 0.15);
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.8946126169337145
    );

    state.interest_rates.insert(FeeBucket::Medium, 0.1);
    state.update_interest_rate();

    assert_eq!(
        *state.interest_rates.get(&FeeBucket::Low).unwrap(),
        0.09044173443496947
    );
    assert_eq!(*state.interest_rates.get(&FeeBucket::Medium).unwrap(), 0.1);
    assert_eq!(
        *state.interest_rates.get(&FeeBucket::High).unwrap(),
        0.5994014551798327
    );
}

prop_compose! {
    fn arb_init_arg()(
        usdg_ledger_id in arb_principal(),
        gldt_ledger_id in arb_principal(),
        gold_dao_governance_id in arb_principal(),
        xrc_id in arb_principal(),
    ) -> EventType {
        EventType::Init {
            usdg_ledger_id,
            gldt_ledger_id,
            gold_dao_governance_id,
            xrc_id
        }
    }
}

prop_compose! {
    fn arb_upgrade_arg()(
        new_medium_fee_percent in proptest::option::of(any::<u64>()),
    ) -> EventType {
        EventType::Upgrade {
            new_medium_fee_percent,
        }
    }
}

prop_compose! {
    fn arb_open_vault()(
        owner in arb_account(),
        margin_amount in arb_gldt_amount(),
        block_index in any::<u64>(),
        borrowed_amount in arb_usdg_amount(),
        fee_bucket in arb_fee_bucket()
    ) -> EventType {
        EventType::OpenVault {
            owner,
            margin_amount,
            borrowed_amount,
            fee_bucket,
            block_index,
        }
    }
}

prop_compose! {
    fn arb_borrow()(
        vault_id in any::<u64>(),
        block_index in any::<u64>(),
        borrowed_amount in arb_usdg_amount(),
    ) -> EventType {
        EventType::Borrow {
            vault_id,
            borrowed_amount,
            block_index,
        }
    }
}

prop_compose! {
    fn arb_add_margin()(
        vault_id in any::<u64>(),
        block_index in any::<u64>(),
        margin_added in arb_gldt_amount(),
    ) -> EventType {
        EventType::AddMargin {
            vault_id,
            margin_added,
            block_index,
        }
    }
}

prop_compose! {
    fn arb_repay()(
        vault_id in any::<u64>(),
        block_index in any::<u64>(),
        debt in arb_usdg_amount(),
    ) -> EventType {
        EventType::Repay {
            vault_id,
            debt,
            block_index,
        }
    }
}

prop_compose! {
    fn arb_close_vault()(
        vault_id in any::<u64>(),
        block_index in proptest::option::of(any::<u64>()),
    ) -> EventType {
        EventType::Close {
            vault_id,
            block_index,
        }
    }
}

prop_compose! {
    fn arb_transfer_executed()(
        block_index in any::<u64>(),
        transfer_id in  any::<u64>(),
    ) -> EventType {
        EventType::TransferExecuted {
            transfer_id,
            block_index,
        }
    }
}

prop_compose! {
    fn arb_provide_liquidity()(
        caller in arb_account(),
        amount in arb_usdg_amount(),
        block_index in any::<u64>(),
    ) -> EventType {
        EventType::DepositLiquidity {
            caller,
            block_index,
            amount,
        }
    }
}

prop_compose! {
    fn arb_withdraw_liquidity()(
        caller in arb_account(),
        amount in arb_usdg_amount(),
        block_index in any::<u64>(),
    ) -> EventType {
        EventType::WithdrawLiquidity {
            caller,
            block_index,
            amount,
        }
    }
}

prop_compose! {
    fn arb_claim_returns()(
        caller in arb_account(),
        amount in arb_gldt_amount(),
        block_index in any::<u64>(),
    ) -> EventType {
        EventType::ClaimReturns {
            caller,
            block_index,
            amount,
        }
    }
}

prop_compose! {
    fn arb_redemption_on_vaults()(
        owner in arb_account(),
        amount in arb_usdg_amount(),
        block_index in any::<u64>(),
        current_rate in any::<u64>(),
    ) -> EventType {
        EventType::Redeem {
            owner,
            block_index,
            amount,
            current_rate: GoldPrice::from_e8s(current_rate)
        }
    }
}

prop_compose! {
    fn arb_liquidate_vault()(
        vault_id in any::<u64>(),
    ) -> EventType {
        EventType::Liquidate {
            vault_id,
        }
    }
}

prop_compose! {
    fn arb_redistribute_on_vault()(
        vault_id in any::<u64>(),
    ) -> EventType {
        EventType::Redistribute {
            vault_id,
        }
    }
}

prop_compose! {
    fn arb_update_vault()(
        vault_id in any::<u64>(),
        new_owner in proptest::option::of(arb_account()),
        fee_bucket in proptest::option::of(arb_fee_bucket()),
    ) -> EventType {
        EventType::UpdateVault {
            vault_id,
            new_owner,
            fee_bucket
        }
    }
}

prop_compose! {
    fn arb_account()(
        owner in arb_principal(),
        subaccount in arb_subaccount(),
    ) -> Account {
        Account {
            owner,
            subaccount: subaccount,
        }
    }
}

fn arb_fee_bucket() -> impl Strategy<Value = FeeBucket> {
    prop_oneof![
        Just(FeeBucket::Low),
        Just(FeeBucket::Medium),
        Just(FeeBucket::High),
    ]
}

fn arb_principal() -> impl Strategy<Value = Principal> {
    pvec(any::<u8>(), 0..=29).prop_map(|bytes| Principal::from_slice(&bytes))
}

fn arb_subaccount() -> impl Strategy<Value = Option<[u8; 32]>> {
    proptest::option::of(uniform32(any::<u8>()))
}

fn arb_usdg_amount() -> impl Strategy<Value = USDG> {
    (0..10_000_000_000_000_000_u64).prop_map(|a| USDG::from_e8s(a))
}

fn arb_gldt_amount() -> impl Strategy<Value = GLDT> {
    (0..10_000_000_000_000_000_u64).prop_map(|a| GLDT::from_e8s(a))
}

fn arb_event_type() -> impl Strategy<Value = EventType> {
    prop_oneof![
        arb_init_arg(),
        arb_upgrade_arg(),
        arb_open_vault(),
        arb_borrow(),
        arb_repay(),
        arb_add_margin(),
        arb_close_vault(),
        arb_transfer_executed(),
        arb_provide_liquidity(),
        arb_withdraw_liquidity(),
        arb_claim_returns(),
        arb_redemption_on_vaults(),
        arb_liquidate_vault(),
        arb_redistribute_on_vault(),
        arb_update_vault()
    ]
}

fn arb_event() -> impl Strategy<Value = Event> {
    (any::<u64>(), arb_event_type()).prop_map(|(timestamp, payload)| Event { timestamp, payload })
}

proptest! {
    #[test]
    fn event_encoding_roundtrip(event in arb_event()) {
        use ic_stable_structures::storable::Storable;
        let bytes = event.to_bytes();
        prop_assert_eq!(&event, &Event::from_bytes(bytes.clone()), "failed to decode bytes {}", hex::encode(bytes));
    }

    #[test]
    fn should_not_borrow_more_than_maximum(usdg_borrowed in arb_usdg_amount()) {
        let state = default_state();

        let margin = GLDT::from_unscaled(100); // 1g of gold

        let max_borrowable_amount = USDG::from_e8s(7_904_761_905); // 79 USDG
        if usdg_borrowed > max_borrowable_amount {
            assert_matches!(
                state.check_max_borrowable_amount(margin, usdg_borrowed),
                Err(BorrowedAmountTooBig {
                    maximum_borrowable_amount: 7_904_761_905,
                },)
            );
        } else {
            assert_matches!(
                state.check_max_borrowable_amount(margin, usdg_borrowed),
                Ok(())
            );
        }
    }

    #[test]
    fn should_sum_usdg_in_bucket(vault_count in 0..100_u64, borrowed in arb_usdg_amount()) {
        let mut state = default_state();

        for index in 0..vault_count {
            let owner = Account {
                owner: Principal::from_text(
                    "5lo5n-u62y5-bemys-zhepa-tz63u-7qe47-wlsa6-5f7ek-rfbwz-xb5re-bae",
                )
                .unwrap(),
                subaccount: None,
            };
            let margin = GLDT::from_e8s(u64::MAX);
            assert_eq!(state.record_vault_creation(owner, borrowed, margin, FeeBucket::Low), index * 2 + index);
            assert_eq!(state.record_vault_creation(owner, borrowed, margin, FeeBucket::Medium), index * 2 + index + 1);
            assert_eq!(state.record_vault_creation(owner, borrowed, margin, FeeBucket::High), index * 2 + index + 2);
        }
        assert_eq!(state.sum_usdg_by_fee_bucket(FeeBucket::Low), USDG::from_e8s(borrowed.0 * vault_count as u64));
    }

    #[test]
    fn should_awlays_have_in_bound_rates(
        low in arb_usdg_amount(),
        medium in arb_usdg_amount(),
        high in arb_usdg_amount()
    ) {
        let mut state = default_state();

        let owner = default_account();
        let margin = GLDT::from_e8s(u64::MAX);
        assert_eq!(
            state.record_vault_creation(owner, low, margin, FeeBucket::Low),
            0
        );
        assert_eq!(
            state.record_vault_creation(owner, medium, margin, FeeBucket::Medium),
            1
        );
        assert_eq!(
            state.record_vault_creation(owner, high, margin, FeeBucket::High),
            2
        );

        for _ in 0..100 {
            state.update_interest_rate();
        }

        let low_rate = *state.interest_rates.get(&FeeBucket::Low).unwrap();

        assert!(low_rate <= DEFAULT_MEDIUM_RATE && DEFAULT_MEDIUM_RATE >= MINIMUM_INTEREST_RATE);
        assert_eq!(
            *state.interest_rates.get(&FeeBucket::Medium).unwrap(),
            DEFAULT_MEDIUM_RATE
        );
        let high_rate = *state.interest_rates.get(&FeeBucket::High).unwrap();
        assert!(high_rate >= DEFAULT_MEDIUM_RATE && DEFAULT_MEDIUM_RATE <= MAXIUM_INTEREST_RATE);
    }

    #[test]
    fn should_always_liquidate_correct_debt_amount(usdg_borrowed in arb_usdg_amount()) {
        let mut state = default_state();

        let owner = default_account();
        let margin = GLDT::from_e8s(usdg_borrowed.0 * 2);
        let fee_bucket = FeeBucket::Medium;

        assert_eq!(
            state.record_vault_creation(owner, usdg_borrowed, margin, fee_bucket),
            0
        );

        state.deposit_liquidity(owner, usdg_borrowed);
        state.deposit_liquidity(default_account_2(), usdg_borrowed);

        state.one_centigram_of_gold_price = GoldPrice::from_e8s(43_000_000);

        check_vaults(&mut state);

        assert_eq!(state.total_gldt_margin(), GLDT::ZERO);
        assert_eq!(state.total_usdg_debt(), USDG::ZERO);
        assert_eq!(state.total_usdg_in_liquidation_pool(), usdg_borrowed);
        assert_eq!(state.total_gldt_in_returns(), margin);
    }

    #[test]
    fn should_always_liquidate_correct_margin_amount(
        margin_amount in 0..10_000_000_000_000_000_u64
    ) {
        let mut state = default_state();

        let owner = default_account();
        let margin = GLDT::from_e8s(margin_amount);
        let borrowed = USDG::from_e8s(margin_amount / 2);
        let fee_bucket = FeeBucket::Medium;

        assert_eq!(
            state.record_vault_creation(owner, borrowed, margin, fee_bucket),
            0
        );

        state.deposit_liquidity(owner, borrowed);
        state.deposit_liquidity(default_account_2(), borrowed);

        state.one_centigram_of_gold_price = GoldPrice::from_e8s(43_000_000);

        check_vaults(&mut state);

        assert_eq!(state.total_gldt_margin(), GLDT::ZERO);
        assert_eq!(state.total_usdg_debt(), USDG::ZERO);
        assert_eq!(state.total_usdg_in_liquidation_pool(), borrowed);
        assert_eq!(state.total_gldt_in_returns(), margin);
    }

    #[test]
    fn should_always_redistribute_correct_margin_amount(
        margin_amount in 100_00_000_000_u64..10_000_000_000_000_000_u64
    ) {
        let mut state = default_state();

        let owner = default_account();
        let margin = GLDT::from_e8s(margin_amount);
        let borrowed = USDG::from_e8s(margin_amount / 2);
        let bigger_borrowed = USDG::from_e8s(margin_amount)
            .checked_sub(USDG::from_e8s(margin_amount / 3))
            .unwrap();
        let fee_bucket = FeeBucket::Medium;

        assert_eq!(
            state.record_vault_creation(owner, bigger_borrowed, margin, fee_bucket),
            0
        );
        assert_eq!(
            state.record_vault_creation(owner, borrowed, margin, fee_bucket),
            1
        );
        assert_eq!(
            state.record_vault_creation(owner, borrowed, margin, fee_bucket),
            2
        );
        assert_eq!(
            state.record_vault_creation(owner, borrowed, margin, fee_bucket),
            3
        );

        state.one_centigram_of_gold_price = GoldPrice::from_e8s(43_000_000);

        check_vaults(&mut state);

        let expected_borrowed = borrowed
            .checked_add(borrowed)
            .unwrap()
            .checked_add(borrowed)
            .unwrap()
            .checked_add(bigger_borrowed)
            .unwrap();

        assert_eq!(state.total_gldt_margin(), GLDT::from_e8s(4 * margin_amount));
        assert_eq!(state.total_usdg_debt(), expected_borrowed);
        assert_eq!(state.total_usdg_in_liquidation_pool(), USDG::ZERO);
        assert_eq!(state.total_gldt_in_returns(), GLDT::ZERO);
    }

    #[test]
    fn should_keep_track_of_fees(
        borrowed_amount in arb_usdg_amount(),
        provided_liquidity_1 in arb_usdg_amount(),
        provided_liquidity_2 in arb_usdg_amount(),
        provided_liquidity_3 in arb_usdg_amount(),
    ) {
        let mut state = default_state();

        let owner = default_account();
        let margin_amount = GLDT::from_e8s(u64::MAX);

        let _ = state.record_vault_creation(owner, borrowed_amount, margin_amount, FeeBucket::Medium);

        state.deposit_liquidity(default_account_2(), provided_liquidity_1);
        state.deposit_liquidity(default_account(), provided_liquidity_2);
        state.deposit_liquidity(default_account_with_sub(9), provided_liquidity_3);

        state.charge_fee();

        let lp_balance_2 = state
            .liquidation_pool
            .get(&default_account_2())
            .unwrap_or(&USDG::ZERO)
            .clone();

        let lp_balance = state
            .liquidation_pool
            .get(&default_account())
            .unwrap_or(&USDG::ZERO)
            .clone();

        let lp_balance_3 = state
            .liquidation_pool
            .get(&default_account_with_sub(9))
            .unwrap_or(&USDG::ZERO)
            .clone();

        let total_fee_occured = lp_balance_2
            .checked_add(lp_balance)
            .unwrap()
            .checked_add(lp_balance_3)
            .unwrap()
            .checked_sub(provided_liquidity_1.
                checked_add(provided_liquidity_2)
                .unwrap()
                .checked_add(provided_liquidity_3)
                .unwrap())
            .unwrap()
            .checked_add(state.reserve_usdg)
            .unwrap();

        assert_eq!(
            total_fee_occured,
            state
                .get_vault(0)
                .unwrap()
                .borrowed_amount
                .checked_sub(borrowed_amount)
                .unwrap()
        );
    }
}
