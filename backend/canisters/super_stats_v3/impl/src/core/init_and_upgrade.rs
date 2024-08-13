use candid::{ decode_one, encode_one };
use ic_cdk_macros::{ init, pre_upgrade, post_upgrade };
use ic_stable_memory::{
    stable_memory_init,
    store_custom_data,
    stable_memory_pre_upgrade,
    SBox,
    stable_memory_post_upgrade,
    retrieve_custom_data,
};
pub use super_stats_v3_api::{
    init_and_upgrade::InitArgs,
    runtime::{ Data, RuntimeState, RUNTIME_STATE },
    stable_memory::{ Main, STABLE_STATE },
    stats::constants::STATS_RETURN_LENGTH,
    utils::log,
    working_stats::WorkingStats,
};

#[init]
fn init(args: InitArgs) {
    stable_memory_init();
    // init stable state
    let stable_data = Main::default();
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });

    // init runtime state
    let mut data = Data::default();
    data.add_admin(args.admin.clone());
    data.add_authorised(args.admin);
    data.set_self_id(ic_cdk::api::id());
    data.max_return_length = STATS_RETURN_LENGTH;
    data.test_mode = args.test_mode;
    let stats = WorkingStats::default();
    let runtime_state = RuntimeState { data, stats };
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
    log("Canister Initialised");
}

#[pre_upgrade]
fn pre_upgrade() {
    // Stable Storage
    let state: Main = STABLE_STATE.with(|s| s.borrow_mut().take().unwrap());
    let boxed_state = SBox::new(state).expect("Out of memory");
    store_custom_data(0, boxed_state);

    // Runtime Storage
    let rstate = RUNTIME_STATE.with(|s| { s.borrow_mut().to_owned() });
    let bytes = encode_one(rstate).expect("Unable to candid encode");
    let boxed_bytes = SBox::new(bytes).expect("Out of memory");
    store_custom_data(1, boxed_bytes);

    stable_memory_pre_upgrade().expect("Out of memory");
}

#[post_upgrade]
fn post_upgrade() {
    stable_memory_post_upgrade();
    let state: Main = retrieve_custom_data::<Main>(0).unwrap().into_inner();
    STABLE_STATE.with(|s| {
        *s.borrow_mut() = Some(state);
    });

    // Runtime Storage
    let bytes: Vec<u8> = retrieve_custom_data::<Vec<u8>>(1).unwrap().into_inner();
    let rstate: RuntimeState = decode_one(&bytes).expect("Unable to candid decode");
    RUNTIME_STATE.with(|s| {
        *s.borrow_mut() = rstate;
    });
    log("Canister has been upgraded");
}
