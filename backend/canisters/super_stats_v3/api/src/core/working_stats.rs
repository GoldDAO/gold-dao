use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use super::{runtime::RUNTIME_STATE, stable_memory::STABLE_STATE};

#[derive(CandidType, Deserialize, Serialize, Default, Clone, Debug)]
pub struct WorkingStats {
    timer_active: bool,
    pub is_busy: bool,
    next_block: u64,
    pub ledger_tip_of_chain: u64,
    is_upto_date: bool,
    pub directory_count: u64,
    pub last_update_time: u64,
    pub metrics: Metrics,
}

impl WorkingStats {
    pub fn update_timer(&mut self, state: bool) -> String {
        self.timer_active = state;
        return "timer_active has been updated".to_string();
    }

    pub fn get_timer_state(&self) -> bool {
        return self.timer_active.clone();
    }

    pub fn get_metrics(&self) -> Metrics {
        return self.metrics.clone();
    }

    pub fn set_next_block(&mut self, block: u64){
        self.next_block = block;
    }

    pub fn get_next_block(&self) -> u64 {
        return self.next_block.clone();
    }

    pub fn set_is_upto_date(&mut self, value: bool){
        self.is_upto_date = value;
    }

    pub fn get_working_stats(&self) -> WorkingStats {
        let total = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_total_entries()
        });
        let mut ws = self.clone();
        ws.directory_count = total;
        return ws;
    }

    pub fn set_busy(&mut self){
        self.is_busy = true;
    }
    
    pub fn set_not_busy(&mut self){
        self.is_busy = false;
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone, Debug)]
pub struct Metrics {
    total_errors: u64,
    total_api_requests: u64,
}

impl Metrics {
    pub fn increment_total_errors(&mut self){
        self.total_errors += 1;
    }

    pub fn increment_total_api(&mut self){
        self.total_api_requests += 1;
    }
}