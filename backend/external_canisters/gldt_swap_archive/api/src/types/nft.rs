use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

#[derive(
    CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct NftID(pub Nat);

impl Into<[u8; 32]> for NftID {
    fn into(self) -> [u8; 32] {
        let mut array = [0u8; 32]; // Fixed length array
        let bytes = self.0 .0.to_bytes_le(); // Assuming `Nat` has a `to_bytes_le` method
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
