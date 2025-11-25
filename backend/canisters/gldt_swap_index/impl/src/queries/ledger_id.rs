use crate::state::read_state;
pub use gldt_swap_index_api_canister::ledger_id::Response as LedgerIdResponse;
use ic_cdk::query;

#[query]
pub fn ledger_id() -> LedgerIdResponse {
    let ledger_id = read_state(|state| state.data.ledger_canister_id);

    LedgerIdResponse { ledger_id }
}
