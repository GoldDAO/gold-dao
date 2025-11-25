use candid::Nat;
use minicbor::data::Tag;
use minicbor::decode::{Decoder, Error};
use minicbor::encode::{Encoder, Write};
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

// NOTE: unsigned bignum CBOR tag:
// https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml
const TAG_POS_BIGNUM: Tag = Tag::new(2);

// Encode a `Nat` into CBOR
pub fn encode<Ctx, W: Write>(
    v: &Nat,
    e: &mut Encoder<W>,
    _ctx: &mut Ctx,
) -> Result<(), minicbor::encode::Error<W::Error>> {
    if v.0.is_zero() {
        return e.u8(0)?.ok();
    }

    if let Some(n) = v.0.to_u64() {
        return e.u64(n)?.ok();
    }

    // Encode as tagged positive bignum
    e.tag(TAG_POS_BIGNUM)?.bytes(&v.0.to_bytes_be())?.ok()
}

// Decode a `Nat` from CBOR
pub fn decode<Ctx>(d: &mut Decoder<'_>, _ctx: &mut Ctx) -> Result<Nat, Error> {
    let pos = d.position();

    // Try small integer first
    if let Ok(n) = d.u64() {
        return Ok(Nat::from(n));
    } else {
        d.set_position(pos);
    }

    // Expect positive bignum tag
    let tag = d.tag()?;
    if tag != TAG_POS_BIGNUM {
        return Err(Error::message(
            "Nat: expected u64 or CBOR tag(2) positive bignum",
        ));
    }

    let be_bytes = d.bytes()?;
    Ok(Nat(BigUint::from_bytes_be(be_bytes)))
}

pub mod option {
    use super::*;
    use minicbor::{Decode, Encode};

    #[derive(Decode, Encode)]
    #[cbor(transparent)]
    struct CborNat(#[cbor(n(0), with = "crate::nat")] Nat);

    pub fn decode<Ctx>(d: &mut Decoder<'_>, ctx: &mut Ctx) -> Result<Option<Nat>, Error> {
        Ok(Option::<CborNat>::decode(d, ctx)?.map(|n| n.0))
    }

    pub fn encode<Ctx, W: Write>(
        v: &Option<Nat>,
        e: &mut Encoder<W>,
        ctx: &mut Ctx,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        v.as_ref().map(|n| CborNat(n.clone())).encode(e, ctx)
    }
}
