pub mod lifecycle;
pub mod management;
pub mod numeric;
pub mod queries;
pub mod state;
pub mod tasks;
pub mod updates;

pub const E8S: u64 = 100_000_000;

/// Time constants
const SEC_NANOS: u64 = 1_000_000_000;

/// Fee constants
const GLDT_TRANSFER_FEE: u64 = 10_000;
const USDG_TRANSFER_FEE: u64 = 1_000_000;
