use std::borrow::Cow;

use candid::{ CandidType, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Clone, Deserialize, CandidType, Copy, PartialEq, Eq, Hash)]
pub enum TokenSymbol {
    OGY,
    ICP,
    GLDGov,
}

const MAX_VALUE_SIZE: u32 = 8;
impl Storable for TokenSymbol {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
