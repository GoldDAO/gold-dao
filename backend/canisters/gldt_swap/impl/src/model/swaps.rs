use std::collections::BTreeMap;
use std::mem;
use candid::{ Nat, Principal };
use gldt_swap_common::archive::ArchiveCanister;
use gldt_swap_common::{ nft::NftID, swap::{ SwapId, SwapInfo, SwapIndex } };
use serde::{ Deserialize, Serialize };

use tracing::debug;

use gldt_swap_common::swap::{ SwapStatusForward, SwapStatusReverse };

#[derive(Serialize, Deserialize)]
pub struct Swaps {
    swap: BTreeMap<SwapId, SwapInfo>,
    current_swap_index: SwapIndex,
    archive_canisters: Vec<ArchiveCanister>,
}

impl Default for Swaps {
    fn default() -> Self {
        Self {
            swap: BTreeMap::default(),
            current_swap_index: SwapIndex::from(0u64),
            archive_canisters: vec![],
        }
    }
}

impl Swaps {
    pub fn get_current_swap_index(&self) -> Nat {
        self.current_swap_index.clone()
    }

    pub fn increment_swap_index(&mut self) -> Nat {
        let new_index = self.current_swap_index.clone() + Nat::from(1u64);
        self.current_swap_index = new_index.clone();
        new_index
    }
    pub fn decrement_swap_index(&mut self) -> Nat {
        if self.current_swap_index == Nat::from(0u64) {
            return self.current_swap_index.clone();
        }
        let new_index = self.current_swap_index.clone() - Nat::from(1u64);
        self.current_swap_index = new_index.clone();
        new_index
    }

    pub fn get_active_swap(&self, swap_id: &SwapId) -> Option<&SwapInfo> {
        self.swap.get(swap_id)
    }

    pub fn is_nft_locked(&self, nft_id: &NftID) -> bool {
        self.swap.iter().any(|(swap_id, _)| &swap_id.0 == nft_id)
    }

    pub fn get_active_swap_by_string_id(
        &self,
        nft_id_string: &String
    ) -> Option<(SwapId, SwapInfo)> {
        self.swap.iter().find_map(|(swap_id, swap)| {
            match swap {
                SwapInfo::Forward(details) => {
                    if &details.nft_id_string == nft_id_string {
                        return Some((swap_id.clone(), swap.clone()));
                    }
                    None
                }
                SwapInfo::Reverse(details) => {
                    if &details.nft_id_string == nft_id_string {
                        return Some((swap_id.clone(), swap.clone()));
                    }
                    None
                }
            }
        })
    }

    pub fn get_active_swap_mut(&mut self, swap_id: &SwapId) -> Option<&mut SwapInfo> {
        self.swap.get_mut(swap_id)
    }

    pub fn get_active_swaps(&self) -> Vec<(SwapId, SwapInfo)> {
        self.swap
            .iter()
            .map(|(swap_id, swap)| { (swap_id.clone(), swap.clone()) })
            .collect()
    }

    pub fn get_stuck_swaps(&self) -> Vec<(SwapId, SwapInfo)> {
        self.get_active_swaps()
            .into_iter()
            .filter(|(_, swap_info)| { swap_info.is_swap_over_time_threshold() })
            .collect()
    }

    pub fn get_history_total(&self) -> Nat {
        self.current_swap_index.clone()
    }

    pub fn get_active_swaps_by_user_principal(
        &self,
        user_principal: Principal
    ) -> Vec<(SwapId, SwapInfo)> {
        let swaps: Vec<(SwapId, SwapInfo)> = self.swap
            .iter()
            .filter_map(|(swap_id, swap_info)| {
                match swap_info {
                    SwapInfo::Forward(details) => {
                        if details.gldt_receiver.owner == user_principal {
                            return Some((swap_id.clone(), swap_info.clone()));
                        } else {
                            return None;
                        }
                    }
                    SwapInfo::Reverse(details) => {
                        if details.user == user_principal {
                            return Some((swap_id.clone(), swap_info.clone()));
                        } else {
                            return None;
                        }
                    }
                }
            })
            .collect();
        swaps
    }

    pub fn get_archive_canisters(&self) -> Vec<ArchiveCanister> {
        self.archive_canisters.clone()
    }

    pub fn find_canister_for_swap_index(&self, swap_index: Nat) -> Option<Principal> {
        let archive = self.archive_canisters
            .iter()
            .rev()
            .find(|archive| { swap_index.clone() >= archive.start_index.clone() });

        match archive {
            Some(a) => Some(a.canister_id),
            None => None,
        }
    }

