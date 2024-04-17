#![cfg(test)]

use std::vec::IntoIter;

use candid::Principal;
use pocket_ic::PocketIc;
use setup::sns::SNSTestEnv;
use types::{ CanisterId, Cycles };

mod client;
mod setup;
mod tests;
mod utils;
mod wasms;

const T: Cycles = 1_000_000_000_000;

#[derive(Debug, Copy, Clone)]
pub struct CanisterIds {
    pub ogy_ledger_id: CanisterId,
    pub icp_ledger_id: CanisterId,
    pub gldgov_ledger_id: CanisterId,
}

impl IntoIterator for CanisterIds {
    type Item = CanisterId;
    type IntoIter = IntoIter<CanisterId>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.ogy_ledger_id, self.icp_ledger_id, self.gldgov_ledger_id].into_iter()
    }
}
