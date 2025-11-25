use gldt_swap_common::swap::SwapIndex;
use gldt_swap_common::swap::SwapInfo;
use std::collections::HashMap;

pub type Args = HashMap<SwapIndex, SwapInfo>;
pub type Response = Result<(), String>;
