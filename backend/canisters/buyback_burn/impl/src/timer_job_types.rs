use crate::jobs::swap_tokens::process_token_swap;
use crate::model::token_swaps::TokenSwap;
use canister_timer_jobs::Job;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, Clone)]
pub enum TimerJob {
    ProcessTokenSwap(Box<ProcessTokenSwapJob>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessTokenSwapJob {
    pub token_swap: TokenSwap,
    pub attempt: u32,
}

impl Job for TimerJob {
    fn execute(self) {
        match self {
            TimerJob::ProcessTokenSwap(job) => job.execute(),
        }
    }
}

impl Job for ProcessTokenSwapJob {
    fn execute(self) {
        ic_cdk::spawn(async move {
            process_token_swap(self.token_swap, self.attempt).await;
        });
    }
}
