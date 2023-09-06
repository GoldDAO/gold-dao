// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{ self, CandidType, Deserialize, Principal };
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum Nft_Canister___set_time_mode_arg0 {
    test,
    standard,
}

#[derive(CandidType, Deserialize)]
pub struct PropertyShared {
    value: Box<CandyShared>,
    name: String,
    immutable: bool,
}

#[derive(CandidType, Deserialize)]
pub enum CandyShared {
    Int(candid::Int),
    Map(Vec<(Box<CandyShared>, Box<CandyShared>)>),
    Nat(candid::Nat),
    Set(Vec<Box<CandyShared>>),
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
    Array(Vec<Box<CandyShared>>),
    Class(Vec<PropertyShared>),
}

#[derive(CandidType, Deserialize)]
pub enum Account {
    account_id(String),
    principal(Principal),
    extensible(Box<CandyShared>),
    account {
        owner: Principal,
        sub_account: Option<serde_bytes::ByteBuf>,
    },
}

#[derive(CandidType, Deserialize)]
pub enum ICTokenSpec__1_standard {
    ICRC1,
    EXTFungible,
    DIP20,
    Other(Box<CandyShared>),
    Ledger,
}

#[derive(CandidType, Deserialize)]
pub struct ICTokenSpec__1 {
    id: Option<candid::Nat>,
    fee: Option<candid::Nat>,
    decimals: candid::Nat,
    canister: Principal,
    standard: ICTokenSpec__1_standard,
    symbol: String,
}

#[derive(CandidType, Deserialize)]
pub enum TokenSpec__1 {
    ic(ICTokenSpec__1),
    extensible(Box<CandyShared>),
}

#[derive(CandidType, Deserialize)]
pub struct EscrowRecord {
    token: TokenSpec__1,
    token_id: String,
    seller: Account,
    lock_to_date: Option<candid::Int>,
    buyer: Account,
    amount: candid::Nat,
    sale_id: Option<String>,
    account_hash: Option<serde_bytes::ByteBuf>,
}

