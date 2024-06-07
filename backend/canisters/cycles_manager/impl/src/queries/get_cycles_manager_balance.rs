use crate::state::read_state;
use ic_cdk_macros::query;
pub use types::Cycles;

#[query]
fn get_cycles_manager_balance() -> Cycles {
    read_state(|state| state.metrics().canister_info.cycles_balance)
}
