use candid::Principal;
use pocket_ic::PocketIc;
use types::CanisterId;

mod init;
mod nft_utils;
mod tests;

pub struct TestEnv {
    pub pic: PocketIc,
    pub canister_ids: CanisterIds,
    pub principal_ids: PrincipalIds,
}

#[derive(Debug, Clone)]
pub struct PrincipalIds {
    net_principal: Principal,
    controller: Principal,
    originator: Principal,
    nft_owner: Principal,
}

#[derive(Debug, Clone)]
pub struct CanisterIds {
    pub origyn_nft: CanisterId,
    pub ogy_ledger: CanisterId,
    pub gldt_ledger: CanisterId,
    pub gldt_swap: CanisterId,
    pub icp_ledger: CanisterId,
}
