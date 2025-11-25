use candid::Principal;
use gldt_swap_common::swap::SwapIndex;

pub type Args = Option<Principal>;
pub type Response = Vec<SwapIndex>;
