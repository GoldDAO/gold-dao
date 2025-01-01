use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum MinterArgument {
    Init(InitArgument),
    Upgrade(UpgradeArgument),
}

#[derive(CandidType, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct InitArgument {
    pub usdg_ledger_id: Principal,
    pub gldt_ledger_id: Principal,
    pub gold_dao_governance_id: Principal,
    pub xrc_id: Principal,
}

#[derive(CandidType, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct UpgradeArgument {
    pub new_medium_fee_percent: Option<u64>,
}
