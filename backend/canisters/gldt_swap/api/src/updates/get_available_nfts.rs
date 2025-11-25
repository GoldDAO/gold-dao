use candid::Nat;
use candid::Principal;
use gldt_swap_common::general_error::GeneralError;
use std::collections::HashMap;
use types::CanisterId;

pub type Args = Option<Principal>;

pub type Response = Result<HashMap<CanisterId, Vec<Nat>>, GeneralError>;
