// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{ self, CandidType, Deserialize, Principal };
use ic_cdk::api::call::CallResult as Result;
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManageSaleResult {
    ok(Box<ManageSaleResponse>),
    err(OrigynError),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OrigynError {
    pub text: String,
    pub error: Errors,
    pub number: u32,
    pub flag_point: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Errors {
    nyi,
    storage_configuration_error,
    escrow_withdraw_payment_failed,
    token_not_found,
    owner_not_found,
    content_not_found,
    auction_ended,
    out_of_range,
    sale_id_does_not_match,
    sale_not_found,
    kyc_fail,
    item_not_owned,
    property_not_found,
    validate_trx_wrong_host,
    withdraw_too_large,
    content_not_deserializable,
    bid_too_low,
    validate_deposit_wrong_amount,
    existing_sale_found,
    noop,
    asset_mismatch,
    escrow_cannot_be_removed,
    deposit_burned,
    cannot_restage_minted_token,
    cannot_find_status_in_metadata,
    receipt_data_mismatch,
    validate_deposit_failed,
    unreachable,
    unauthorized_access,
    item_already_minted,
    no_escrow_found,
    escrow_owner_not_the_owner,
    improper_interface,
    app_id_not_found,
    token_non_transferable,
    kyc_error,
    sale_not_over,
    escrow_not_large_enough,
    update_class_error,
    malformed_metadata,
    token_id_mismatch,
    id_not_found_in_metadata,
    auction_not_started,
    library_not_found,
    attempt_to_stage_system_data,
    validate_deposit_wrong_buyer,
    not_enough_storage,
    sales_withdraw_payment_failed,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManageSaleResponse {
    bid(BidResponse),
    escrow_deposit(EscrowResponse),
    recognize_escrow(RecognizeEscrowResponse),
    withdraw(WithdrawResponse),
    ask_subscribe(AskSubscribeResponse),
    end_sale(EndSaleResponse),
    refresh_offers(Vec<EscrowRecord>),
    distribute_sale(DistributeSaleResponse),
    open_sale(bool),
}
pub type DistributeSaleResponse = Vec<Result1>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Result1 {
    ok(Box<ManageSaleResponse>),
    err(OrigynError),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PropertyShared {
    value: Box<CandyShared>,
    name: String,
    immutable: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CandyShared {
    Int(candid::Int),
    Map(Vec<(Box<CandyShared>, Box<CandyShared>)>),
    Nat(candid::Nat),
    Set(Vec<CandyShared>),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Bool(bool),
    Int8(i8),
    Ints(Vec<candid::Int>),
    Nat8(u8),
    Nats(Vec<candid::Nat>),
    Text(String),
    Bytes(serde_bytes::ByteBuf),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Option(Option<Box<CandyShared>>),
    Floats(Vec<f64>),
    Float(f64),
    Principal(Principal),
    Array(Vec<CandyShared>),
    Class(Vec<PropertyShared>),
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EscrowRecord {
    token: TokenSpec,
    token_id: String,
    seller: Account,
    lock_to_date: Option<candid::Int>,
    buyer: Account,
    amount: candid::Nat,
    sale_id: Option<String>,
    account_hash: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Account {
    account_id(String),
    principal(Principal),
    extensible(Box<CandyShared>),
    account {
        owner: Principal,
        sub_account: Option<serde_bytes::ByteBuf>,
    },
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionID {
    nat(candid::Nat),
    text(String),
    extensible(Box<CandyShared>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EndSaleResponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_network_updated {
        network: Principal,
        extensible: Box<CandyShared>,
    },
    escrow_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account>,
        extensible: Box<CandyShared>,
    },
    data {
        hash: Option<serde_bytes::ByteBuf>,
        extensible: Box<CandyShared>,
        data_dapp: Option<String>,
        data_path: Option<String>,
    },
    sale_ended {
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account,
        from: Account,
        sale: Option<EndSaleResponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        receiver: Account,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account,
        from: Account,
        extensible: Box<CandyShared>,
    },
    sale_opened {
        pricing: PricingConfigShared,
        extensible: Box<CandyShared>,
        sale_id: String,
    },
    canister_owner_updated {
        owner: Principal,
        extensible: Box<CandyShared>,
    },
    sale_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EndSaleResponse {
    token_id: String,
    txn_type: EndSaleResponse_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ICTokenSpec {
    pub id: Option<candid::Nat>,
    pub fee: Option<candid::Nat>,
    pub decimals: candid::Nat,
    pub canister: Principal,
    pub standard: ICTokenSpec_standard,
    pub symbol: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ICTokenSpec_standard {
    ICRC1,
    EXTFungible,
    DIP20,
    Other(Box<CandyShared>),
    Ledger,
}
pub type AskSubscribeResponse = bool;
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EndSaleResponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WithdrawResponse {
    token_id: String,
    txn_type: WithdrawResponse_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WithdrawResponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WithdrawResponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_network_updated {
        network: Principal,
        extensible: Box<CandyShared>,
    },
    escrow_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account>,
        extensible: Box<CandyShared>,
    },
    data {
        hash: Option<serde_bytes::ByteBuf>,
        extensible: Box<CandyShared>,
        data_dapp: Option<String>,
        data_path: Option<String>,
    },
    sale_ended {
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account,
        from: Account,
        sale: Option<WithdrawResponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        receiver: Account,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account,
        from: Account,
        extensible: Box<CandyShared>,
    },
    sale_opened {
        pricing: PricingConfigShared,
        extensible: Box<CandyShared>,
        sale_id: String,
    },
    canister_owner_updated {
        owner: Principal,
        extensible: Box<CandyShared>,
    },
    sale_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PricingConfigShared {
    ask(AskConfigShared),
    extensible(Box<CandyShared>),
    instant,
    auction(AuctionConfig),
}
pub type AskConfigShared = Option<Vec<AskFeature>>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AskFeature_min_increase {
    amount(candid::Nat),
    percentage(f64),
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AskFeature {
    kyc(Principal),
    start_price(candid::Nat),
    token(TokenSpec),
    notify(Vec<Principal>),
    wait_for_quiet {
        max: candid::Nat,
        fade: f64,
        extension: u64,
    },
    reserve(candid::Nat),
    start_date(candid::Int),
    min_increase(AskFeature_min_increase),
    allow_list(Vec<Principal>),
    buy_now(candid::Nat),
    nifty_settlement {
        fixed: bool,
        interestRatePerSecond: f64,
        duration: Option<candid::Int>,
        expiration: Option<candid::Int>,
        lenderOffer: bool,
    },
    atomic,
    dutch(DutchParams),
    ending(AskFeature_ending),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AskFeature_ending {
    date(candid::Int),
    timeout(candid::Nat),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DutchParams {
    time_unit: DutchParams_time_unit,
    decay_type: DutchParams_decay_type,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DutchParams_time_unit {
    day(candid::Nat),
    hour(candid::Nat),
    minute(candid::Nat),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DutchParams_decay_type {
    flat(candid::Nat),
    percent(f64),
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuctionConfig {
    start_price: candid::Nat,
    token: TokenSpec,
    reserve: Option<candid::Nat>,
    start_date: candid::Int,
    min_increase: AuctionConfig_min_increase,
    allow_list: Option<Vec<Principal>>,
    buy_now: Option<candid::Nat>,
    ending: AuctionConfig_ending,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecognizeEscrowResponse {
    balance: candid::Nat,
    receipt: EscrowReceipt,
    transaction: Option<TransactionRecord>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EscrowReceipt {
    pub token: TokenSpec,
    pub token_id: String,
    pub seller: Account,
    pub buyer: Account,
    pub amount: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransactionRecord {
    token_id: String,
    txn_type: TransactionRecord_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuctionConfig_ending {
    date(candid::Int),
    wait_for_quiet {
        max: candid::Nat,
        date: candid::Int,
        fade: f64,
        extension: u64,
    },
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransactionRecord_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionRecord_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_network_updated {
        network: Principal,
        extensible: Box<CandyShared>,
    },
    escrow_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account>,
        extensible: Box<CandyShared>,
    },
    data {
        hash: Option<serde_bytes::ByteBuf>,
        extensible: Box<CandyShared>,
        data_dapp: Option<String>,
        data_path: Option<String>,
    },
    sale_ended {
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account,
        from: Account,
        sale: Option<TransactionRecord_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        receiver: Account,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account,
        from: Account,
        extensible: Box<CandyShared>,
    },
    sale_opened {
        pricing: PricingConfigShared,
        extensible: Box<CandyShared>,
        sale_id: String,
    },
    canister_owner_updated {
        owner: Principal,
        extensible: Box<CandyShared>,
    },
    sale_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuctionConfig_min_increase {
    amount(candid::Nat),
    percentage(f64),
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EscrowResponse {
    balance: candid::Nat,
    receipt: EscrowReceipt,
    transaction: TransactionRecord,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BidRequest {
    pub broker_id: Option<Principal>,
    pub escrow_receipt: EscrowReceipt,
    pub sale_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DepositDetail {
    pub token: TokenSpec,
    pub trx_id: Option<TransactionID>,
    pub seller: Account,
    pub buyer: Account,
    pub amount: candid::Nat,
    pub sale_id: Option<String>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EscrowRequest {
    pub token_id: String,
    pub deposit: DepositDetail,
    pub lock_to_date: Option<candid::Int>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WithdrawRequest {
    reject(RejectDescription),
    sale(WithdrawDescription),
    deposit(DepositWithdrawDescription),
    escrow(WithdrawDescription),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TokenSpec {
    ic(ICTokenSpec),
    extensible(Box<CandyShared>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WithdrawDescription {
    token: TokenSpec,
    token_id: String,
    seller: Account,
    withdraw_to: Account,
    buyer: Account,
    amount: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DepositWithdrawDescription {
    pub token: TokenSpec,
    pub withdraw_to: Account,
    pub buyer: Account,
    pub amount: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RejectDescription {
    token: TokenSpec,
    token_id: String,
    seller: Account,
    buyer: Account,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManageSaleRequest {
    bid(BidRequest),
    escrow_deposit(EscrowRequest),
    recognize_escrow(EscrowRequest),
    withdraw(WithdrawRequest),
    ask_subscribe(AskSubscribeRequest),
    end_sale(String),
    refresh_offers(Option<Account>),
    distribute_sale(DistributeSaleRequest),
    open_sale(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenIDFilter_tokens_item {
    token: TokenSpec,
    min_amount: Option<candid::Nat>,
    max_amount: Option<candid::Nat>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TokenIDFilter_filter_type {
    allow,
    block,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenIDFilter {
    filter_type: TokenIDFilter_filter_type,
    token_id: String,
    tokens: Vec<TokenIDFilter_tokens_item>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TokenSpecFilter_filter_type {
    allow,
    block,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenSpecFilter {
    token: TokenSpec,
    filter_type: TokenSpecFilter_filter_type,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AskSubscribeRequest_subscribe_filter_inner {
    tokens: Option<Vec<TokenSpecFilter>>,
    token_ids: Option<Vec<TokenIDFilter>>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AskSubscribeRequest {
    subscribe {
        stake: (Principal, candid::Nat),
        filter: Option<AskSubscribeRequest_subscribe_filter_inner>,
    },
    unsubscribe(Principal, candid::Nat),
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SaleStatusShared {
    pub token_id: String,
    pub sale_type: SaleStatusShared_sale_type,
    pub broker_id: Option<Principal>,
    pub original_broker_id: Option<Principal>,
    pub sale_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuctionStateShared_status {
    closed,
    open,
    not_started,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuctionStateShared {
    pub status: AuctionStateShared_status,
    pub participants: Vec<(Principal, candid::Int)>,
    pub token: TokenSpec,
    pub current_bid_amount: candid::Nat,
    pub winner: Option<Account>,
    pub end_date: candid::Int,
    pub start_date: candid::Int,
    pub wait_for_quiet_count: Option<candid::Nat>,
    pub current_escrow: Option<EscrowReceipt>,
    pub allow_list: Option<Vec<(Principal, bool)>>,
    pub current_broker_id: Option<Principal>,
    pub min_next_bid: candid::Nat,
    pub config: PricingConfigShared,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SaleStatusShared_sale_type {
    auction(AuctionStateShared),
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubAccountInfo {
    pub account_id: serde_bytes::ByteBuf,
    pub principal: Principal,
    pub account_id_text: String,
    pub account: SubAccountInfo_account,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubAccountInfo_account {
    pub principal: Principal,
    pub sub_account: serde_bytes::ByteBuf,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DistributeSaleRequest {
    seller: Option<Account>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BidResponse {
    token_id: String,
    pub txn_type: BidResponse_txn_type,
    timestamp: candid::Int,
    pub index: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BidResponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BidResponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_network_updated {
        network: Principal,
        extensible: Box<CandyShared>,
    },
    escrow_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account>,
        extensible: Box<CandyShared>,
    },
    data {
        hash: Option<serde_bytes::ByteBuf>,
        extensible: Box<CandyShared>,
        data_dapp: Option<String>,
        data_path: Option<String>,
    },
    sale_ended {
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account,
        from: Account,
        sale: Option<BidResponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
        receiver: Account,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account,
        from: Account,
        extensible: Box<CandyShared>,
    },
    sale_opened {
        pricing: PricingConfigShared,
        extensible: Box<CandyShared>,
        sale_id: String,
    },
    canister_owner_updated {
        owner: Principal,
        extensible: Box<CandyShared>,
    },
    sale_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account,
        amount: candid::Nat,
    },
}
pub struct Service(pub Principal);
impl Service {
    pub async fn sale_nft_origyn(&self, arg0: ManageSaleRequest) -> Result<(ManageSaleResult,)> {
        ic_cdk::call(self.0, "sale_nft_origyn", (arg0,)).await
    }
}
