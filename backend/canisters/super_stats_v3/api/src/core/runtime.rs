use std::cell::RefCell;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use crate::stats::custom_types::{IndexerType, SmallTX, BlockHolder, TimeStats};

use super::constants::MAX_LOGS;
use super::types::LogEntry;
use super::working_stats::WorkingStats;

thread_local! {
    // Runtime state (not stored in stable memory)
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default(); 
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct RuntimeState {
    pub data: Data,
    pub stats: WorkingStats
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            data: Data::default(),
            stats: WorkingStats::default(),
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct Data {
    admin: Vec<String>,
    authorised: Vec<String>,
    canister_logs: Vec<LogEntry>,
    self_id: String,
    target_ledger: String,
pub test_mode: bool,
pub target_ledger_locked: bool,
    tx_store: String,
    ledger_fee: u128,
    ledger_decimals: u8,
    index_type:IndexerType,
pub latest_blocks: BlockHolder,
pub temp_small_tx: Vec<SmallTX>,
pub max_return_length: usize,
pub hourly_stats: TimeStats,
pub daily_stats: TimeStats
}

impl Data {
    pub fn check_authorised(&self, principal_id: String){
        let auth_vec: &Vec<String> = &self.authorised;
        let mut auth:bool = false;
        if auth_vec.contains(&principal_id) {
            auth = true;
        }
        match auth {
            true => (),
            _ => ic_cdk::trap("Caller Not Authorised")
        }
    }

    pub fn add_authorised(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.authorised;
        if auth_vec.contains(&principal_id){
            let rtn:String = String::from("Principal is already authorised");
            return rtn;
        } else {
            auth_vec.push(principal_id);
        }
        let rtn:String = String::from("Principal authorised");
        return rtn;
    }

    pub fn remove_authorised(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.authorised;
        if auth_vec.contains(&principal_id){
            auth_vec.retain(|x: &String| x != &principal_id);
        } else {
            let rtn:String = String::from("Can't remove - Principal isn't in the list of authorised users");
            return rtn;
        }
        let rtn:String = String::from("Principal removed from authorised list");
        return rtn;
    }

    pub fn get_all_authorised(&self) -> Vec<String> {
        let auth_vec: &Vec<String> = &self.authorised;
        return auth_vec.to_owned();
    }
   
    pub fn check_admin(&self, principal_id: String){
        let auth_vec: &Vec<String> = &self.admin;
        let mut auth:bool = false;
        if auth_vec.contains(&principal_id) {
            auth = true;
        }
        match auth {
            true => (),
            _ => ic_cdk::trap("Caller Not Admin")
        }
    }

    pub fn add_admin(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.admin;
        if auth_vec.contains(&principal_id){
            let rtn:String = String::from("Principal is already admin");
            return rtn;
        } else {
            auth_vec.push(principal_id);
        }
        let rtn:String = String::from("Admin added");
        return rtn;
    }

    pub fn remove_admin(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.admin;
        if auth_vec.contains(&principal_id){
            auth_vec.retain(|x: &String| x != &principal_id);
        } else {
            let rtn:String = String::from("Can't remove - Principal isn't in the list of admins");
            return rtn;
        }
        let rtn:String = String::from("Admin removed");
        return rtn;
    }

    pub fn get_all_admins(&self) -> Vec<String> {
        let auth_vec: &Vec<String> = &self.admin;
        return auth_vec.to_owned();
    }

    pub fn add_log(&mut self, text: String){
        let nano_time: u64 = ic_cdk::api::time();
        let log_entry: LogEntry = LogEntry {
            timestamp: nano_time.to_string(),
            text,
            };
        self.canister_logs.push(log_entry);
        if &self.canister_logs.len() > &MAX_LOGS { self.canister_logs.remove(0);}
    }

    pub fn get_logs(&self) -> Option<Vec<LogEntry>>{
        match &self.canister_logs.len() {
            0 => { return None; },
            _ => { return Some(self.canister_logs.clone()); }
        }
    }

    pub fn set_self_id(&mut self, principal_id: Principal) -> String {
        let principal = principal_id.to_text();
        self.self_id = principal;
        return "Self-Principal Saved".to_string();
    }

    pub fn get_self_id(&self) -> String {
        return self.self_id.clone()
    }

    pub fn get_target_ledger(&self) -> String {
        return self.target_ledger.clone()
    }

    pub fn set_target_ledger(&mut self, ledger: String){
        self.target_ledger = ledger
    }   

    pub fn get_tx_store(&self) -> String {
        return self.tx_store.clone()
    }

    pub fn set_tx_store(&mut self, store: String){
        self.tx_store = store;
    }

    pub fn get_ledger_fee(&self) -> u128 {
        return self.ledger_fee.clone()
    }

    pub fn set_ledger_fee(&mut self, value: u128){
        self.ledger_fee = value;
    }

    pub fn get_ledger_decimals(&self) -> u8 {
        return self.ledger_decimals.clone()
    }

    pub fn set_ledger_decimals(&mut self, decimals: u8){
        self.ledger_decimals = decimals
    }

    pub fn set_index_type(&mut self, index_type: IndexerType){
        self.index_type = index_type;
    }

    pub fn get_index_type(&self) -> IndexerType {
        return self.index_type.clone();
    } 
}