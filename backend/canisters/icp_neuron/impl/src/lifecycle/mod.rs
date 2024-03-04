use crate::ecdsa::{ get_key_id, get_public_key };
use crate::state::mutate_state;
use std::time::Duration;

pub mod init;
pub mod post_upgrade;
pub mod pre_upgrade;

use crate::state::{ init_state, RuntimeState };

pub fn init_canister(runtime_state: RuntimeState) {
    if runtime_state.data.public_key.is_empty() {
        ic_cdk_timers::set_timer(Duration::ZERO, init_public_key);
    }

    crate::jobs::start();
    init_state(runtime_state);
}

fn init_public_key() {
    ic_cdk::spawn(init_public_key_inner());

    async fn init_public_key_inner() {
        let key_id = get_key_id(false);

        if let Ok(public_key) = get_public_key(key_id).await {
            mutate_state(|state| {
                state.data.public_key = public_key;
            });
        }
    }
}
