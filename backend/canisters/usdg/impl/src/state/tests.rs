use crate::state::{Account, State, GLDT, USDG};
use crate::vault::FeeBucket;
use candid::Principal;

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
