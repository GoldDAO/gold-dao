pub const GLDT_SUBDIVIDABLE_BY: u64 = 100_000_000;
pub const GLDT_DECIMALS: u8 = 8;
pub const GLDT_PRICE_RATIO: u8 = 100;
pub const GLDT_TX_FEE: u64 = 10_000_000u64;
pub const GLDT_SWAP_FEE_ACCOUNT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
];
pub const GLDT_LEDGER_FEE_ACCOUNT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];
pub const MEMO_GLDT_SWAP: [u8; 9] = [0x47, 0x4c, 0x44, 0x54, 0x5f, 0x53, 0x57, 0x41, 0x50]; // GLDT_SWAP
