use crate::model::swap_configs::SwapConfigs;
use bity_ic_canister_time::timestamp_millis;
use candid::{Nat, Principal};
use gldt_swap_common::nft::Nft;
use gldt_swap_common::swap::{SwapIndex, SwapInfo, SwapStatus, SwapType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use types::TimestampMillis;

#[derive(Serialize, Deserialize)]
pub struct SwapSystem {
    pub swaps: HashMap<SwapIndex, SwapInfo>,
    pub failed_swaps: HashMap<SwapIndex, SwapInfo>,
    pub latest_timestamp: TimestampMillis,
    pub last_index: SwapIndex,
}

impl Default for SwapSystem {
    fn default() -> Self {
        let now = timestamp_millis();
        Self {
            swaps: HashMap::new(),
            failed_swaps: HashMap::new(),
            latest_timestamp: now,
            last_index: Nat::from(0_u64),
        }
    }
}

impl SwapSystem {
    pub fn create_swap(
        &mut self,
        swap_type: SwapType,
        user: Principal,
        nft: &Nft,
        swap_configs: &SwapConfigs,
    ) -> Result<SwapIndex, gldt_swap_common::general_error::GeneralError> {
        // Generate new index
        let index = self.get_new_index();

        // Get token equivalent for this NFT
        let nft_token_equivalent = swap_configs.get_nft_equivalent(nft)?;

        // Build SwapInfo
        let swap_info = SwapInfo::new(
            index.clone(),
            swap_type,
            user.into(),
            nft.clone(),
            nft_token_equivalent,
        );

        // Insert it immediately
        self.insert_swap(index.clone(), swap_info);

        Ok(index)
    }

    pub fn create_swaps_batch<I>(
        &mut self,
        swap_type: SwapType,
        user: Principal,
        nfts: I,
        swap_configs: &SwapConfigs,
    ) -> Result<HashMap<SwapIndex, SwapInfo>, gldt_swap_common::general_error::GeneralError>
    where
        I: IntoIterator<Item = Nft>,
    {
        let nfts_iter = nfts.into_iter();
        let (lower, _) = nfts_iter.size_hint();
        let mut results = HashMap::with_capacity(lower);

        for nft in nfts_iter {
            let eq = swap_configs.get_nft_equivalent(&nft)?;

            let index = self.get_new_index();

            let swap_info = SwapInfo::new(index.clone(), swap_type, user.into(), nft, eq);

            self.insert_swap(index.clone(), swap_info.clone());

            results.insert(index, swap_info);
        }

        self.latest_timestamp = timestamp_millis();

        Ok(results)
    }

    pub fn get_new_index(&self) -> SwapIndex {
        self.last_index.clone()
    }

    pub fn insert_swap(&mut self, swap_id: SwapIndex, info: SwapInfo) {
        self.latest_timestamp = timestamp_millis();
        self.swaps.insert(swap_id.clone(), info.clone());
        self.last_index += SwapIndex::from(1u64);
    }

    pub fn get_swap(&self, swap_id: &SwapIndex) -> Option<&SwapInfo> {
        self.swaps.get(swap_id)
    }

    pub fn get_active_swap_by_ids(&self, ids: &HashSet<SwapIndex>) -> HashMap<SwapIndex, SwapInfo> {
        self.swaps
            .iter()
            .filter(|(index, _)| ids.contains(*index))
            .map(|(index, swap_info)| (index.clone(), swap_info.clone()))
            .collect()
    }

    pub fn get_failed_swap_by_ids(&self, ids: &HashSet<SwapIndex>) -> HashMap<SwapIndex, SwapInfo> {
        self.failed_swaps
            .iter()
            .filter(|(index, _)| ids.contains(*index))
            .map(|(index, swap_info)| (index.clone(), swap_info.clone()))
            .collect()
    }

    pub fn get_mut_swap(&mut self, swap_id: &SwapIndex) -> Option<&mut SwapInfo> {
        self.swaps.get_mut(swap_id)
    }

    pub fn get_all_swaps(&self) -> HashMap<SwapIndex, SwapInfo> {
        self.swaps.clone()
    }

    pub fn get_failed_swaps(&self) -> HashMap<SwapIndex, SwapInfo> {
        self.failed_swaps.clone()
    }

    pub fn get_active_swaps_by_user_principal(
        &self,
        user_principal: Principal,
    ) -> HashMap<SwapIndex, SwapInfo> {
        self.swaps
            .iter()
            .filter(|(_, swap_info)| swap_info.user_account.owner == user_principal)
            .map(|(swap_id, swap_info)| (swap_id.clone(), swap_info.clone()))
            .collect()
    }

    pub fn get_failed_swaps_by_user_principal(
        &self,
        user_principal: Principal,
    ) -> HashMap<SwapIndex, SwapInfo> {
        self.failed_swaps
            .iter()
            .filter(|(_, swap_info)| swap_info.user_account.owner == user_principal)
            .map(|(swap_id, swap_info)| (swap_id.clone(), swap_info.clone()))
            .collect()
    }

    pub fn update_swaps_statuses(&mut self, swap_ids: &[SwapIndex], new_status: SwapStatus) {
        for swap_id in swap_ids {
            if let Some(swap) = self.get_mut_swap(swap_id) {
                // Attempt to update, ignore errors
                let _ = swap.update_status(new_status.clone());
            }
        }
    }

    pub fn update_swaps_status(&mut self, swap_id: &SwapIndex, new_status: SwapStatus) {
        if let Some(swap) = self.get_mut_swap(swap_id) {
            let _ = swap.update_status(new_status.clone());
        }
    }

    pub fn filter_swaps_to_reimburse(&self, ids: &[SwapIndex]) -> HashMap<SwapIndex, SwapInfo> {
        self.filter_swaps_by(ids, |status| {
            matches!(
                status,
                SwapStatus::NftTransferFailed(_) | SwapStatus::MintFailed(_)
            )
        })
    }

    pub fn filter_completed_swaps(&self, ids: &[SwapIndex]) -> HashMap<SwapIndex, SwapInfo> {
        self.filter_swaps_by(ids, |status| matches!(status, SwapStatus::Complete))
    }

    pub fn filter_swaps_by<F>(
        &self,
        ids: &[SwapIndex],
        predicate: F,
    ) -> HashMap<SwapIndex, SwapInfo>
    where
        F: Fn(&SwapStatus) -> bool,
    {
        ids.iter()
            .filter_map(|id| {
                self.swaps.get(id).and_then(|s| {
                    if predicate(&s.status) {
                        Some((id.clone(), s.clone()))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn remove_swap(&mut self, swap_id: &SwapIndex) -> Option<SwapInfo> {
        self.latest_timestamp = timestamp_millis();
        self.swaps.remove(swap_id)
    }

    pub fn remove_active_swaps(&mut self, swap_ids: Vec<SwapIndex>) {
        self.latest_timestamp = timestamp_millis();
        swap_ids.iter().for_each(|id| {
            self.swaps.remove(id);
        });
    }

    // Removes completed swaps and moves failed swaps to `failed_swaps`.
    pub fn finalize_swaps(&mut self, swap_ids: &[SwapIndex]) {
        self.latest_timestamp = timestamp_millis();

        for swap_id in swap_ids {
            if let Some(swap) = self.swaps.remove(swap_id) {
                match swap.status {
                    SwapStatus::Complete => {
                        continue;
                    }
                    SwapStatus::NftTransferFailed(_) | SwapStatus::MintFailed(_) => {
                        self.failed_swaps.insert(swap_id.clone(), swap);
                    }
                    _ => {
                        self.swaps.insert(swap_id.clone(), swap);
                    }
                }
            }
        }
    }

    pub fn finalize_all_swaps(&mut self) {
        self.latest_timestamp = timestamp_millis();

        let all_ids: Vec<SwapIndex> = self.swaps.keys().cloned().collect();

        for swap_id in all_ids {
            if let Some(swap) = self.swaps.remove(&swap_id) {
                match swap.status {
                    SwapStatus::Complete => {
                        continue;
                    }
                    SwapStatus::NftTransferFailed(_) | SwapStatus::MintFailed(_) => {
                        self.failed_swaps.insert(swap_id.clone(), swap);
                    }
                    _ => {
                        self.failed_swaps.insert(swap_id.clone(), swap);
                    }
                }
            }
        }
    }

    pub fn swaps_by_status(&self, status: SwapStatus) -> Vec<SwapInfo> {
        self.swaps
            .values()
            .filter(|swap| swap.status == status)
            .cloned()
            .collect()
    }

    pub fn bump_timestamp(&mut self) {
        self.latest_timestamp = timestamp_millis();
    }

    pub fn get_stuck_swaps(&self) -> Vec<(SwapIndex, SwapInfo)> {
        self.swaps
            .clone()
            .into_iter()
            .filter(|(_, swap_info)| swap_info.is_swap_over_time_threshold())
            .collect()
    }
}
