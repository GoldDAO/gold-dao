use std::collections::BTreeMap;

use candid::{Nat, Principal};

pub type Args = ();
pub type Response = BTreeMap<(Principal, u16), Nat>;
