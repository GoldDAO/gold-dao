use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, State};
use canister_tracing_macros::trace;
pub use cycles_manager_canister::add_canister::Args as AddCanisterArgs;
pub use cycles_manager_canister::add_canister::Response as AddCanisterResponse;
use ic_cdk::api::id;
use ic_cdk_macros::update;
use utils::env::Environment;

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn add_canister(args: AddCanisterArgs) -> AddCanisterResponse {
    mutate_state(|state| add_canister_impl(args, state))
}

fn add_canister_impl(args: AddCanisterArgs, state: &mut State) -> AddCanisterResponse {
    let now = state.env.now();
    let my_id = id().to_string();
    println!("Canister ID: {}", my_id);
    if id() != args.canister_id {
        if state.data.canisters.add(args.canister_id, now) {
            AddCanisterResponse::Success
        } else {
            AddCanisterResponse::AlreadyAdded
        }
    } else {
        AddCanisterResponse::CannotAddSelf
    }
}
