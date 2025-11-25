use gldt_swap_api_archive::swap::SwapIndex as SwapIndexOld;
use gldt_swap_api_archive::swap::SwapInfo as SwapInfoOld;
use std::collections::HashMap;

pub type Args = HashMap<SwapIndexOld, SwapInfoOld>;
pub type Response = Result<(), String>;
