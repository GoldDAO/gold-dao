use ic_stable_structures::{ Storable, storable::Bound };
use std::borrow::Cow;

/// A helper type implementing Storable for all
/// serde-serializable types using the CBOR encoding.
#[derive(Default, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Cbor<T>(pub T) where T: serde::Serialize + serde::de::DeserializeOwned;

impl<T> std::ops::Deref for Cbor<T> where T: serde::Serialize + serde::de::DeserializeOwned {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const MAX_VALUE_SIZE: u32 = 100000;

impl<T> Storable for Cbor<T> where T: serde::Serialize + serde::de::DeserializeOwned {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self.0, &mut buf).unwrap();

        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(ciborium::de::from_reader(bytes.as_ref()).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
