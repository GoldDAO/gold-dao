pub mod lifecycle;
pub mod management;
pub mod queries;
pub mod state;
pub mod tasks;
pub mod updates;

/// Time constants
const SEC_NANOS: u64 = 1_000_000_000;

/// Fee constants
const GLDT_TRANSFER_FEE: u64 = 10_000;
const USDG_TRANSFER_FEE: u64 = 1_000_000;
