#![cfg(test)]

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

#[derive(Debug)]
pub struct CanisterIds {
    pub ogy_ledger_id: CanisterId,
    pub icp_ledger_id: CanisterId,
    pub gldgov_ledger_id: CanisterId,
}
