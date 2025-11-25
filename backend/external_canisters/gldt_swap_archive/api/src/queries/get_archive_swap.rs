use crate::swap::SwapId;
use crate::swap::SwapInfo;

pub type Args = SwapId;
pub type Response = Option<(SwapId, SwapInfo)>;
