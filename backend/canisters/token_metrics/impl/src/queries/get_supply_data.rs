use ic_cdk_macros::query;

use crate::state::{ read_state, TokenSupplyData };

#[query]
fn get_supply_data() -> TokenSupplyData {
    read_state(|state| state.data.supply_data.clone())
}
