use crate::LiquidityError;

pub type Args = Option<[u8; 32]>;
pub type Response = Result<Option<u64>, LiquidityError>;