    pub fn remove_swap_from_active_swaps(&mut self, swap_id: &SwapId) -> Option<SwapInfo> {
        self.swap.remove(&swap_id)
    }
    // you may only insert swaps with INIT status to active, otherwise they will go history. you also can't insert if there is already an nft in active swaps
    pub fn insert_active_swap(
        &mut self,
        nft_id: &NftID,
        new_swap: &SwapInfo
    ) -> Result<SwapId, ()> {
        // check if it already exists - can't insert a swap that already exists
        if self.is_nft_locked(&nft_id) {
            debug!("Swap is already present");
            return Err(());
        }
        // insert to active or history depending on the status
        match new_swap.clone() {
            SwapInfo::Forward(swap_details) => {
                match swap_details.status {
                    SwapStatusForward::Init => {
                        let swap_id = new_swap.get_swap_id();
                        self.swap.insert(swap_id.clone(), new_swap.clone());
                        debug!("FORWARD SWAP :: SwapId {swap_id:?} :: initialized");
                        Ok(new_swap.get_swap_id())
                    }
                    _ => { Err(()) }
                }
            }
            SwapInfo::Reverse(swap_details) => {
                match swap_details.status {
                    SwapStatusReverse::Init => {
                        let swap_id = new_swap.get_swap_id();
                        self.swap.insert(swap_id.clone(), new_swap.clone());
                        debug!("FORWARD SWAP :: SwapId {swap_id:?} :: initialized");
                        Ok(new_swap.get_swap_id())
                    }
                    _ => { Err(()) }
                }
            }
        }
    }
    pub fn is_active_swaps_capacity_full(&self) -> bool {
        let swap_key_size = mem::size_of::<SwapId>();
        let swap_info_size = mem::size_of::<SwapInfo>();
        let entry_size = swap_key_size + swap_info_size;
        let total_size = entry_size * self.swap.len();

        const ONE_GB: usize = 1024 * 1024 * 1024; // 1 GB in bytes

        total_size >= ONE_GB
    }
    pub fn set_new_archive_canister(&mut self, archive_canister: ArchiveCanister) {
        if self.archive_canisters.len() > 0 {
            self.update_archive_canister_end_index(
                archive_canister.start_index.clone() - Nat::from(1u64)
            );
        }
        self.archive_canisters.push(archive_canister);
    }

    pub fn update_archive_canister_end_index(&mut self, end_index: Nat) {
        match self.archive_canisters.last_mut() {
            Some(archive) => {
                archive.end_index = Some(end_index.clone());
            }
            None => {}
        }
    }

    pub fn get_total_archive_canisters(&self) -> usize {
        self.archive_canisters.len()
    }
}

#[cfg(test)]
mod tests {
    use candid::{ Nat, Principal };
    use super::*;

    use crate::state::{ init_state, mutate_state, read_state, RuntimeState };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_find_canister_for_swap_index() {
        init_runtime_state();
        let archive_canister_1 = Principal::from_slice(
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
            ]
        );
        let archive_canister_2 = Principal::from_slice(
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8,
            ]
        );
        let archive_canister_3 = Principal::from_slice(
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
            ]
        );
        let archive_canister_1 = ArchiveCanister {
            canister_id: archive_canister_1,
            start_index: Nat::from(0u64),
            end_index: None,
        };

        let archive_canister_2 = ArchiveCanister {
            canister_id: archive_canister_2,
            start_index: Nat::from(100u64),
            end_index: None,
        };

        let archive_canister_3 = ArchiveCanister {
            canister_id: archive_canister_3,
            start_index: Nat::from(200u64),
            end_index: None,
        };
        // archive should be from 0u8 - 99
        mutate_state(|s| s.data.swaps.set_new_archive_canister(archive_canister_1.clone()));

        mutate_state(|s| s.data.swaps.set_new_archive_canister(archive_canister_2.clone()));

        mutate_state(|s| s.data.swaps.set_new_archive_canister(archive_canister_3.clone()));

        assert_eq!(
            read_state(|s| s.data.swaps.find_canister_for_swap_index(Nat::from(0u64))).unwrap(),
            archive_canister_1.canister_id
        );

        assert_eq!(
            read_state(|s| s.data.swaps.find_canister_for_swap_index(Nat::from(1u64))).unwrap(),
            archive_canister_1.canister_id
        );

        assert_eq!(
            read_state(|s| s.data.swaps.find_canister_for_swap_index(Nat::from(99u64))).unwrap(),
            archive_canister_1.canister_id
        );

        assert_eq!(
            read_state(|s| s.data.swaps.find_canister_for_swap_index(Nat::from(100u64))).unwrap(),
            archive_canister_2.canister_id
        );
    }

    // #[test]
    // fn test_swap_size() {
    //     let swap_key_size = mem::size_of::<SwapId>();
    //     let swap_info_size = mem::size_of::<SwapInfo>();

    //     let entry_size = swap_key_size + swap_info_size;
    //     let total_size = entry_size * 1;
    //     let swap_is_less_than_1000_bytes = 1000;
    //     // assert!(total_size < swap_is_less_than_1000_bytes);
    // }
}
