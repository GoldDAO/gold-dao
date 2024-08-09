// [][] -- ADMIN GATED -- [][]

use ic_cdk::update;
pub use super_stats_v3_api::{
    custom_types::IndexerType,
    runtime::RUNTIME_STATE,
    stats::updates::init_target_ledger::InitLedgerArgs,
};

use crate::stats::fetch_data::{
    dfinity_icp::t1_impl_set_target_canister,
    dfinity_icrc2::t2_impl_set_target_canister,
};

#[update]
pub async fn init_target_ledger(args: InitLedgerArgs) -> String {
    // check admin
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text()) });
    // select route
    match args.index_type {
        IndexerType::DfinityIcp => {
            let res = t1_impl_set_target_canister(args.target).await;
            match res {
                Ok(v) => {
                    RUNTIME_STATE.with(|s| { s.borrow_mut().data.set_index_type(args.index_type) });
                    return v;
                }
                Err(e) => {
                    return e;
                }
            }
        }
        IndexerType::DfinityIcrc2 => {
            let res = t2_impl_set_target_canister(args.target).await;
            match res {
                Ok(v) => {
                    RUNTIME_STATE.with(|s| { s.borrow_mut().data.set_index_type(args.index_type) });
                    return v;
                }
                Err(e) => {
                    return e;
                }
            }
        }
        IndexerType::DfinityIcrc3 => {
            return String::from("Route not yet created");
        }
    }
}
