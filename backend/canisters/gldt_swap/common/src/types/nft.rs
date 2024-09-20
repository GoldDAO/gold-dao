use std::borrow::Cow;

use candid::{ CandidType, Decode, Encode, Nat };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };

const MAX_NFT_ID_BYTE_SIZE: u32 = 40;

#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default
)]
pub struct NftID(pub Nat);

impl Storable for NftID {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_NFT_ID_BYTE_SIZE,
        is_fixed_size: false,
    };
}

impl Into<[u8; 32]> for NftID {
    fn into(self) -> [u8; 32] {
        let mut array = [0u8; 32]; // Fixed length array
        let bytes = self.0.0.to_bytes_le(); // Assuming `Nat` has a `to_bytes_le` method
        let len = bytes.len().min(array.len());
        array[..len].copy_from_slice(&bytes[..len]);
        array
    }
}

pub type NftWeight = u16;
/// Configuration information for a single NFT canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct NftCanisterConf {
    /// The size in grams of the physical NFTs managed by this
    /// canister.  Note that the max value of u16 in grams is over
    /// 65kg. The largest gold bars are 400oz (~11kg) and the largest
    /// silver bars are 1000oz (~31kg).
    pub grams: NftWeight,
}

#[cfg(test)]
mod tests {
    use candid::Nat;
    use icrc_ledger_types::icrc1::account::Subaccount;

    use super::NftID;

    #[test]
    fn test_nft_id_to_subaccount() {
        let nft_id = NftID(Nat::from(1u64));
        let subaccount: Subaccount = nft_id.into();
        assert_eq!(
            subaccount,
            [
                1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ]
        );

        let nft_id = NftID(Nat::from(192u64));
        let subaccount: Subaccount = nft_id.into();
        assert_eq!(
            subaccount,
            [
                192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ]
        );

        let nft_id = NftID(Nat::from(38291u64));
        let subaccount: Subaccount = nft_id.into();

        assert_eq!(
            subaccount,
            [
                147u8, 149u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ]
        );
    }
}
