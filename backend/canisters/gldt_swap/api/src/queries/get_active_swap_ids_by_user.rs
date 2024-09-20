use candid::Principal;
use gldt_swap_common::swap::SwapId;

pub type Args = Option<Principal>;
pub type Response = Vec<SwapId>;
