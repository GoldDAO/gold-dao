// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "decreaseLiquidity")]
    DecreaseLiquidity,
    #[serde(rename = "limitOrder")]
    LimitOrder {
        token0InAmount: candid::Nat,
        positionId: candid::Nat,
        tickLimit: candid::Int,
        token1InAmount: candid::Nat,
    },
    #[serde(rename = "claim")]
    Claim,
    #[serde(rename = "swap")]
    Swap,
    #[serde(rename = "addLiquidity")]
    AddLiquidity,
    #[serde(rename = "transferPosition")]
    TransferPosition(candid::Nat),
    #[serde(rename = "increaseLiquidity")]
    IncreaseLiquidity,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    pub to: String,
    pub action: TransactionType,
    pub token0Id: String,
    pub token1Id: String,
    pub liquidityTotal: candid::Nat,
    pub from: String,
    pub hash: String,
    pub tick: candid::Int,
    pub token1Price: f64,
    pub recipient: String,
    pub token0ChangeAmount: f64,
    pub sender: String,
    pub liquidityChange: candid::Nat,
    pub token1Standard: String,
    pub token0Fee: f64,
    pub token1Fee: f64,
    pub timestamp: candid::Int,
    pub token1ChangeAmount: f64,
    pub token1Decimals: f64,
    pub token0Standard: String,
    pub amountUSD: f64,
    pub amountToken0: f64,
    pub amountToken1: f64,
    pub poolFee: candid::Nat,
    pub token0Symbol: String,
    pub token0Decimals: f64,
    pub token0Price: f64,
    pub token1Symbol: String,
    pub poolId: String,
}

#[derive(CandidType, Deserialize)]
pub struct PoolTvlData {
    pub token0Id: String,
    pub token1Id: String,
    pub pool: String,
    pub tvlUSD: f64,
    pub token0Symbol: String,
    pub token1Symbol: String,
}

#[derive(CandidType, Deserialize)]
pub enum NatResult {
    #[serde(rename = "ok")]
    Ok(candid::Nat),
    #[serde(rename = "err")]
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub struct PublicTokenOverview {
    pub id: candid::Nat,
    pub volumeUSD1d: f64,
    pub volumeUSD7d: f64,
    pub totalVolumeUSD: f64,
    pub name: String,
    pub volumeUSD: f64,
    pub feesUSD: f64,
    pub priceUSDChange: f64,
    pub address: String,
    pub txCount: candid::Int,
    pub priceUSD: f64,
    pub standard: String,
    pub symbol: String,
}

#[derive(CandidType, Deserialize)]
pub struct PoolInfo {
    pub fee: candid::Int,
    pub token0Id: String,
    pub token1Id: String,
    pub pool: String,
    pub token1Price: f64,
    pub token1Standard: String,
    pub token1Decimals: f64,
    pub token0Standard: String,
    pub token0Symbol: String,
    pub token0Decimals: f64,
    pub token0Price: f64,
    pub token1Symbol: String,
}

#[derive(CandidType, Deserialize)]
pub struct PublicTokenChartDayData {
    pub id: candid::Int,
    pub volumeUSD: f64,
    pub timestamp: candid::Int,
    pub txCount: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub struct PublicTokenPricesData {
    pub id: candid::Int,
    pub low: f64,
    pub high: f64,
    pub close: f64,
    pub open: f64,
    pub timestamp: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub struct OldPublicTokenOverview {
    pub id: candid::Nat,
    pub totalVolumeUSD: f64,
    pub name: String,
    pub priceUSDChangeWeek: f64,
    pub volumeUSD: f64,
    pub feesUSD: f64,
    pub priceUSDChange: f64,
    pub tvlUSD: f64,
    pub address: String,
    pub volumeUSDWeek: f64,
    pub txCount: candid::Int,
    pub priceUSD: f64,
    pub volumeUSDChange: f64,
    pub tvlUSDChange: f64,
    pub standard: String,
    pub tvlToken: f64,
    pub symbol: String,
}
