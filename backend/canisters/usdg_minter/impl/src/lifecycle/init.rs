use crate::lifecycle::timer::setup_timers;
use crate::memory::record_event;
use crate::state::event::EventType;
use crate::state::{replace_state, State};
use ic_cdk::init;
use usdg_minter_api::lifecycle::MinterArgument;

#[init]
fn init(args: MinterArgument) {
    match args {
        MinterArgument::Init(init_arg) => {
            record_event(
                EventType::Init {
                    usdg_ledger_id: init_arg.usdg_ledger_id,
                    gldt_ledger_id: init_arg.gldt_ledger_id,
                    gold_dao_governance_id: init_arg.gold_dao_governance_id,
                    xrc_id: init_arg.xrc_id,
                },
                ic_cdk::api::time(),
            );
            replace_state(State::new(init_arg));
        }
        MinterArgument::Upgrade(_) => {
            panic!("expected init args got upgrade args");
        }
    }

    setup_timers();
}
