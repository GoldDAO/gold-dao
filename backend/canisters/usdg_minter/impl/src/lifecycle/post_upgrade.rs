use crate::lifecycle::timer::setup_timers;
use crate::logs::INFO;
use crate::memory::total_event_count;
use crate::state::audit::{process_event, replay_events};
use crate::state::event::EventType;
use crate::state::{mutate_state, replace_state};
use crate::MinterArgument;
use ic_canister_log::log;
use ic_cdk::post_upgrade;

#[post_upgrade]
fn post_upgrade(arg: MinterArgument) {
    let start = ic_cdk::api::instruction_counter();

    log!(INFO, "[upgrade]: replaying {} events", total_event_count());

    let upgrade_args = match arg {
        MinterArgument::Init(_) => ic_cdk::trap("expected Upgrade got Init"),
        MinterArgument::Upgrade(upgrade_args) => {
            log!(
                INFO,
                "[upgrade]: updating configuration with {:?}",
                upgrade_args
            );
            upgrade_args
        }
    };

    let state = replay_events();

    replace_state(state);

    mutate_state(|s| {
        process_event(
            s,
            EventType::Upgrade {
                new_medium_fee_percent: upgrade_args.new_medium_fee_percent,
            },
        )
    });

    let end = ic_cdk::api::instruction_counter();

    log!(
        INFO,
        "[upgrade]: replaying events consumed {} instructions",
        end - start
    );

    setup_timers();
}
