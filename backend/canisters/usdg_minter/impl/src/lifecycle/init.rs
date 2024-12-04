use crate::state::{replace_state, State};
use ic_cdk::init;
use usdg_minter_api::lifecycle::MinterArgument;

#[init]
fn init(args: MinterArgument) {
    match args {
        MinterArgument::Init(init_arg) => {
            replace_state(State::new(init_arg));
        }
        MinterArgument::Upgrade(_) => {
            panic!("expected init args got upgrade args");
        }
    }
}