pub type StableSalesBalances = Vec<(Account, Account, String, EscrowRecord)>;
pub type StableOffers = Vec<(Account, Account, candid::Int)>;
#[derive(CandidType, Deserialize)]
pub struct StableCollectionData {
    active_bucket: Option<Principal>,
    managers: Vec<Principal>,
    owner: Principal,
    metadata: Option<Box<CandyShared>>,
    logo: Option<String>,
    name: Option<String>,
    network: Option<Principal>,
    available_space: candid::Nat,
    symbol: Option<String>,
    allocated_storage: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum ICTokenSpec_standard {
    ICRC1,
    EXTFungible,
    DIP20,
    Other(Box<CandyShared>),
    Ledger,
}

#[derive(CandidType, Deserialize)]
pub struct ICTokenSpec {
    id: Option<candid::Nat>,
    fee: Option<candid::Nat>,
    decimals: candid::Nat,
    canister: Principal,
    standard: ICTokenSpec_standard,
    symbol: String,
}

#[derive(CandidType, Deserialize)]
pub enum TokenSpec {
    ic(ICTokenSpec),
    extensible(Box<CandyShared>),
}

#[derive(CandidType, Deserialize)]
pub enum TransactionID {
    nat(candid::Nat),
    text(String),
    extensible(Box<CandyShared>),
}

#[derive(CandidType, Deserialize)]
pub enum Account__1 {
    account_id(String),
    principal(Principal),
    extensible(Box<CandyShared>),
    account {
        owner: Principal,
        sub_account: Option<serde_bytes::ByteBuf>,
    },
}

#[derive(CandidType, Deserialize)]
pub struct TransactionRecord_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum AskFeature_min_increase {
    amount(candid::Nat),
    percentage(f64),
}

#[derive(CandidType, Deserialize)]
pub enum DutchParams_time_unit {
    day(candid::Nat),
    hour(candid::Nat),
    minute(candid::Nat),
}

#[derive(CandidType, Deserialize)]
pub enum DutchParams_decay_type {
    flat(candid::Nat),
    percent(f64),
}

#[derive(CandidType, Deserialize)]
pub struct DutchParams {
    time_unit: DutchParams_time_unit,
    decay_type: DutchParams_decay_type,
}

#[derive(CandidType, Deserialize)]
pub enum AskFeature_ending {
    date(candid::Int),
    timeout(candid::Nat),
}

#[derive(CandidType, Deserialize)]
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

pub type AskConfigShared = Option<Vec<AskFeature>>;
#[derive(CandidType, Deserialize)]
pub enum AuctionConfig_min_increase {
    amount(candid::Nat),
    percentage(f64),
}

#[derive(CandidType, Deserialize)]
pub enum AuctionConfig_ending {
    date(candid::Int),
    wait_for_quiet {
        max: candid::Nat,
        date: candid::Int,
        fade: f64,
        extension: u64,
    },
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub enum PricingConfigShared {
    ask(AskConfigShared),
    extensible(Box<CandyShared>),
    instant,
    auction(AuctionConfig),
}

#[derive(CandidType, Deserialize)]
pub enum TransactionRecord_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account__1>,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account__1,
        from: Account__1,
        sale: Option<TransactionRecord_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        receiver: Account__1,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account__1,
        from: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct TransactionRecord {
    token_id: String,
    txn_type: TransactionRecord_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

pub type StableNftLedger = Vec<(String, TransactionRecord)>;
#[derive(CandidType, Deserialize)]
pub struct AllocationRecordStable {
    allocated_space: candid::Nat,
    token_id: String,
    available_space: candid::Nat,
    canister: Principal,
    chunks: Vec<candid::Nat>,
    library_id: String,
}

#[derive(CandidType, Deserialize)]
pub enum AuctionStateShared_status {
    closed,
    open,
    not_started,
}

#[derive(CandidType, Deserialize)]
pub struct EscrowReceipt {
    token: TokenSpec__1,
    token_id: String,
    seller: Account,
    buyer: Account,
    amount: candid::Nat,
}

pub type AskConfigShared__1 = Option<Vec<AskFeature>>;
#[derive(CandidType, Deserialize)]
pub enum AuctionConfig__1_min_increase {
    amount(candid::Nat),
    percentage(f64),
}

#[derive(CandidType, Deserialize)]
pub enum AuctionConfig__1_ending {
    date(candid::Int),
    wait_for_quiet {
        max: candid::Nat,
        date: candid::Int,
        fade: f64,
        extension: u64,
    },
}

#[derive(CandidType, Deserialize)]
pub struct AuctionConfig__1 {
    start_price: candid::Nat,
    token: TokenSpec__1,
    reserve: Option<candid::Nat>,
    start_date: candid::Int,
    min_increase: AuctionConfig__1_min_increase,
    allow_list: Option<Vec<Principal>>,
    buy_now: Option<candid::Nat>,
    ending: AuctionConfig__1_ending,
}

#[derive(CandidType, Deserialize)]
pub enum PricingConfigShared__1 {
    ask(AskConfigShared__1),
    extensible(Box<CandyShared>),
    instant,
    auction(AuctionConfig__1),
}

#[derive(CandidType, Deserialize)]
pub struct AuctionStateShared {
    status: AuctionStateShared_status,
    participants: Vec<(Principal, candid::Int)>,
    token: TokenSpec__1,
    current_bid_amount: candid::Nat,
    winner: Option<Account>,
    end_date: candid::Int,
    start_date: candid::Int,
    wait_for_quiet_count: Option<candid::Nat>,
    current_escrow: Option<EscrowReceipt>,
    allow_list: Option<Vec<(Principal, bool)>>,
    current_broker_id: Option<Principal>,
    min_next_bid: candid::Nat,
    config: PricingConfigShared__1,
}

#[derive(CandidType, Deserialize)]
pub enum SaleStatusShared_sale_type {
    auction(AuctionStateShared),
}

#[derive(CandidType, Deserialize)]
pub struct SaleStatusShared {
    token_id: String,
    sale_type: SaleStatusShared_sale_type,
    broker_id: Option<Principal>,
    original_broker_id: Option<Principal>,
    sale_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct StableBucketData {
    principal: Principal,
    allocated_space: candid::Nat,
    date_added: candid::Int,
    version: (candid::Nat, candid::Nat, candid::Nat),
    b_gateway: bool,
    available_space: candid::Nat,
    allocations: Vec<((String, String), candid::Int)>,
}

pub type StableEscrowBalances = Vec<(Account, Account, String, EscrowRecord)>;
#[derive(CandidType, Deserialize)]
pub struct NFTBackupChunk {
    sales_balances: StableSalesBalances,
    offers: StableOffers,
    collection_data: StableCollectionData,
    nft_ledgers: StableNftLedger,
    canister: Principal,
    allocations: Vec<((String, String), AllocationRecordStable)>,
    nft_sales: Vec<(String, SaleStatusShared)>,
    buckets: Vec<(Principal, StableBucketData)>,
    escrow_balances: StableEscrowBalances,
}

#[derive(CandidType, Deserialize)]
pub enum Nft_Canister_back_up_ret0 {
    eof(NFTBackupChunk),
    data(NFTBackupChunk),
}

pub type EXTTokenIdentifier = String;
#[derive(CandidType, Deserialize)]
pub enum EXTUser {
    principal(Principal),
    address(String),
}

#[derive(CandidType, Deserialize)]
pub struct EXTBalanceRequest {
    token: EXTTokenIdentifier,
    user: EXTUser,
}

pub type EXTBalance = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum EXTCommonError {
    InvalidToken(EXTTokenIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
pub enum EXTBalanceResult {
    ok(EXTBalance),
    err(EXTCommonError),
}

#[derive(CandidType, Deserialize)]
pub struct StakeRecord {
    staker: Account,
    token_id: String,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct BalanceResponse {
    nfts: Vec<String>,
    offers: Vec<EscrowRecord>,
    sales: Vec<EscrowRecord>,
    stake: Vec<StakeRecord>,
    multi_canister: Option<Vec<Principal>>,
    escrow: Vec<EscrowRecord>,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub struct OrigynError {
    text: String,
    error: Errors,
    number: u32,
    flag_point: String,
}

#[derive(CandidType, Deserialize)]
pub enum BalanceResult {
    ok(BalanceResponse),
    err(OrigynError),
}

pub type EXTAccountIdentifier = String;
#[derive(CandidType, Deserialize)]
pub enum EXTBearerResult {
    ok(EXTAccountIdentifier),
    err(EXTCommonError),
}

#[derive(CandidType, Deserialize)]
pub enum BearerResult {
    ok(Account),
    err(OrigynError),
}

pub type canister_id = Principal;
#[derive(CandidType, Deserialize)]
pub struct Nft_Canister_canister_status_arg0 {
    canister_id: canister_id,
}

#[derive(CandidType, Deserialize)]
pub enum canister_status_status {
    stopped,
    stopping,
    running,
}

#[derive(CandidType, Deserialize)]
pub struct definite_canister_settings {
    freezing_threshold: candid::Nat,
    controllers: Option<Vec<Principal>>,
    memory_allocation: candid::Nat,
    compute_allocation: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct canister_status {
    status: canister_status_status,
    memory_size: candid::Nat,
    cycles: candid::Nat,
    settings: definite_canister_settings,
    module_hash: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct ChunkRequest {
    token_id: String,
    chunk: Option<candid::Nat>,
    library_id: String,
}

#[derive(CandidType, Deserialize)]
pub enum ChunkContent {
    remote {
        args: ChunkRequest,
        canister: Principal,
    },
    chunk {
        total_chunks: candid::Nat,
        content: serde_bytes::ByteBuf,
        storage_allocation: AllocationRecordStable,
        current_chunk: Option<candid::Nat>,
    },
}

#[derive(CandidType, Deserialize)]
pub enum ChunkResult {
    ok(ChunkContent),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct CollectionInfo {
    multi_canister_count: Option<candid::Nat>,
    managers: Option<Vec<Principal>>,
    owner: Option<Principal>,
    metadata: Option<Box<CandyShared>>,
    logo: Option<String>,
    name: Option<String>,
    network: Option<Principal>,
    created_at: Option<u64>,
    fields: Option<Vec<(String, Option<candid::Nat>, Option<candid::Nat>)>>,
    upgraded_at: Option<u64>,
    token_ids_count: Option<candid::Nat>,
    available_space: Option<candid::Nat>,
    multi_canister: Option<Vec<Principal>>,
    token_ids: Option<Vec<String>>,
    transaction_count: Option<candid::Nat>,
    unique_holders: Option<candid::Nat>,
    total_supply: Option<candid::Nat>,
    symbol: Option<String>,
    allocated_storage: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub enum CollectionResult {
    ok(CollectionInfo),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub enum ManageCollectionCommand {
    UpdateOwner(Principal),
    UpdateManagers(Vec<Principal>),
    UpdateMetadata(String, Option<Box<CandyShared>>, bool),
    UpdateAnnounceCanister(Option<Principal>),
    UpdateNetwork(Option<Principal>),
    UpdateSymbol(Option<String>),
    UpdateLogo(Option<String>),
    UpdateName(Option<String>),
}

#[derive(CandidType, Deserialize)]
pub enum OrigynBoolResult {
    ok(bool),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub enum NftError {
    UnauthorizedOperator,
    SelfTransfer,
    TokenNotFound,
    UnauthorizedOwner,
    TxNotFound,
    SelfApprove,
    OperatorNotFound,
    ExistedNFT,
    OwnerNotFound,
    Other(String),
}

#[derive(CandidType, Deserialize)]
pub enum DIP721BoolResult {
    Ok(bool),
    Err(NftError),
}

#[derive(CandidType, Deserialize)]
pub struct DIP721Metadata {
    logo: Option<String>,
    name: Option<String>,
    created_at: u64,
    upgraded_at: u64,
    custodians: Vec<Principal>,
    symbol: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum DIP721TokensListMetadata {
    Ok(Vec<candid::Nat>),
    Err(NftError),
}

#[derive(CandidType, Deserialize)]
pub enum Vec_item_1 {
    Nat64Content(u64),
    Nat32Content(u32),
    BoolContent(bool),
    Nat8Content(u8),
    Int64Content(i64),
    IntContent(candid::Int),
    NatContent(candid::Nat),
    Nat16Content(u16),
    Int32Content(i32),
    Int8Content(i8),
    FloatContent(f64),
    Int16Content(i16),
    BlobContent(serde_bytes::ByteBuf),
    Principal(Principal),
    TextContent(String),
}

// #[derive(CandidType, Deserialize)]
// pub struct Vec(Vec<(String, Vec_item_1)>);

#[derive(CandidType, Deserialize)]
pub enum GenericValue {
    Nat64Content(u64),
    Nat32Content(u32),
    BoolContent(bool),
    Nat8Content(u8),
    Int64Content(i64),
    IntContent(candid::Int),
    NatContent(candid::Nat),
    Nat16Content(u16),
    Int32Content(i32),
    Int8Content(i8),
    FloatContent(f64),
    Int16Content(i16),
    BlobContent(serde_bytes::ByteBuf),
    Principal(Principal),
    TextContent(String),
}

#[derive(CandidType, Deserialize)]
pub struct TokenMetadata {
    transferred_at: Option<u64>,
    transferred_by: Option<Principal>,
    owner: Option<Principal>,
    operator: Option<Principal>,
    approved_at: Option<u64>,
    approved_by: Option<Principal>,
    properties: Vec<(String, GenericValue)>,
    is_burned: bool,
    token_identifier: candid::Nat,
    burned_at: Option<u64>,
    burned_by: Option<Principal>,
    minted_at: u64,
    minted_by: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum DIP721TokensMetadata {
    Ok(Vec<TokenMetadata>),
    Err(NftError),
}

#[derive(CandidType, Deserialize)]
pub enum OwnerOfResponse {
    Ok(Option<Principal>),
    Err(NftError),
}

#[derive(CandidType, Deserialize)]
pub struct DIP721Stats {
    cycles: candid::Nat,
    total_transactions: candid::Nat,
    total_unique_holders: candid::Nat,
    total_supply: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum DIP721SupportedInterface {
    Burn,
    Mint,
    Approval,
    TransactionHistory,
}

#[derive(CandidType, Deserialize)]
pub enum DIP721TokenMetadata {
    Ok(TokenMetadata),
    Err(NftError),
}

#[derive(CandidType, Deserialize)]
pub enum DIP721NatResult {
    Ok(candid::Nat),
    Err(NftError),
}

#[derive(CandidType, Deserialize)]
pub struct GetLogMessagesFilter {
    analyzeCount: u32,
    messageRegex: Option<String>,
    messageContains: Option<String>,
}
pub type Nanos = u64;
#[derive(CandidType, Deserialize)]
pub struct GetLogMessagesParameters {
    count: u32,
    filter: Option<GetLogMessagesFilter>,
    fromTimeNanos: Option<Nanos>,
}

#[derive(CandidType, Deserialize)]
pub struct GetLatestLogMessagesParameters {
    upToTimeNanos: Option<Nanos>,
    count: u32,
    filter: Option<GetLogMessagesFilter>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogRequest {
    getMessagesInfo,
    getMessages(GetLogMessagesParameters),
    getLatestMessages(GetLatestLogMessagesParameters),
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogFeature {
    filterMessageByContains,
    filterMessageByRegex,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessagesInfo {
    features: Vec<Option<CanisterLogFeature>>,
    lastTimeNanos: Option<Nanos>,
    count: u32,
    firstTimeNanos: Option<Nanos>,
}

#[derive(CandidType, Deserialize)]
pub enum Data {
    Int(candid::Int),
    Map(Vec<(Box<CandyShared>, Box<CandyShared>)>),
    Nat(candid::Nat),
    Set(Vec<Box<CandyShared>>),
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
    Array(Vec<Box<CandyShared>>),
    Class(Vec<PropertyShared>),
}

pub type Caller = Option<Principal>;
#[derive(CandidType, Deserialize)]
pub struct LogMessagesData {
    data: Data,
    timeNanos: Nanos,
    message: String,
    caller: Caller,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessages {
    data: Vec<LogMessagesData>,
    lastAnalyzedMessageTimeNanos: Option<Nanos>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogResponse {
    messagesInfo(CanisterLogMessagesInfo),
    messages(CanisterLogMessages),
}

#[derive(CandidType, Deserialize)]
pub enum MetricsGranularity {
    hourly,
    daily,
}

#[derive(CandidType, Deserialize)]
pub struct GetMetricsParameters {
    dateToMillis: candid::Nat,
    granularity: MetricsGranularity,
    dateFromMillis: candid::Nat,
}

pub type UpdateCallsAggregatedData = Vec<u64>;
pub type CanisterHeapMemoryAggregatedData = Vec<u64>;
pub type CanisterCyclesAggregatedData = Vec<u64>;
pub type CanisterMemoryAggregatedData = Vec<u64>;
#[derive(CandidType, Deserialize)]
pub struct HourlyMetricsData {
    updateCalls: UpdateCallsAggregatedData,
    canisterHeapMemorySize: CanisterHeapMemoryAggregatedData,
    canisterCycles: CanisterCyclesAggregatedData,
    canisterMemorySize: CanisterMemoryAggregatedData,
    timeMillis: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub struct NumericEntity {
    avg: u64,
    max: u64,
    min: u64,
    first: u64,
    last: u64,
}

#[derive(CandidType, Deserialize)]
pub struct DailyMetricsData {
    updateCalls: u64,
    canisterHeapMemorySize: NumericEntity,
    canisterCycles: NumericEntity,
    canisterMemorySize: NumericEntity,
    timeMillis: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterMetricsData {
    hourly(Vec<HourlyMetricsData>),
    daily(Vec<DailyMetricsData>),
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMetrics {
    data: CanisterMetricsData,
}

#[derive(CandidType, Deserialize)]
pub enum OrigynTextResult {
    ok(String),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub enum GovernanceRequest {
    update_system_var {
        key: String,
        val: Box<CandyShared>,
        token_id: String,
    },
    clear_shared_wallets(String),
}

#[derive(CandidType, Deserialize)]
pub enum GovernanceResponse {
    update_system_var(bool),
    clear_shared_wallets(bool),
}

#[derive(CandidType, Deserialize)]
pub enum GovernanceResult {
    ok(GovernanceResponse),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub enum HistoryResult {
    ok(Vec<TransactionRecord>),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct HeaderField(String, String);

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    url: String,
    method: String,
    body: serde_bytes::ByteBuf,
    headers: Vec<HeaderField>,
}

#[derive(CandidType, Deserialize)]
pub struct StreamingCallbackToken {
    key: String,
    index: candid::Nat,
    content_encoding: String,
}

#[derive(CandidType, Deserialize)]
pub struct StreamingCallbackResponse {
    token: Option<StreamingCallbackToken>,
    body: serde_bytes::ByteBuf,
}

pub type Subaccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct Account__2 {
    owner: Principal,
    subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize)]
pub struct ApprovalArgs {
    to: Account__2,
    tokenIds: Option<Vec<candid::Nat>>,
    memo: Option<serde_bytes::ByteBuf>,
    created_at: Option<u64>,
    from_subaccount: serde_bytes::ByteBuf,
    expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum ApprovalError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Unauthorized(Vec<candid::Nat>),
    TooOld,
}

#[derive(CandidType, Deserialize)]
pub enum ApprovalResult {
    Ok(candid::Nat),
    Err(ApprovalError),
}

#[derive(CandidType, Deserialize)]
pub struct CollectionMetadata {
    icrc7_supply_cap: Option<candid::Nat>,
    icrc7_description: Option<String>,
    icrc7_total_supply: candid::Nat,
    icrc7_royalty_recipient: Option<Account__2>,
    icrc7_royalties: Option<u16>,
    icrc7_symbol: String,
    icrc7_image: Option<String>,
    icrc7_name: String,
}

#[derive(CandidType, Deserialize)]
pub enum Metadata {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(serde_bytes::ByteBuf),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct SupportedStandard {
    url: String,
    name: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    to: Account__2,
    from: Option<Account__2>,
    memo: Option<serde_bytes::ByteBuf>,
    is_atomic: Option<bool>,
    token_ids: Vec<candid::Nat>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Duplicate {
        duplicate_of: candid::Nat,
    },
    Unauthorized {
        token_ids: Vec<candid::Nat>,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
}

#[derive(CandidType, Deserialize)]
pub enum TransferResult {
    Ok(candid::Nat),
    Err(TransferError),
}

#[derive(CandidType, Deserialize)]
pub enum ManageStorageRequest_configure_storage {
    stableBtree(Option<candid::Nat>),
    heap(Option<candid::Nat>),
}

#[derive(CandidType, Deserialize)]
pub enum ManageStorageRequest {
    add_storage_canisters(Vec<(Principal, candid::Nat, (candid::Nat, candid::Nat, candid::Nat))>),
    configure_storage(ManageStorageRequest_configure_storage),
}

#[derive(CandidType, Deserialize)]
pub enum ManageStorageResponse {
    add_storage_canisters(candid::Nat, candid::Nat),
    configure_storage(candid::Nat, candid::Nat),
}

#[derive(CandidType, Deserialize)]
pub enum ManageStorageResult {
    ok(ManageStorageResponse),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct SalesConfig {
    broker_id: Option<Principal>,
    pricing: PricingConfigShared__1,
    escrow_receipt: Option<EscrowReceipt>,
}

#[derive(CandidType, Deserialize)]
pub struct MarketTransferRequest {
    token_id: String,
    sales_config: SalesConfig,
}

#[derive(CandidType, Deserialize)]
pub struct MarketTransferRequestReponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum MarketTransferRequestReponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account__1>,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account__1,
        from: Account__1,
        sale: Option<MarketTransferRequestReponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        receiver: Account__1,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account__1,
        from: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct MarketTransferRequestReponse {
    token_id: String,
    txn_type: MarketTransferRequestReponse_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum MarketTransferResult {
    ok(MarketTransferRequestReponse),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub enum EXTMetadata {
    fungible {
        decimals: u8,
        metadata: Option<serde_bytes::ByteBuf>,
        name: String,
        symbol: String,
    },
    nonfungible {
        metadata: Option<serde_bytes::ByteBuf>,
    },
}

#[derive(CandidType, Deserialize)]
pub enum EXTMetadataResult {
    ok(EXTMetadata),
    err(EXTCommonError),
}

#[derive(CandidType, Deserialize)]
pub struct NFTInfoStable {
    metadata: Box<CandyShared>,
    current_sale: Option<SaleStatusShared>,
}

#[derive(CandidType, Deserialize)]
pub enum NFTInfoResult {
    ok(NFTInfoStable),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct BidRequest {
    broker_id: Option<Principal>,
    escrow_receipt: EscrowReceipt,
    sale_id: String,
}

#[derive(CandidType, Deserialize)]
pub enum TransactionID__1 {
    nat(candid::Nat),
    text(String),
    extensible(Box<CandyShared>),
}

#[derive(CandidType, Deserialize)]
pub struct DepositDetail {
    token: TokenSpec__1,
    trx_id: Option<TransactionID__1>,
    seller: Account,
    buyer: Account,
    amount: candid::Nat,
    sale_id: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct EscrowRequest {
    token_id: String,
    deposit: DepositDetail,
    lock_to_date: Option<candid::Int>,
}

#[derive(CandidType, Deserialize)]
pub struct RejectDescription {
    token: TokenSpec__1,
    token_id: String,
    seller: Account,
    buyer: Account,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawDescription {
    token: TokenSpec__1,
    token_id: String,
    seller: Account,
    withdraw_to: Account,
    buyer: Account,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct DepositWithdrawDescription {
    token: TokenSpec__1,
    withdraw_to: Account,
    buyer: Account,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum WithdrawRequest {
    reject(RejectDescription),
    sale(WithdrawDescription),
    deposit(DepositWithdrawDescription),
    escrow(WithdrawDescription),
}

#[derive(CandidType, Deserialize)]
pub enum TokenSpecFilter_filter_type {
    allow,
    block,
}

#[derive(CandidType, Deserialize)]
pub struct TokenSpecFilter {
    token: TokenSpec__1,
    filter_type: TokenSpecFilter_filter_type,
}

#[derive(CandidType, Deserialize)]
pub enum TokenIDFilter_filter_type {
    allow,
    block,
}

#[derive(CandidType, Deserialize)]
pub struct TokenIDFilter_tokens_item {
    token: TokenSpec__1,
    min_amount: Option<candid::Nat>,
    max_amount: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct TokenIDFilter {
    filter_type: TokenIDFilter_filter_type,
    token_id: String,
    tokens: Vec<TokenIDFilter_tokens_item>,
}

#[derive(CandidType, Deserialize)]
pub struct AskSubscribeRequest_subscribe_filter_inner {
    tokens: Option<Vec<TokenSpecFilter>>,
    token_ids: Option<Vec<TokenIDFilter>>,
}

#[derive(CandidType, Deserialize)]
pub enum AskSubscribeRequest {
    subscribe {
        stake: (Principal, candid::Nat),
        filter: Option<AskSubscribeRequest_subscribe_filter_inner>,
    },
    unsubscribe(Principal, candid::Nat),
}

#[derive(CandidType, Deserialize)]
pub struct DistributeSaleRequest {
    seller: Option<Account>,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub struct BidResponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum BidResponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account__1>,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account__1,
        from: Account__1,
        sale: Option<BidResponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        receiver: Account__1,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account__1,
        from: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct BidResponse {
    token_id: String,
    txn_type: BidResponse_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct EscrowResponse {
    balance: candid::Nat,
    receipt: EscrowReceipt,
    transaction: TransactionRecord,
}

#[derive(CandidType, Deserialize)]
pub struct RecognizeEscrowResponse {
    balance: candid::Nat,
    receipt: EscrowReceipt,
    transaction: Option<TransactionRecord>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawResponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum WithdrawResponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account__1>,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account__1,
        from: Account__1,
        sale: Option<WithdrawResponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        receiver: Account__1,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account__1,
        from: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawResponse {
    token_id: String,
    txn_type: WithdrawResponse_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

pub type AskSubscribeResponse = bool;
#[derive(CandidType, Deserialize)]
pub struct EndSaleResponse_txn_type_mint_sale_inner {
    token: TokenSpec,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum EndSaleResponse_txn_type {
    escrow_deposit {
        token: TokenSpec,
        token_id: String,
        trx_id: TransactionID,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    canister_managers_updated {
        managers: Vec<Principal>,
        extensible: Box<CandyShared>,
    },
    auction_bid {
        token: TokenSpec,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: String,
    },
    burn {
        from: Option<Account__1>,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        sale_id: Option<String>,
    },
    mint {
        to: Account__1,
        from: Account__1,
        sale: Option<EndSaleResponse_txn_type_mint_sale_inner>,
        extensible: Box<CandyShared>,
    },
    royalty_paid {
        tag: String,
        token: TokenSpec,
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
        receiver: Account__1,
        sale_id: Option<String>,
    },
    extensible(Box<CandyShared>),
    owner_transfer {
        to: Account__1,
        from: Account__1,
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
        seller: Account__1,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
    deposit_withdraw {
        fee: candid::Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: Box<CandyShared>,
        buyer: Account__1,
        amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct EndSaleResponse {
    token_id: String,
    txn_type: EndSaleResponse_txn_type,
    timestamp: candid::Int,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum ManageSaleResponse {
    bid(BidResponse),
    escrow_deposit(EscrowResponse),
    recognize_escrow(RecognizeEscrowResponse),
    withdraw(WithdrawResponse),
    ask_subscribe(AskSubscribeResponse),
    end_sale(EndSaleResponse),
    refresh_offers(Vec<EscrowRecord>),
    open_sale(bool),
}

#[derive(CandidType, Deserialize)]
pub enum ManageSaleResult {
    ok(Box<ManageSaleResponse>),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub enum SaleInfoRequest {
    status(String),
    active(Option<(candid::Nat, candid::Nat)>),
    deposit_info(Option<Account>),
    history(Option<(candid::Nat, candid::Nat)>),
    escrow_info(EscrowReceipt),
}

#[derive(CandidType, Deserialize)]
pub struct SubAccountInfo_account {
    principal: Principal,
    sub_account: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct SubAccountInfo {
    account_id: serde_bytes::ByteBuf,
    principal: Principal,
    account_id_text: String,
    account: SubAccountInfo_account,
}

#[derive(CandidType, Deserialize)]
pub enum SaleInfoResponse {
    status(Option<SaleStatusShared>),
    active {
        eof: bool,
        records: Vec<(String, Option<SaleStatusShared>)>,
        count: candid::Nat,
    },
    deposit_info(SubAccountInfo),
    history {
        eof: bool,
        records: Vec<Option<SaleStatusShared>>,
        count: candid::Nat,
    },
    escrow_info(SubAccountInfo),
}

#[derive(CandidType, Deserialize)]
pub enum SaleInfoResult {
    ok(SaleInfoResponse),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct ShareWalletRequest {
    to: Account,
    token_id: String,
    from: Account,
}

#[derive(CandidType, Deserialize)]
pub struct OwnerTransferResponse {
    transaction: TransactionRecord,
    assets: Vec<Box<CandyShared>>,
}

#[derive(CandidType, Deserialize)]
pub enum OwnerUpdateResult {
    ok(OwnerTransferResponse),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct Nft_Canister_stage_batch_nft_origyn_arg0_item {
    metadata: Box<CandyShared>,
}

#[derive(CandidType, Deserialize)]
pub struct StageChunkArg {
    content: serde_bytes::ByteBuf,
    token_id: String,
    chunk: candid::Nat,
    filedata: Box<CandyShared>,
    library_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct StageLibraryResponse {
    canister: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum StageLibraryResult {
    ok(StageLibraryResponse),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct Nft_Canister_stage_nft_origyn_arg0 {
    metadata: Box<CandyShared>,
}

#[derive(CandidType, Deserialize)]
pub struct StateSize {
    sales_balances: candid::Nat,
    offers: candid::Nat,
    nft_ledgers: candid::Nat,
    allocations: candid::Nat,
    nft_sales: candid::Nat,
    buckets: candid::Nat,
    escrow_balances: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct StorageMetrics {
    gateway: Principal,
    available_space: candid::Nat,
    allocations: Vec<AllocationRecordStable>,
    allocated_storage: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum StorageMetricsResult {
    ok(StorageMetrics),
    err(OrigynError),
}

#[derive(CandidType, Deserialize)]
pub struct EXTTokensResponse_1_inner {
    locked: Option<candid::Int>,
    seller: Principal,
    price: u64,
}

#[derive(CandidType, Deserialize)]
pub struct EXTTokensResponse(u32, Option<EXTTokensResponse_1_inner>, Option<serde_bytes::ByteBuf>);

#[derive(CandidType, Deserialize)]
pub enum EXTTokensResult {
    ok(Vec<EXTTokensResponse>),
    err(EXTCommonError),
}

pub type EXTMemo = serde_bytes::ByteBuf;
pub type EXTSubAccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct EXTTransferRequest {
    to: EXTUser,
    token: EXTTokenIdentifier,
    notify: bool,
    from: EXTUser,
    memo: EXTMemo,
    subaccount: Option<EXTSubAccount>,
    amount: EXTBalance,
}

#[derive(CandidType, Deserialize)]
pub enum EXTTransferResponse_err {
    CannotNotify(EXTAccountIdentifier),
    InsufficientBalance,
    InvalidToken(EXTTokenIdentifier),
    Rejected,
    Unauthorized(EXTAccountIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
pub enum EXTTransferResponse {
    ok(EXTBalance),
    err(EXTTransferResponse_err),
}

#[derive(CandidType, Deserialize)]
pub enum UpdateModeShared {
    Set(Box<CandyShared>),
    Lock(Box<CandyShared>),
    Next(Vec<Box<UpdateShared>>),
}

#[derive(CandidType, Deserialize)]
pub struct UpdateShared {
    mode: UpdateModeShared,
    name: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateRequestShared {
    id: String,
    update: Vec<Box<UpdateShared>>,
}

#[derive(CandidType, Deserialize)]
pub enum NFTUpdateRequest {
    update {
        token_id: String,
        update: UpdateRequestShared,
        app_id: String,
    },
    replace {
        token_id: String,
        data: Box<CandyShared>,
    },
}

pub type NFTUpdateResponse = bool;
#[derive(CandidType, Deserialize)]
pub enum NFTUpdateResult {
    ok(NFTUpdateResponse),
    err(OrigynError),
}

pub struct SERVICE(pub Principal);
impl SERVICE {
    pub async fn sale_nft_origyn(&self, arg0: ManageSaleRequest) -> Result<(ManageSaleResult,)> {
        ic_cdk::call(self.0, "sale_nft_origyn", (arg0,)).await
    }
}
