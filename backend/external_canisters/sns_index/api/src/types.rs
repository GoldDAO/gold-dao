// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize)]
pub struct UpgradeArg {
    pub ledger_id: Option<Principal>,
    pub retrieve_blocks_from_ledger_interval_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct InitArg {
    pub ledger_id: Principal,
    pub retrieve_blocks_from_ledger_interval_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum IndexArg {
    Upgrade(UpgradeArg),
    Init(InitArg),
}

pub type BlockIndex = candid::Nat;
pub type SubAccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<SubAccount>,
}

#[derive(CandidType, Deserialize)]
pub struct GetAccountTransactionsArgs {
    pub max_results: candid::Nat,
    pub start: Option<BlockIndex>,
    pub account: Account,
}

pub type Tokens = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct Burn {
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
    pub to: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
    pub fee: Option<candid::Nat>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub expected_allowance: Option<candid::Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    pub burn: Option<Burn>,
    pub kind: String,
    pub mint: Option<Mint>,
    pub approve: Option<Approve>,
    pub timestamp: u64,
    pub transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionWithId {
    pub id: BlockIndex,
    pub transaction: Transaction,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactions {
    pub balance: Tokens,
    pub transactions: Vec<TransactionWithId>,
    pub oldest_tx_id: Option<BlockIndex>,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsErr {
    pub message: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionsResult {
    Ok(GetTransactions),
    Err(GetTransactionsErr),
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksRequest {
    pub start: candid::Nat,
    pub length: candid::Nat,
}

pub type Map = Vec<(String, Box<Value>)>;
#[derive(CandidType, Deserialize)]
pub enum Value {
    Int(candid::Int),
    Map(Map),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Vec<Box<Value>>),
}

pub type Block = Box<Value>;
#[derive(CandidType, Deserialize)]
pub struct GetBlocksResponse {
    pub blocks: Vec<Block>,
    pub chain_length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct FeeCollectorRanges {
    pub ranges: Vec<(Account, Vec<(BlockIndex, BlockIndex)>)>,
}

#[derive(CandidType, Deserialize)]
pub struct ListSubaccountsArgs {
    pub owner: Principal,
    pub start: Option<SubAccount>,
}

#[derive(CandidType, Deserialize)]
pub struct Status {
    pub num_blocks_synced: BlockIndex,
}
