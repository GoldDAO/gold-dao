use candid::Principal;
use serde::{ Deserialize, Serialize };
use types::{ CanisterId, Cycles, TimestampMillis, TimestampNanos };
use canister_time::now_nanos;

#[derive(Default, Serialize, Deserialize)]
pub struct CanisterEnv {
    test_mode: bool,
}

pub trait Environment {
    fn now_nanos(&self) -> TimestampNanos;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> CanisterId;
    fn cycles_balance(&self) -> Cycles;

    fn now(&self) -> TimestampMillis {
        self.now_nanos() / 1_000_000
    }
    fn cycles_balance_in_tc(&self) -> f64 {
        (self.cycles_balance() as f64) / 1_000_000_000_000.0
    }
}

impl CanisterEnv {
    pub fn new(test_mode: bool) -> Self {
        Self {
            test_mode,
        }
    }

    pub fn is_test_mode(&self) -> bool {
        self.test_mode
    }
}

impl Environment for CanisterEnv {
    fn now_nanos(&self) -> TimestampNanos {
        now_nanos()
    }

    #[cfg(target_arch = "wasm32")]
    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn caller(&self) -> Principal {
        Principal::anonymous()
    }

    #[cfg(target_arch = "wasm32")]
    fn canister_id(&self) -> CanisterId {
        ic_cdk::id()
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn canister_id(&self) -> CanisterId {
        Principal::anonymous()
    }

    #[cfg(target_arch = "wasm32")]
    fn cycles_balance(&self) -> Cycles {
        ic_cdk::api::canister_balance().into()
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn cycles_balance(&self) -> Cycles {
        0
    }
}
