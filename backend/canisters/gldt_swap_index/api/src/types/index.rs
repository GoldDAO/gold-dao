use crate::wrapped_values::WrappedAccount;
use candid::CandidType;
use gldt_swap_common::nft::Nft;
use gldt_swap_common::swap::SwapStatus;
use ic_stable_structures::{storable::Bound, Storable};
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(
    CandidType, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Clone, Encode, Decode, Debug,
)]
pub enum IndexType {
    #[n(0)]
    Account(#[n(0)] WrappedAccount),
    #[n(1)]
    BlockType(#[n(0)] String),
    #[n(2)]
    Status(#[n(0)] SwapStatus),
    #[n(3)]
    Nft(#[n(0)] Nft),
}

#[derive(Debug)]
pub struct IndexValue(pub Vec<u64>);

impl Storable for IndexType {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let mut buffer = Vec::new();
        minicbor::encode(self, &mut buffer).expect("failed to encode IndexType");
        Cow::Owned(buffer)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        minicbor::decode(&bytes).expect("failed to decode IndexType")
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for IndexValue {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let mut buffer = Vec::new();
        minicbor::encode(&self.0, &mut buffer).expect("failed to encode IndexValue");
        Cow::Owned(buffer)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let index_value = minicbor::decode(&bytes).expect("failed to decode IndexValue");
        IndexValue(index_value)
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(
    CandidType, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Clone, Encode, Decode, Debug,
)]
pub enum SortBy {
    #[n(0)]
    Ascending,
    #[n(1)]
    Descending,
}
