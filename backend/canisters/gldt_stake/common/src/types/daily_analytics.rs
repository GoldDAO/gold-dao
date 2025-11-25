use candid::CandidType;
use candid::Nat;
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::TokenSymbol;

#[derive(Serialize, Deserialize, CandidType, Default, Clone, Debug, PartialEq, Encode, Decode)]
pub struct DailyAnalytics {
    #[n(0)]
    pub apy: f64,
    #[cbor(n(2), with = "nat_cbor")]
    pub staked_gldt: Nat,
    #[cbor(n(3), with = "nat_cbor")]
    pub weighted_stake: Nat,
    #[cbor(n(4), with = "hash_map_nat_cbor")]
    pub rewards: HashMap<TokenSymbol, Nat>,
}

mod nat_cbor {
    use candid::Nat;
    use std::str::FromStr;

    pub fn encode<Ctx, W: minicbor::encode::Write>(
        nat: &Nat,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut Ctx,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.str(&nat.to_string())?;
        Ok(())
    }

    pub fn decode<Ctx>(
        d: &mut minicbor::Decoder<'_>,
        _ctx: &mut Ctx,
    ) -> Result<Nat, minicbor::decode::Error> {
        let s = d.str()?;
        Nat::from_str(s).map_err(|_| minicbor::decode::Error::message("invalid Nat encoding"))
    }
}

mod hash_map_nat_cbor {
    use super::{nat_cbor, TokenSymbol};
    use minicbor::{Decode, Encode};
    use minicbor::{Decoder, Encoder};
    use std::collections::HashMap;

    pub fn encode<Ctx, W: minicbor::encode::Write>(
        map: &HashMap<TokenSymbol, super::Nat>,
        e: &mut Encoder<W>,
        ctx: &mut Ctx,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(map.len() as u64)?;
        for (k, v) in map.iter() {
            k.encode(e, ctx)?;
            nat_cbor::encode(v, e, ctx)?;
        }
        Ok(())
    }

    pub fn decode<Ctx>(
        d: &mut Decoder<'_>,
        ctx: &mut Ctx,
    ) -> Result<HashMap<TokenSymbol, super::Nat>, minicbor::decode::Error> {
        let len_opt = d.map()?;
        let len = len_opt
            .ok_or_else(|| minicbor::decode::Error::message("indefinite map not supported"))?;

        let mut map = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            let k = TokenSymbol::decode(d, ctx)?;
            let v = nat_cbor::decode(d, ctx)?;
            map.insert(k, v);
        }

        Ok(map)
    }
}

use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use std::borrow::Cow;

// NOTE: Max bytes size for TokenSymbol = 20, the max is calculated for at least 5 token symbols + 40 maximal bytes
const MAX_DAILY_ANALYTICS_BYTES_SIZE: u32 = 140;

impl Storable for DailyAnalytics {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        minicbor::encode(self, &mut buf).expect("DailyAnalytics encoding should always succeed");
        Cow::Owned(buf)
    }

    // fn into_bytes(self) -> std::vec::Vec<u8> {
    //     let mut buf = vec![];
    //     minicbor::encode(self, &mut buf).expect("DailyAnalytics encoding should always succeed");
    //     buf
    // }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        minicbor::decode(bytes.as_ref()).unwrap_or_else(|e| {
            panic!(
                "failed to decode DailyAnalytics bytes {}: {e}",
                hex::encode(bytes)
            )
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_DAILY_ANALYTICS_BYTES_SIZE,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Nat;
    use std::collections::HashMap;
    use types::TokenSymbol;

    fn sample_daily_analytics() -> DailyAnalytics {
        let mut rewards1 = HashMap::new();
        rewards1.insert(TokenSymbol::ICP, Nat::from(100u32));
        rewards1.insert(TokenSymbol::OGY, Nat::from(200u32));

        let mut rewards2 = HashMap::new();
        rewards2.insert(TokenSymbol::GOLDAO, Nat::from(300u32));
        rewards2.insert(TokenSymbol::GLDT, Nat::from(400u32));

        DailyAnalytics {
            apy: 12.34,
            staked_gldt: Nat::from(500u32),
            weighted_stake: Nat::from(600u32),
            rewards: rewards1,
        }
    }

    #[test]
    fn test_nat_cbor_roundtrip() {
        let n = Nat::from(123456u32);
        let mut buf = vec![];
        let mut ctx = ();
        nat_cbor::encode(&n, &mut minicbor::Encoder::new(&mut buf), &mut ctx).unwrap();
        let decoded = nat_cbor::decode(&mut minicbor::Decoder::new(&buf), &mut ctx).unwrap();
        assert_eq!(n, decoded);
    }

    #[test]
    fn test_hash_map_nat_cbor_roundtrip() {
        let mut map = HashMap::new();
        map.insert(TokenSymbol::ICP, Nat::from(100u32));
        map.insert(TokenSymbol::GLDT, Nat::from(200u32));

        let mut buf = vec![];
        let mut ctx = ();
        hash_map_nat_cbor::encode(&map, &mut minicbor::Encoder::new(&mut buf), &mut ctx).unwrap();
        let decoded =
            hash_map_nat_cbor::decode(&mut minicbor::Decoder::new(&buf), &mut ctx).unwrap();
        assert_eq!(map, decoded);
    }

    #[test]
    fn test_btreemap_rewards_cbor_roundtrip() {
        let mut rewards1 = HashMap::new();
        rewards1.insert(TokenSymbol::ICP, Nat::from(100u32));
        rewards1.insert(TokenSymbol::OGY, Nat::from(200u32));

        let mut buf = vec![];
        let mut ctx = ();
        hash_map_nat_cbor::encode(&rewards1, &mut minicbor::Encoder::new(&mut buf), &mut ctx)
            .unwrap();
        let decoded =
            hash_map_nat_cbor::decode(&mut minicbor::Decoder::new(&buf), &mut ctx).unwrap();
        assert_eq!(rewards1, decoded);
    }

    #[test]
    fn test_daily_analytics_storable_roundtrip() {
        let analytics = sample_daily_analytics();

        let bytes = analytics.to_bytes();
        let decoded = DailyAnalytics::from_bytes(bytes);

        assert_eq!(analytics, decoded);
    }

    #[test]
    fn test_daily_analytics_encode_decode_max_case() {
        // Build a max DailyAnalytics instance
        let mut rewards = HashMap::new();
        rewards.insert(TokenSymbol::ICP, Nat::from(1_000_000u64));
        rewards.insert(TokenSymbol::OGY, Nat::from(2_000_000u64));
        rewards.insert(TokenSymbol::GOLDAO, Nat::from(3_000_000u64));
        rewards.insert(TokenSymbol::WTN, Nat::from(4_000_000u64));
        rewards.insert(TokenSymbol::GLDT, Nat::from(5_000_000u64));

        let analytics = DailyAnalytics {
            apy: 12.34,
            staked_gldt: Nat::from(10_000_000u64),
            weighted_stake: Nat::from(20_000_000u64),
            rewards,
        };

        // Encode to bytes
        let mut buf = Vec::new();
        minicbor::encode(&analytics, &mut buf).expect("Encoding should succeed");
        println!(
            "Encoded DailyAnalytics to bytes (len={}): {:?}",
            buf.len(),
            buf
        );

        // Decode back
        let decoded: DailyAnalytics = minicbor::decode(&buf).expect("Decoding should succeed");

        // Check equality
        assert_eq!(decoded, analytics);
    }
}
