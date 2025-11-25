use gldt_swap_common::swap::{SwapIndex, SwapInfo};
use std::collections::HashMap;

pub type Args = ();
pub type Response = HashMap<SwapIndex, SwapInfo>;
