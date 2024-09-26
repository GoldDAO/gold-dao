use gldt_swap_common::swap::{ SwapId, SwapInfo };

pub type Args = SwapId;
pub type Response = Option<(SwapId, SwapInfo)>;
