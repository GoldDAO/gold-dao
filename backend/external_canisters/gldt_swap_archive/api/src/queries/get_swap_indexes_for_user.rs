use crate::swap::SwapId;
use candid::Principal;

pub type Args = Principal;
pub type Response = Option<Vec<SwapId>>;
