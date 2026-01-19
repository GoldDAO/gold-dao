use crate::wasms;
use bity_ic_types::BuildVersion;
use buyback_burn_api::post_upgrade::UpgradeArgs;
use candid::CandidType;
use candid::{encode_one, Principal};
use ic_ledger_types::Tokens;
use pocket_ic::PocketIc;
use serde::Deserialize;
use serde::Serialize;
use types::{TokenInfo, TokenSymbol};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Args {
    Init(InitArgs),
    Upgrade(UpgradeArgs),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub tokens: Vec<TokenAndPool>,
    pub buyback_interval_in_secs: u64,
    pub icp_swap_canister_id: Principal,
    pub burn_rate: u8,
    pub min_burn_amount: Tokens, // in GOLDAO tokens
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TokenAndPool {
    pub token: TokenInfo,
    pub swap_pool_id: Principal,
}

pub fn setup_old_buyback_burn_canister(
    pic: &PocketIc,
    sns_buyback_burn_id: Principal,
    controller: &Principal,
) -> Principal {
    let buyback_burn_wasm = wasms::BUYBACK_BURN_OLD.clone();
    pic.add_cycles(sns_buyback_burn_id, 100_000_000_000_000_000);
    pic.set_controllers(
        sns_buyback_burn_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    let init_args = Args::Init(InitArgs {
        test_mode: true,
        version: BuildVersion::default(),
        commit_hash: String::new(),
        authorized_principals: vec![],
        gldgov_token_info: TokenSymbol::GOLDAO.get_token_info(),
        tokens: vec![],
        buyback_interval_in_secs: 0,
        icp_swap_canister_id: Principal::anonymous(),
        burn_rate: 0,
        min_burn_amount: Tokens::from_e8s(0),
    });
    let _ = pic
        .reinstall_canister(
            sns_buyback_burn_id,
            buyback_burn_wasm,
            encode_one(init_args).unwrap(),
            Some(controller.clone()),
        )
        .unwrap();
    sns_buyback_burn_id
}
