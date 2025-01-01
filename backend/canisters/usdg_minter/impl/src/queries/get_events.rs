use crate::state::event::event_to_candid_event;
use ic_cdk::query;
use usdg_minter_api::queries::get_events::{GetEventsArg, GetEventsResult};

#[query]
fn get_events(args: GetEventsArg) -> GetEventsResult {
    if ic_cdk::api::data_certificate().is_none() {
        ic_cdk::trap("update call rejected");
    }

    const MAX_EVENTS_PER_QUERY: u64 = 100;
    let events = crate::memory::with_event_iter(|it| {
        it.skip(args.start as usize)
            .take(args.length.min(MAX_EVENTS_PER_QUERY) as usize)
            .map(event_to_candid_event)
            .collect()
    });
    GetEventsResult {
        events,
        total_event_count: crate::memory::total_event_count(),
    }
}
