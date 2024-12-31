use crate::state::{Account, State, GLDT, USDG};
use crate::transfer::PendingTransfer;
use crate::transfer::Unit;
use crate::vault::{FeeBucket, Vault};
use crate::{Factor, DEFAULT_MEDIUM_RATE, MAXIUM_INTEREST_RATE, MINIMUM_INTEREST_RATE};
use assert_matches::assert_matches;
use candid::Principal;
use proptest::prelude::*;
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
        state.get_vault(0).unwrap(),
        Vault {
            vault_id: 0,
            owner,
            borrowed_amount: USDG::from_e8s(10_001_32_625_457),
            margin_amount: margin,
            fee_bucket: FeeBucket::Low,
        }
    );
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

fn arb_usdg_amount() -> impl Strategy<Value = USDG> {
    (0..10_000_000_000_000_000_u64).prop_map(|a| USDG::from_e8s(a))
}

proptest! {
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
    fn should_awlays_have_in_bound_rates(low in arb_usdg_amount(), medium in arb_usdg_amount(), high in arb_usdg_amount()) {
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
}
