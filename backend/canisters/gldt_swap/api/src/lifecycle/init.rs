use candid::{ CandidType, Principal };
use serde::Deserialize;

use gldt_swap_common::nft::NftCanisterConf;

#[derive(Deserialize, CandidType, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub gldt_ledger_id: Principal,
    pub gldnft_canisters: Vec<(Principal, NftCanisterConf)>,
    pub ogy_ledger_id: Principal,
    pub authorized_principals: Vec<Principal>,
    pub version: String,
}
