use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use gldt_stake_common::stake_position::{StakePosition, StakePositionId};
use ic_cdk::api::stable::{stable_size, WASM_PAGE_SIZE_IN_BYTES};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};

use crate::memory::{get_stake_history_memory, get_stake_user_history_memory, VM};

#[derive(Serialize, Deserialize)]
pub struct Archive {
    #[serde(skip, default = "init_item_map")]
    archive: StableBTreeMap<StakePositionId, StakePosition, VM>,
    #[serde(skip, default = "init_user_position_id_map")]
    user_positions: StableBTreeMap<Principal, VecU8, VM>,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecU8(pub Vec<StakePositionId>);

impl Storable for VecU8 {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

impl Default for VecU8 {
    fn default() -> Self {
        Self(vec![])
    }
}

fn init_item_map() -> StableBTreeMap<StakePositionId, StakePosition, VM> {
    let memory = get_stake_history_memory();
    StableBTreeMap::init(memory)
}

fn init_user_position_id_map() -> StableBTreeMap<Principal, VecU8, VM> {
    let memory = get_stake_user_history_memory();
    StableBTreeMap::init(memory)
}

impl Default for Archive {
    fn default() -> Self {
        Self {
            archive: init_item_map(),
            user_positions: init_user_position_id_map(),
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

    fn _insert_archive_item(&mut self, id: StakePositionId, item: StakePosition) {
        let user = item.owned_by;
        self.archive.insert(id.clone(), item);
        self.add_id_for_user(user, id);
    }

    pub fn add_id_for_user(&mut self, user: Principal, id: StakePositionId) {
        let mut indexes = self.user_positions.get(&user).unwrap_or_default();
        indexes.0.push(id);
        self.user_positions.insert(user, indexes);
    }

    pub fn get_item_ids_for_user(&self, user: &Principal) -> Vec<StakePositionId> {
        match self.user_positions.get(user) {
            Some(vecnat) => vecnat.0,
            None => vec![],
        }
    }

    pub fn insert_archive_item_bulk(&mut self, items: Vec<(StakePositionId, StakePosition)>) {
        items
            .into_iter()
            .for_each(|(id, item)| self._insert_archive_item(id, item))
    }

    pub fn get_item(&self, id: &StakePositionId) -> Option<StakePosition> {
        self.archive.get(&id)
    }

    pub fn get_item_bulk(
        &self,
        ids: &Vec<StakePositionId>,
    ) -> Vec<(StakePositionId, StakePosition)> {
        let mut ids_and_items: Vec<(StakePositionId, StakePosition)> = ids
            .iter()
            .filter_map(|id| match self.archive.get(id) {
                Some(item) => Some((id.clone(), item)),
                None => None,
            })
            .collect();
        ids_and_items.sort_by_key(|&(id, _)| id);
        ids_and_items
    }
}

#[cfg(test)]
mod tests {}
