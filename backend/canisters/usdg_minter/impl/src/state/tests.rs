use crate::state::{Account, State, GLDT, USDG};
use crate::vault::FeeBucket;
use crate::DEFAULT_GOLD_PRICE;
use assert_matches::assert_matches;
use candid::Principal;
use proptest::prelude::*;
use usdg_minter_api::VaultError::BorrowedAmountTooBig;

fn default_state() -> State {
    State {
        next_vault_id: 0,
        next_transfer_id: 0,

        // Vault related fields
        fee_bucket_to_vault_ids: Default::default(),
        account_to_vault_ids: Default::default(),

        vault_id_to_vault: Default::default(),

        // Pending transfers
        pending_transfers: Default::default(),

        one_centigram_of_gold_price: DEFAULT_GOLD_PRICE,

        // Canister ids
        usdg_ledger_id: Principal::management_canister(),
        gldt_ledger_id: Principal::management_canister(),
        gold_dao_governance_id: Principal::management_canister(),
        xrc_id: Principal::management_canister(),
    }
}

#[test]
fn should_create_vault() {
    let mut state = default_state();

    assert_eq!(state.active_vault_count(), 0);

    let owner = Account {
        owner: Principal::from_text(
            "5lo5n-u62y5-bemys-zhepa-tz63u-7qe47-wlsa6-5f7ek-rfbwz-xb5re-bae",
        )
        .unwrap(),
        subaccount: None,
    };
    let margin = GLDT::from_unscaled(500);
    let borrowed = USDG::from_unscaled(100);
    let fee_bucket = FeeBucket::Medium;

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        0
    );

    assert_eq!(state.active_vault_count(), 1);

    assert_eq!(
        state.record_vault_creation(owner, borrowed, margin, fee_bucket),
        1
    );

    assert_eq!(state.active_vault_count(), 2);
    assert_eq!(state.pending_transfers.len(), 2);
}

#[test]
fn should_reject_unvalid_open_vault() {
    let state = default_state();

    let margin = GLDT::from_unscaled(100); // 1g of gold

    let usdg_borrowed = USDG::from_e8s(890_047_619);
    assert_matches!(
        state.check_open_vault_args_validity(margin, usdg_borrowed),
        Err(BorrowedAmountTooBig {
            maximum_borrowable_amount: 790476190,
        },)
    );

    let usdg_borrowed = USDG::from_e8s(100_047_619);
    assert_matches!(
        state.check_open_vault_args_validity(margin, usdg_borrowed),
        Ok(())
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

        let max_borrowable_amount = USDG::from_e8s(790476190); // 79 USDG
        if usdg_borrowed > max_borrowable_amount {
            assert_matches!(
                state.check_open_vault_args_validity(margin, usdg_borrowed),
                Err(BorrowedAmountTooBig {
                    maximum_borrowable_amount: 790476190,
                },)
            );
        } else {
            assert_matches!(
                state.check_open_vault_args_validity(margin, usdg_borrowed),
                Ok(())
            );
        }
    }
}
