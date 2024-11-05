use candid::Principal;

use gldt_swap_common::swap::{SwapId, SwapInfo};

pub type Args = Option<Principal>;
pub type Response = Vec<(SwapId, SwapInfo)>;
