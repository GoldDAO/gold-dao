use canister_timer_jobs::Job;
use serde::{ Deserialize, Serialize };
use utils::env::Environment;
use crate::jobs::swap_tokens::process_token_swap;
use crate::state::read_state;
use crate::state::mutate_state;

#[derive(Serialize, Deserialize, Clone)]
pub enum TimerJob {
    BurnTokens(BurnTokensJob),
    ProcessTokenSwap(ProcessTokenSwapJob),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BurnTokensJob {
    pub attempt: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessTokenSwapJob {
    pub attempt: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Args {}

impl Job for TimerJob {
    fn execute(self) {
        match self {
            TimerJob::BurnTokens(job) => job.execute(),
            TimerJob::ProcessTokenSwap(job) => job.execute(),
        }
    }
}

use crate::jobs::burn_tokens::process_token_burn;
impl Job for BurnTokensJob {
    fn execute(self) {
        ic_cdk::spawn(process_token_burn_async());
    }
}

async fn process_token_burn_async() {
    let _ = process_token_burn().await;
}

impl Job for ProcessTokenSwapJob {
    fn execute(self) {
        ic_cdk::spawn(run_async_swap());
    }
}

async fn run_async_swap() {
    let swap_clients = read_state(|state| { state.data.swap_clients.clone() });

    for swap_client in swap_clients.iter() {
        let args = swap_client.get_config();
        let token_swap = mutate_state(|state|
            state.data.token_swaps.push_new(args, state.env.now())
        );
        let _ = process_token_swap(swap_client, token_swap.clone()).await;
    }
}
