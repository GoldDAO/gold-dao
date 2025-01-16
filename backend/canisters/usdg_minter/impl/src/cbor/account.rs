use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use minicbor::decode::{Decoder, Error};
use minicbor::encode::{Encoder, Write};

pub fn decode<Ctx>(d: &mut Decoder<'_>, _ctx: &mut Ctx) -> Result<Account, Error> {
    d.array()?;
    let principal_bytes = d.bytes()?;
    let owner =
        Principal::try_from_slice(principal_bytes).map_err(|e| Error::message(e.to_string()))?;

    let subaccount = match d.datatype()? {
        minicbor::data::Type::Bytes => {
            let subaccount_bytes = d.bytes()?.to_vec();
            if subaccount_bytes.len() == 32 {
                let mut array = [0u8; 32];
                array.copy_from_slice(&subaccount_bytes);
                Some(array)
            } else {
                return Err(Error::message("Subaccount must be 32 bytes"));
            }
        }
        minicbor::data::Type::Null => {
            d.skip()?;
            None
        }
        _ => return Err(Error::message("Invalid data type for subaccount")),
    };

    Ok(Account { owner, subaccount })
}

pub fn encode<Ctx, W: Write>(
    v: &Account,
    e: &mut Encoder<W>,
    _ctx: &mut Ctx,
) -> Result<(), minicbor::encode::Error<W::Error>> {
    e.array(2)?;
    e.bytes(v.owner.as_slice())?;

    if let Some(subaccount) = &v.subaccount {
        e.bytes(subaccount.as_slice())?;
    } else {
        e.null()?;
    }

    Ok(())
}

pub mod option {
    use super::*;
    use minicbor::{Decode, Encode};

    #[derive(Decode, Encode)]
    #[cbor(transparent)]
    struct CborAccount(#[cbor(n(0), with = "crate::cbor::account")] pub Account);

    pub fn decode<Ctx>(d: &mut Decoder<'_>, ctx: &mut Ctx) -> Result<Option<Account>, Error> {
        Ok(Option::<CborAccount>::decode(d, ctx)?.map(|n| n.0))
    }

    pub fn encode<Ctx, W: Write>(
        v: &Option<Account>,
        e: &mut Encoder<W>,
        ctx: &mut Ctx,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        (*v).map(CborAccount).encode(e, ctx)
    }
}
