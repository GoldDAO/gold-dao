use std::cell::RefCell;
use ic_stable_memory::derive::{ AsFixedSizeBytes, StableType };
use crate::{active_accounts::ActivityStats, stats::{ account_tree::AccountTree, directory::Directory }};

thread_local! {
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub account_data: AccountTree,
    pub principal_data: AccountTree,
    pub directory_data: Directory,
    pub activity_stats: ActivityStats,
}
// Impl for Main is in indexer/account_tree/active_accounts.rs
