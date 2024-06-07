use candid::Nat;
use cycles_manager_api_canister::get_canisters_summary::{CanisterMetrics, CyclesTopUp};
use cycles_manager_api_canister::get_latest_top_ups::CanisterTopUp;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::Vacant;
use std::collections::{BinaryHeap, HashMap};
use types::{CanisterId, Cycles, TimestampMillis};
#[derive(Serialize, Deserialize, Default)]
pub struct Canisters {
    canisters: HashMap<CanisterId, Canister>,
}

impl Canisters {
    pub fn new(canister_ids: Vec<CanisterId>, now: TimestampMillis) -> Canisters {
        Canisters {
            canisters: canister_ids
                .into_iter()
                .map(|c| {
                    (
                        c,
                        Canister {
                            added: now,
                            top_ups: Vec::new(),
                            top_up_in_progress: false,
                            balance: None,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn add(
        &mut self,
        canister_id: CanisterId,
        balance: Option<Nat>,
        now: TimestampMillis,
    ) -> bool {
        if let Vacant(e) = self.canisters.entry(canister_id) {
            e.insert(Canister {
                added: now,
                top_ups: Vec::new(),
                top_up_in_progress: false,
                balance,
            });
            true
        } else {
            false
        }
    }

    pub fn get_mut(&mut self, canister_id: &CanisterId) -> Option<&mut Canister> {
        self.canisters.get_mut(canister_id)
    }

    pub fn metrics(&self) -> Vec<CanisterMetrics> {
        self.canisters
            .iter()
            .map(|(id, c)| CanisterMetrics {
                canister_id: *id,
                added: c.added,
                top_ups: c.top_ups.clone(),
                balance: c.balance.clone(),
            })
            .collect()
    }

    pub fn latest_top_ups(&self, count: usize) -> Vec<CanisterTopUp> {
        let mut heap = BinaryHeap::with_capacity(count);

        for top_up in self.canisters.iter().flat_map(|(id, c)| {
            c.top_ups.iter().map(|t| CanisterTopUp {
                timestamp: t.date,
                canister_id: *id,
                amount: t.amount,
            })
        }) {
            if heap.len() < count {
                heap.push(top_up);
            } else if top_up > *heap.peek().unwrap() {
                heap.pop();
                heap.push(top_up);
            }
        }

        let mut vec = heap.into_sorted_vec();
        vec.reverse();
        vec
    }
}

#[derive(Serialize, Deserialize)]
pub struct Canister {
    added: TimestampMillis,
    top_ups: Vec<CyclesTopUp>,
    top_up_in_progress: bool,
    balance: Option<Nat>,
}

impl Canister {
    pub fn top_up_in_progress(&self) -> bool {
        self.top_up_in_progress
    }

    pub fn set_top_up_in_progress(&mut self, in_progress: bool) {
        self.top_up_in_progress = in_progress;
    }

    pub fn latest_top_up(&self) -> Option<TimestampMillis> {
        self.top_ups.last().map(|t| t.date)
    }

    pub fn record_top_up(&mut self, amount: Cycles, now: TimestampMillis) {
        self.top_ups.push(CyclesTopUp { date: now, amount });
    }
}
