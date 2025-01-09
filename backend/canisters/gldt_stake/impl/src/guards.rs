use crate::state::mutate_state;
use crate::state::read_state;
use candid::Principal;
use std::marker::PhantomData;

const MAX_CONCURRENT: usize = 100;

/// Guards a block from executing twice when called by the same user and from being
/// executed [MAX_CONCURRENT] or more times in parallel.
#[must_use]
pub struct GuardPrincipal {
    principal: Principal,
    _marker: PhantomData<GuardPrincipal>,
}

impl GuardPrincipal {
    /// Attempts to create a new guard for the current block. Fails if there is
    /// already a pending request for the specified [principal] or if there
    /// are at least [MAX_CONCURRENT] pending requests.
    pub fn new(principal: Principal) -> Result<Self, String> {
        mutate_state(|s| {
            if s.data.principal_guards.contains(&principal) {
                return Err(format!("Error: Duplicate request"));
            }
            if s.data.principal_guards.len() >= MAX_CONCURRENT {
                return Err(format!("Service is too busy, try again shortly"));
            }
            s.data.principal_guards.insert(principal);
            Ok(Self {
                principal,
                _marker: PhantomData,
            })
        })
    }
}

impl Drop for GuardPrincipal {
    fn drop(&mut self) {
        mutate_state(|s| s.data.principal_guards.remove(&self.principal));
    }
}

pub fn caller_is_governance_principal() -> Result<(), String> {
    if read_state(|state| state.is_caller_governance_principal()) {
        Ok(())
    } else {
        Err("Caller is not a governance principal".to_string())
    }
}

pub fn reject_anonymous_caller() -> Result<(), String> {
    if ic_cdk::caller() == Principal::anonymous() {
        return Err(format!("You may not use an anonymous principal"));
    }
    Ok(())
}
