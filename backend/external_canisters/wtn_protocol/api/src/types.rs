// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct UpgradeArg {
    pub governance_fee_share_percent: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct InitArg {
    pub wtn_ledger_id: Principal,
    pub wtn_governance_id: Principal,
    pub nicp_ledger_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum LiquidArg {
    Upgrade(Option<UpgradeArg>),
    Init(InitArg),
}

#[derive(CandidType, Deserialize)]
pub struct NeuronId {
    pub id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct BallotInfo {
    pub vote: i32,
    pub proposal_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
pub enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}

#[derive(CandidType, Deserialize)]
pub struct Followees {
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
pub struct NeuronStakeTransfer {
    pub to_subaccount: serde_bytes::ByteBuf,
    pub neuron_stake_e8s: u64,
    pub from: Option<Principal>,
    pub memo: u64,
    pub from_subaccount: serde_bytes::ByteBuf,
    pub transfer_timestamp: u64,
    pub block_height: u64,
}

#[derive(CandidType, Deserialize)]
pub struct KnownNeuronData {
    pub name: String,
    pub description: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct Neuron {
    pub id: Option<NeuronId>,
    pub staked_maturity_e8s_equivalent: Option<u64>,
    pub controller: Option<Principal>,
    pub recent_ballots: Vec<BallotInfo>,
    pub kyc_verified: bool,
    pub neuron_type: Option<i32>,
    pub not_for_profit: bool,
    pub maturity_e8s_equivalent: u64,
    pub cached_neuron_stake_e8s: u64,
    pub created_timestamp_seconds: u64,
    pub auto_stake_maturity: Option<bool>,
    pub aging_since_timestamp_seconds: u64,
    pub hot_keys: Vec<Principal>,
    pub account: serde_bytes::ByteBuf,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub dissolve_state: Option<DissolveState>,
    pub followees: Vec<(i32, Followees)>,
    pub neuron_fees_e8s: u64,
    pub transfer: Option<NeuronStakeTransfer>,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub spawn_at_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct NeuronInfo {
    pub dissolve_delay_seconds: u64,
    pub recent_ballots: Vec<BallotInfo>,
    pub neuron_type: Option<i32>,
    pub created_timestamp_seconds: u64,
    pub state: i32,
    pub stake_e8s: u64,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub retrieved_at_timestamp_seconds: u64,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub voting_power: u64,
    pub age_seconds: u64,
}

#[derive(CandidType, Deserialize)]
pub struct MergeResponse {
    pub target_neuron: Option<Neuron>,
    pub source_neuron: Option<Neuron>,
    pub target_neuron_info: Option<NeuronInfo>,
    pub source_neuron_info: Option<NeuronInfo>,
}

#[derive(CandidType, Deserialize)]
pub struct GovernanceError {
    pub error_message: String,
    pub error_type: i32,
}

#[derive(CandidType, Deserialize)]
pub enum GuardError {
    AlreadyProcessing,
    TooManyConcurrentRequests,
}

#[derive(CandidType, Deserialize)]
pub enum CancelWithdrawalError {
    GenericError { code: i32, message: String },
    TooLate,
    BadCommand { message: String },
    UnknownTimeLeft,
    BadCaller { message: String },
    MergeNeuronError { message: String },
    StopDissolvementError { message: String },
    RequestNotFound,
    GovernanceError(GovernanceError),
    GuardError { guard_error: GuardError },
    GetFullNeuronError { message: String },
}

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok(MergeResponse),
    Err(CancelWithdrawalError),
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    InsufficientAllowance {
        allowance: candid::Nat,
    },
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum ConversionError {
    GenericError { code: i32, message: String },
    TransferError(TransferError),
    AmountTooLow { minimum_amount_e8s: u64 },
    TransferFromError(TransferFromError),
    GuardError { guard_error: GuardError },
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(u64),
    Err(ConversionError),
}

#[derive(CandidType, Deserialize)]
pub struct GetEventsArg {
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize)]
pub enum NeuronOrigin {
    #[serde(rename = "NICPSixMonths")]
    NicpSixMonths,
    SnsGovernanceEightYears,
}

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub enum EventType {
    ClaimedAirdrop {
        block_index: u64,
        caller: Principal,
    },
    StartedToDissolve {
        withdrawal_id: u64,
    },
    MaturityNeuron {
        from_neuron_type: NeuronOrigin,
        neuron_id: NeuronId,
    },
    NeuronSixMonths(NeuronId),
    Upgrade(UpgradeArg),
    Init(InitArg),
    MirroredProposal {
        nns_proposal_id: NeuronId,
        sns_proposal_id: NeuronId,
    },
    NeuronEightYears(NeuronId),
    #[serde(rename = "DistributeICPtoSNS")]
    DistributeIcPtoSns {
        amount: u64,
        receiver: Principal,
    },
    NIcpWithdrawal {
        nicp_burned: u64,
        nicp_burn_index: u64,
        receiver: Account,
    },
    MergeNeuron {
        neuron_id: NeuronId,
    },
    IcpDeposit {
        block_index: u64,
        amount: u64,
        receiver: Account,
    },
    DisbursedUserNeuron {
        withdrawal_id: u64,
        transfer_block_height: u64,
    },
    TransferExecuted {
        block_index: Option<u64>,
        transfer_id: u64,
    },
    DisbursedMaturityNeuron {
        transfer_block_height: u64,
        neuron_id: NeuronId,
    },
    #[serde(rename = "DispatchICPRewards")]
    DispatchIcpRewards {
        nicp_amount: u64,
        sns_gov_amount: u64,
        from_neuron_type: NeuronOrigin,
    },
    #[serde(rename = "DistributeICPtoSNSv2")]
    DistributeIcPtoSnSv2,
    SplitNeuron {
        withdrawal_id: u64,
        neuron_id: NeuronId,
    },
}

#[derive(CandidType, Deserialize)]
pub struct Event {
    pub timestamp: u64,
    pub payload: EventType,
}

#[derive(CandidType, Deserialize)]
pub struct GetEventsResult {
    pub total_event_count: u64,
    pub events: Vec<Event>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterInfo {
    pub neuron_6m_account: Account,
    pub latest_distribution_icp_per_vp: Option<f64>,
    pub neuron_id_6m: Option<NeuronId>,
    pub neuron_id_8y: Option<NeuronId>,
    pub tracked_6m_stake: u64,
    pub minimum_withdraw_amount: u64,
    pub neuron_8y_stake_e8s: u64,
    pub governance_fee_share_percent: u64,
    pub neuron_8y_account: Account,
    pub minimum_deposit_amount: u64,
    pub neuron_6m_stake_e8s: u64,
    pub exchange_rate: u64,
    pub nicp_supply: u64,
    pub total_icp_deposited: u64,
    pub stakers_count: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Unit {
    #[serde(rename = "ICP")]
    Icp,
    #[serde(rename = "WTN")]
    Wtn,
    #[serde(rename = "NICP")]
    Nicp,
}

#[derive(CandidType, Deserialize)]
pub struct PendingTransfer {
    pub memo: Option<u64>,
    pub unit: Unit,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub transfer_id: u64,
    pub amount: u64,
    pub receiver: Account,
}

#[derive(CandidType, Deserialize)]
pub struct ExecutedTransfer {
    pub block_index: Option<u64>,
    pub timestamp: u64,
    pub transfer: PendingTransfer,
}

#[derive(CandidType, Deserialize)]
pub enum TransferStatus {
    Executed(ExecutedTransfer),
    Unknown,
    Pending(PendingTransfer),
}

#[derive(CandidType, Deserialize)]
pub enum WithdrawalStatus {
    ConversionDone { transfer_block_height: u64 },
    NotFound,
    Cancelled,
    WaitingToSplitNeuron,
    WaitingDissolvement { neuron_id: NeuronId },
    WaitingToStartDissolving { neuron_id: NeuronId },
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawalRequest {
    pub nicp_burned: u64,
    pub withdrawal_id: u64,
    pub icp_due: u64,
    pub nicp_burn_index: u64,
    pub timestamp: u64,
    pub receiver: Account,
    pub neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawalDetails {
    pub status: WithdrawalStatus,
    pub request: WithdrawalRequest,
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(NeuronId),
    Err(NeuronId),
}

#[derive(CandidType, Deserialize)]
pub struct ConversionArg {
    pub maybe_subaccount: Option<serde_bytes::ByteBuf>,
    pub amount_e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct DepositSuccess {
    pub nicp_amount: Option<u64>,
    pub block_index: candid::Nat,
    pub transfer_id: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(DepositSuccess),
    Err(ConversionError),
}

#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageMetadata {
    pub utc_offset_minutes: Option<u16>,
    pub language: String,
}

#[derive(CandidType, Deserialize)]
pub enum DisplayMessageType {
    GenericDisplay,
    LineDisplay {
        characters_per_line: u16,
        lines_per_page: u16,
    },
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageSpec {
    pub metadata: ConsentMessageMetadata,
    pub device_spec: Option<DisplayMessageType>,
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageRequest {
    pub arg: serde_bytes::ByteBuf,
    pub method: String,
    pub user_preferences: ConsentMessageSpec,
}

#[derive(CandidType, Deserialize)]
pub struct LineDisplayPage {
    pub lines: Vec<String>,
}

#[derive(CandidType, Deserialize)]
pub enum ConsentMessage {
    LineDisplayMessage { pages: Vec<LineDisplayPage> },
    GenericDisplayMessage(String),
}

#[derive(CandidType, Deserialize)]
pub struct ConsentInfo {
    pub metadata: ConsentMessageMetadata,
    pub consent_message: ConsentMessage,
}

#[derive(CandidType, Deserialize)]
pub struct ErrorInfo {
    pub description: String,
}

#[derive(CandidType, Deserialize)]
pub enum Icrc21Error {
    GenericError {
        description: String,
        error_code: u64,
    },
    InsufficientPayment(ErrorInfo),
    UnsupportedCanisterCall(ErrorInfo),
    ConsentMessageUnavailable(ErrorInfo),
}

#[derive(CandidType, Deserialize)]
pub enum Result5 {
    Ok(ConsentInfo),
    Err(Icrc21Error),
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawalSuccess {
    pub block_index: candid::Nat,
    pub withdrawal_id: u64,
    pub icp_amount: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum Result4 {
    Ok(WithdrawalSuccess),
    Err(ConversionError),
}
