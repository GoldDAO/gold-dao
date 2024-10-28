use std::borrow::Cow;

use candid::{ CandidType, Decode, Encode, Principal };
use gldt_swap_common::swap::SwapIndex;
use gldt_swap_common::swap::{ SwapId, SwapInfo };
use ic_cdk::api::stable::{ stable_size, WASM_PAGE_SIZE_IN_BYTES };
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{ StableBTreeMap, Storable };
use serde::{ Deserialize, Serialize };

use crate::memory::{ get_swap_history_memory, get_user_swap_id_memory, VM };

#[derive(Serialize, Deserialize)]
pub struct Archive {
    #[serde(skip, default = "init_swap_map")]
    archive: StableBTreeMap<SwapId, SwapInfo, VM>,
    #[serde(skip, default = "get_user_swap_id_map")]
    user_swap_id_map: StableBTreeMap<Principal, VecNat, VM>,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecNat(pub Vec<SwapId>);

impl Storable for VecNat {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

impl Default for VecNat {
    fn default() -> Self {
        Self(vec![])
    }
}

fn init_swap_map() -> StableBTreeMap<SwapId, SwapInfo, VM> {
    let memory = get_swap_history_memory();
    StableBTreeMap::init(memory)
}

fn get_user_swap_id_map() -> StableBTreeMap<Principal, VecNat, VM> {
    let memory = get_user_swap_id_memory();
    StableBTreeMap::init(memory)
}

impl Default for Archive {
    fn default() -> Self {
        Self {
            archive: init_swap_map(),
            user_swap_id_map: get_user_swap_id_map(),
        }
    }
}

impl Archive {
    // each StableBTreeMap allocates 8MB worth of memory pages
    // default start is around 16~ MB
    pub fn get_archive_size_bytes(&self) -> usize {
        let num_pages = stable_size();
        let bytes = (num_pages as usize) * (WASM_PAGE_SIZE_IN_BYTES as usize);
        bytes
    }

    fn _insert_archive_swap(&mut self, swap_id: SwapId, swap: SwapInfo) {
        let user = swap.get_user_principal();
        self.archive.insert(swap_id.clone(), swap);
        self.add_swap_index_for_user(user, swap_id)
    }

    pub fn add_swap_index_for_user(&mut self, user: Principal, swap_index: SwapId) {
        let mut indexes = self.user_swap_id_map.get(&user).unwrap_or_default();
        indexes.0.push(swap_index);
        self.user_swap_id_map.insert(user, indexes);
    }

    pub fn get_swap_ids_for_user(&self, user: &Principal) -> Option<Vec<SwapId>> {
        match self.user_swap_id_map.get(user) {
            Some(vecnat) => { Some(vecnat.0) }
            None => None,
        }
    }

    pub fn insert_archive_swap_bulk(&mut self, swaps: Vec<(SwapId, SwapInfo)>) {
        swaps.into_iter().for_each(|(swap_id, swap)| { self._insert_archive_swap(swap_id, swap) })
    }

    pub fn get_swap(&self, swap_id: &SwapId) -> Option<SwapInfo> {
        self.archive.get(&swap_id)
    }

    pub fn get_swap_bulk(&self, swap_ids: &Vec<SwapId>) -> Vec<SwapInfo> {
        swap_ids
            .iter()
            .filter_map(|id| { self.archive.get(id) })
            .collect()
    }

    pub fn get_swaps(
        &self,
        start_index: SwapIndex,
        count: usize,
        user_principal: Option<Principal>
    ) -> Vec<(SwapId, SwapInfo)> {
        fn swap_id_comparator(swap_id: &SwapId, start_index: SwapIndex) -> bool {
            swap_id.1 <= start_index.clone()
        }

        // Collect all relevant swaps in a vector
        let mut swaps: Vec<_> = self.archive
            .iter()
            .filter(|(s_id, swap)| {
                swap_id_comparator(s_id, start_index.clone()) &&
                    user_principal
                        .as_ref()
                        .map_or(true, |principal| &swap.get_user_principal() == principal)
            })
            .map(|(swap_id, swap_info)| (swap_id.clone(), swap_info.clone()))
            .collect();

        // Sort the swaps in descending order by SwapIndex
        swaps.sort_by(|a, b| b.0.1.cmp(&a.0.1));

        // Return only the required number of swaps
        swaps.clone().into_iter().take(count).collect()
    }
}

#[cfg(test)]
mod tests {}
