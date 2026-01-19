use crate::state::read_state;

pub fn caller_is_governance_principal() -> Result<(), String> {
    if read_state(|state| state.is_caller_governance_principal()) {
        Ok(())
    } else {
        Err("Caller is not a governance principal".to_string())
    }
}

use crate::state::mutate_state;
use std::marker::PhantomData;

// TODO: think on the limit, how much NFTs are issued now? + how much can be issued in the future?
const MAX_CONCURRENT: usize = 1;

/// Guards a block from executing twice when called by the same user and from being
/// executed [MAX_CONCURRENT] or more times in parallel.
#[must_use]
pub struct GuardExchangeJob {
    exchange_job_id: u128,
    _marker: PhantomData<GuardExchangeJob>,
}

impl GuardExchangeJob {
    /// Attempts to create a new guard for the current block. Fails if there is
    /// already a pending request for the specified [exchange_job_id] or if there
    /// are at least [MAX_CONCURRENT] pending requests.
    pub fn new(exchange_job_id: u128) -> Result<Self, String> {
        mutate_state(|s| {
            if s.data.exchange_job_guards.contains(&exchange_job_id) {
                return Err("Error: Duplicate request".to_string());
            }
            if s.data.exchange_job_guards.len() >= MAX_CONCURRENT {
                return Err("Service is too busy, try again shortly".to_string());
            }
            s.data.exchange_job_guards.insert(exchange_job_id);
            Ok(Self {
                exchange_job_id,
                _marker: PhantomData,
            })
        })
    }
}

impl Drop for GuardExchangeJob {
    fn drop(&mut self) {
        mutate_state(|s| s.data.exchange_job_guards.remove(&self.exchange_job_id));
    }
}
