use candid::Principal;

use gldt_swap_common::swap::SwapId;

pub type Args = Principal;
pub type Response = Option<Vec<SwapId>>;
