use crate::VaultError;

pub type Args = u64;
pub type Response = Result<Option<u64>, VaultError>;
