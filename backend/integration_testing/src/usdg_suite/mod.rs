use candid::Principal;
use pocket_ic::PocketIc;
use types::CanisterId;

pub mod init;
#[cfg(test)]
mod tests;

pub struct TestEnv {
    pub pic: PocketIc,
    pub canister_ids: CanisterIds,
    pub principal_ids: PrincipalIds,
}

#[derive(Debug, Clone)]
pub struct PrincipalIds {
    controller: Principal,
    user: Principal,
}

#[derive(Debug, Clone)]
pub struct CanisterIds {
    pub gldt_ledger: Principal,
    pub usdg_ledger: Principal,
    pub usdg_minter: Principal,
}
