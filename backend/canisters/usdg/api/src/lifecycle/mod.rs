use candid::{Principal, CandidType};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum MinterArgument {
    Init(InitArgument),
    Upgrade(UpgradeArgument)
}

#[derive(CandidType, Deserialize)]
pub struct InitArgument {
    pub usdg_ledger_id: Principal,
    pub gldt_ledger_id: Principal,
    pub gold_dao_governance_id: Principal,
    pub xrc_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgument {}