use candid::CandidType;
use ic_stable_memory::{collections::{SHashMap, SVec}, derive::{AsFixedSizeBytes, StableType}, StableType};
use serde::Deserialize;

use crate::core::{constants::D1_AS_NANOS, stable_memory::STABLE_STATE, types::IDKey};

// Struct and impl for simple activity stats - Count of active accounts over time, total unique accounts. 
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct ActivityStats {
    // store for historic snapshots
    daily_snapshots: SVec<ActivitySnapshot>,
    // working data for latest snapshot 'window'
    pub chunk_start_time: u64,
    pub chunk_end_time: u64,
    chunk_window_size: u64,
    accounts_count_during_current_snapshot: u64,
    principals_count_during_current_snapshot: u64,
}

impl ActivityStats {
    pub fn add_account_to_current_snapshot(&mut self, count: u64){
        self.accounts_count_during_current_snapshot += count;
    }

    pub fn add_principal_to_current_snapshot(&mut self, count: u64){
        self.principals_count_during_current_snapshot += count;
    }

    pub fn set_chunk_window_size(&mut self, window_size: u64){
        self.chunk_window_size = window_size;   
    }

    pub fn take_activity_snapshot(&mut self, total_accounts: u64, total_principals: u64) -> (u64, u64) {
        let ret = ActivitySnapshot{
            start_time: self.chunk_start_time.clone(),
            end_time: self.chunk_end_time.clone(),
            total_unique_accounts: total_accounts,
            total_unique_principals: total_principals,
            accounts_active_during_snapshot: self.accounts_count_during_current_snapshot.clone(),
            principals_active_during_snapshot: self.principals_count_during_current_snapshot.clone(),
        };
        self.daily_snapshots.push(ret);
        self.chunk_start_time = self.chunk_end_time;
        self.chunk_end_time = self.chunk_start_time + self.chunk_window_size;
        self.accounts_count_during_current_snapshot = 0;
        self.principals_count_during_current_snapshot = 0;

        return (self.chunk_start_time.clone(), self.chunk_end_time.clone());
    }

    // padding when no transactions are made within 24 hours
    pub fn add_empty_snapshot(&mut self, start_time: u64, end_time: u64, total_accounts: u64, total_principals: u64) -> (u64, u64) {
        let ret = ActivitySnapshot{
            start_time,
            end_time: end_time.clone(),
            total_unique_accounts: total_accounts,
            total_unique_principals: total_principals,
            accounts_active_during_snapshot: 0,
            principals_active_during_snapshot: 0,
        };
        self.daily_snapshots.push(ret);
        self.chunk_start_time = end_time.clone();
        self.chunk_end_time = end_time + self.chunk_window_size;
        self.accounts_count_during_current_snapshot = 0;
        self.principals_count_during_current_snapshot = 0;

        return (self.chunk_start_time.clone(), self.chunk_end_time.clone());
    }

    pub fn init(&mut self, start: u64, end: u64){
        self.chunk_start_time = start;
        self.chunk_end_time = end;
        self.chunk_window_size = D1_AS_NANOS;
    }

    pub fn get_daily_snapshots(&self, mut ret_len: usize) -> Vec<ActivitySnapshot> {
        let mut ret: Vec<ActivitySnapshot> = Vec::new();
        let len = self.daily_snapshots.len();
        if ret_len > len { ret_len = len+1 };
        let start = (len) - (ret_len as usize -1); // -1 as current snapshot is then added
        for i in start..len{
            let sh = self.daily_snapshots.get(i).unwrap();
            ret.push(ActivitySnapshot{
                start_time: sh.start_time.clone(),
                end_time: sh.end_time.clone(),
                total_unique_accounts: sh.total_unique_accounts.clone(),
                total_unique_principals: sh.total_unique_principals.clone(),
                accounts_active_during_snapshot: sh.accounts_active_during_snapshot.clone(),
                principals_active_during_snapshot: sh.principals_active_during_snapshot.clone(),
            });
        }
        // add current window
        let last_total = self.daily_snapshots.get(len-1).unwrap().clone();
        ret.push(ActivitySnapshot{
            start_time: self.chunk_start_time,
            end_time: self.chunk_end_time,
            total_unique_accounts: last_total.total_unique_accounts,
            total_unique_principals: last_total.total_unique_principals,
            accounts_active_during_snapshot: self.accounts_count_during_current_snapshot,
            principals_active_during_snapshot: self.principals_count_during_current_snapshot,
        });
        ret
    }
}

#[derive(StableType, AsFixedSizeBytes, Deserialize, Debug, Default, Clone, CandidType)]
pub struct ActivitySnapshot {
    pub start_time: u64,
    pub end_time: u64,
    pub total_unique_accounts: u64,
    pub total_unique_principals: u64,
    pub accounts_active_during_snapshot: u64,
    pub principals_active_during_snapshot: u64
}