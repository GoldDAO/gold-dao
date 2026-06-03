use crate::types::token_swaps::TokenSwaps;
use candid::Principal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;
use utils::env::CanisterEnv;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub authorized_principals: Vec<Principal>,
    pub token_swaps: TokenSwaps,

    // NOTE: the main ICP Swap canister
    pub icp_swap_canister_id: Principal,
    pub exchange_jobs: ExchangeJobsV0,

    // storage for swap guard see guards.rs
    pub exchange_job_guards: BTreeSet<u128>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ExchangeJobsV0 {
    pub exchange_jobs: HashMap<u128, ExchangeJobV0>,
    pub last_used_id: u128,
}

use crate::types::SwapClientEnum;
use ic_cdk_timers::TimerId;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::account::Subaccount;
use std::time::Duration;
use utils::numeric::Rate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExchangeJobV0 {
    pub id: u128,
    pub exchange: SwapClientEnum,
    pub rate_per_interval: Rate,
    // NOTE: timers are not preserved, so there is no need to serialize them
    #[serde(skip)]
    pub timer_id: Option<TimerId>,
    pub job_interval: Duration,
    pub source_subaccount: Option<Subaccount>, // NOTE: subaccount from which tokens are sold
    pub min_amount: Tokens,                    // in token that is bought
    pub max_amount: Option<Tokens>,
    pub destination_account: Option<Account>,
}
