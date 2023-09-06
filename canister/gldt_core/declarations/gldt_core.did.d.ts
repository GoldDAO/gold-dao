import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type Account_1 = { 'account_id' : string } |
  { 'principal' : Principal } |
  { 'extensible' : CandyShared } |
  {
    'account' : {
      'owner' : Principal,
      'sub_account' : [] | [Uint8Array | number[]],
    }
  };
export type AskFeature = { 'kyc' : Principal } |
  { 'start_price' : bigint } |
  { 'token' : TokenSpec } |
  { 'notify' : Array<Principal> } |
  {
    'wait_for_quiet' : { 'max' : bigint, 'fade' : number, 'extension' : bigint }
  } |
  { 'reserve' : bigint } |
  { 'start_date' : bigint } |
  { 'min_increase' : AskFeature_min_increase } |
  { 'allow_list' : Array<Principal> } |
  { 'buy_now' : bigint } |
  {
    'nifty_settlement' : {
      'fixed' : boolean,
      'interestRatePerSecond' : number,
      'duration' : [] | [bigint],
      'expiration' : [] | [bigint],
      'lenderOffer' : boolean,
    }
  } |
  { 'atomic' : null } |
  { 'dutch' : DutchParams } |
  { 'ending' : AskFeature_ending };
export type AskFeature_ending = { 'date' : bigint } |
  { 'timeout' : bigint };
export type AskFeature_min_increase = { 'amount' : bigint } |
  { 'percentage' : number };
export interface AuctionConfig__1 {
  'start_price' : bigint,
  'token' : TokenSpec,
  'reserve' : [] | [bigint],
  'start_date' : bigint,
  'min_increase' : AskFeature_min_increase,
  'allow_list' : [] | [Array<Principal>],
  'buy_now' : [] | [bigint],
  'ending' : AuctionConfig__1_ending,
}
export type AuctionConfig__1_ending = { 'date' : bigint } |
  {
    'wait_for_quiet' : {
      'max' : bigint,
      'date' : bigint,
      'fade' : number,
      'extension' : bigint,
    }
  };
export interface AuctionStateShared {
  'status' : AuctionStateShared_status,
  'participants' : Array<[Principal, bigint]>,
  'token' : TokenSpec,
  'current_bid_amount' : bigint,
  'winner' : [] | [Account_1],
  'end_date' : bigint,
  'start_date' : bigint,
  'wait_for_quiet_count' : [] | [bigint],
  'current_escrow' : [] | [EscrowReceipt],
  'allow_list' : [] | [Array<[Principal, boolean]>],
  'current_broker_id' : [] | [Principal],
  'min_next_bid' : bigint,
  'config' : PricingConfigShared__1,
}
export type AuctionStateShared_status = { 'closed' : null } |
  { 'open' : null } |
  { 'not_started' : null };
export interface BidRequest {
  'broker_id' : [] | [Principal],
  'escrow_receipt' : EscrowReceipt,
  'sale_id' : string,
}
export type Box = { 'Int' : bigint } |
  { 'Map' : Array<[Box, Box]> } |
  { 'Nat' : bigint } |
  { 'Set' : Array<Box> } |
  { 'Nat16' : number } |
  { 'Nat32' : number } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Bool' : boolean } |
  { 'Int8' : number } |
  { 'Ints' : Array<bigint> } |
  { 'Nat8' : number } |
  { 'Nats' : Array<bigint> } |
  { 'Text' : string } |
  { 'Bytes' : Uint8Array | number[] } |
  { 'Int16' : number } |
  { 'Int32' : number } |
  { 'Int64' : bigint } |
  { 'Option' : [] | [Box] } |
  { 'Floats' : Array<number> } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array<Box> } |
  {
    'Class' : Array<{ 'value' : Box, 'name' : string, 'immutable' : boolean }>
  };
export type CandyShared = { 'Int' : bigint } |
  { 'Map' : Array<[Box, Box]> } |
  { 'Nat' : bigint } |
  { 'Set' : Array<Box> } |
  { 'Nat16' : number } |
  { 'Nat32' : number } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Bool' : boolean } |
  { 'Int8' : number } |
  { 'Ints' : Array<bigint> } |
  { 'Nat8' : number } |
  { 'Nats' : Array<bigint> } |
  { 'Text' : string } |
  { 'Bytes' : Uint8Array | number[] } |
  { 'Int16' : number } |
  { 'Int32' : number } |
  { 'Int64' : bigint } |
  { 'Option' : [] | [Box] } |
  { 'Floats' : Array<number> } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array<Box> } |
  { 'Class' : Array<PropertyShared> };
export type CanisterLogFeature = { 'filterMessageByContains' : null } |
  { 'filterMessageByRegex' : null };
export interface CanisterLogMessages {
  'data' : Array<LogMessageData>,
  'lastAnalyzedMessageTimeNanos' : [] | [bigint],
}
export interface CanisterLogMessagesInfo {
  'features' : Array<[] | [CanisterLogFeature]>,
  'lastTimeNanos' : [] | [bigint],
  'count' : number,
  'firstTimeNanos' : [] | [bigint],
}
export type CanisterLogRequest = { 'getMessagesInfo' : null } |
  { 'getMessages' : GetLogMessagesParameters } |
  { 'getLatestMessages' : GetLatestLogMessagesParameters };
export type CanisterLogResponse = { 'messagesInfo' : CanisterLogMessagesInfo } |
  { 'messages' : CanisterLogMessages };
export interface CanisterMetrics { 'data' : CanisterMetricsData }
export type CanisterMetricsData = { 'hourly' : Array<HourlyMetricsData> } |
  { 'daily' : Array<DailyMetricsData> };
export type CollectMetricsRequestType = { 'force' : null } |
  { 'normal' : null };
export interface Conf {
  'gld_nft_canister_ids' : Array<[Principal, NftCanisterConf]>,
  'gldt_ledger_canister_id' : Principal,
}
export interface DailyMetricsData {
  'updateCalls' : bigint,
  'canisterHeapMemorySize' : NumericEntity,
  'canisterCycles' : NumericEntity,
  'canisterMemorySize' : NumericEntity,
  'timeMillis' : bigint,
}
export interface DutchParams {
  'time_unit' : DutchParams_time_unit,
  'decay_type' : DutchParams_decay_type,
}
export type DutchParams_decay_type = { 'flat' : bigint } |
  { 'percent' : number };
export type DutchParams_time_unit = { 'day' : bigint } |
  { 'hour' : bigint } |
  { 'minute' : bigint };
export interface EscrowReceipt {
  'token' : TokenSpec,
  'token_id' : string,
  'seller' : Account_1,
  'buyer' : Account_1,
  'amount' : bigint,
}
export interface GetInformationRequest {
  'status' : [] | [StatusRequest],
  'metrics' : [] | [MetricsRequest],
  'logs' : [] | [CanisterLogRequest],
  'version' : boolean,
}
export interface GetInformationResponse {
  'status' : [] | [StatusResponse],
  'metrics' : [] | [MetricsResponse],
  'logs' : [] | [CanisterLogResponse],
  'version' : [] | [bigint],
}
export interface GetLatestLogMessagesParameters {
  'upToTimeNanos' : [] | [bigint],
  'count' : number,
  'filter' : [] | [GetLogMessagesFilter],
}
export interface GetLogMessagesFilter {
  'analyzeCount' : number,
  'messageRegex' : [] | [string],
  'messageContains' : [] | [string],
}
export interface GetLogMessagesParameters {
  'count' : number,
  'filter' : [] | [GetLogMessagesFilter],
  'fromTimeNanos' : [] | [bigint],
}
export interface GetMetricsParameters {
  'dateToMillis' : bigint,
  'granularity' : MetricsGranularity,
  'dateFromMillis' : bigint,
}
export interface GetRecordsRequest {
  'page' : [] | [bigint],
  'limit' : [] | [bigint],
}
export interface GetRecordsResponse {
  'total' : bigint,
  'data' : [] | [Array<GldtRecord>],
}
export interface GldNft {
  'requested_memo' : Uint8Array | number[],
  'older_record' : [] | [GldNft],
  'to_subaccount' : Uint8Array | number[],
  'minted' : [] | [GldtMinted],
  'receiving_account' : Account,
  'grams' : number,
  'gldt_minting_timestamp_seconds' : bigint,
  'gld_nft_canister_id' : Principal,
}
export interface GldtBurned { 'burn_block_height' : bigint }
export interface GldtMinted {
  'mint_block_height' : bigint,
  'last_audited_timestamp_seconds' : bigint,
  'burned' : [] | [GldtBurned],
}
export interface GldtRecord {
  'nft_id' : string,
  'gldt_minted' : bigint,
  'record_type' : string,
  'receiving_account' : Account,
  'grams' : number,
  'gldt_minting_timestamp_seconds' : bigint,
  'index' : bigint,
  'gld_nft_canister_id' : Principal,
  'block_height' : bigint,
}
export interface HourlyMetricsData {
  'updateCalls' : BigUint64Array | bigint[],
  'canisterHeapMemorySize' : BigUint64Array | bigint[],
  'canisterCycles' : BigUint64Array | bigint[],
  'canisterMemorySize' : BigUint64Array | bigint[],
  'timeMillis' : bigint,
}
export interface ICTokenSpec {
  'id' : [] | [bigint],
  'fee' : [] | [bigint],
  'decimals' : bigint,
  'canister' : Principal,
  'standard' : ICTokenSpec_standard,
  'symbol' : string,
}
export type ICTokenSpec_standard = { 'ICRC1' : null } |
  { 'EXTFungible' : null } |
  { 'DIP20' : null } |
  { 'Other' : CandyShared } |
  { 'Ledger' : null };
export interface InfoRequest {
  'nft_id' : string,
  'source_canister' : Principal,
}
export interface LogMessageData { 'timeNanos' : bigint, 'message' : string }
export type MetricsGranularity = { 'hourly' : null } |
  { 'daily' : null };
export interface MetricsRequest { 'parameters' : GetMetricsParameters }
export interface MetricsResponse { 'metrics' : [] | [CanisterMetrics] }
export interface NftCanisterConf { 'grams' : number }
export interface NftInfo { 'info' : [] | [GldNft] }
export interface NumericEntity {
  'avg' : bigint,
  'max' : bigint,
  'min' : bigint,
  'first' : bigint,
  'last' : bigint,
}
export interface Offer { 'block_height' : bigint, 'tokens_minted' : bigint }
export interface OfferRequest {
  'nft_id' : string,
  'requested_memo' : Uint8Array | number[],
  'to_subaccount' : Uint8Array | number[],
  'receiving_account' : Account,
}
export type PricingConfigShared__1 = { 'ask' : [] | [Array<AskFeature>] } |
  { 'extensible' : CandyShared } |
  { 'instant' : null } |
  { 'auction' : AuctionConfig__1 };
export interface PropertyShared {
  'value' : Box,
  'name' : string,
  'immutable' : boolean,
}
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Offer } |
  { 'Err' : string };
export interface SaleStatusShared {
  'token_id' : string,
  'sale_type' : SaleStatusShared_sale_type,
  'broker_id' : [] | [Principal],
  'original_broker_id' : [] | [Principal],
  'sale_id' : string,
}
export type SaleStatusShared_sale_type = { 'auction' : AuctionStateShared };
export interface StatusRequest {
  'memory_size' : boolean,
  'cycles' : boolean,
  'heap_memory_size' : boolean,
}
export interface StatusResponse {
  'memory_size' : [] | [bigint],
  'cycles' : [] | [bigint],
  'heap_memory_size' : [] | [bigint],
}
export interface SubAccountInfo {
  'account_id' : Uint8Array | number[],
  'principal' : Principal,
  'account_id_text' : string,
  'account' : SubAccountInfo_account,
}
export interface SubAccountInfo_account {
  'principal' : Principal,
  'sub_account' : Uint8Array | number[],
}
export interface SubscriberNotification {
  'collection' : Principal,
  'sale' : SaleStatusShared,
  'seller' : Account_1,
  'escrow_info' : SubAccountInfo,
}
export type TokenSpec = { 'ic' : ICTokenSpec } |
  { 'extensible' : CandyShared };
export interface UpdateInformationRequest {
  'metrics' : [] | [CollectMetricsRequestType],
}
export interface _SERVICE {
  'get_canistergeek_information' : ActorMethod<
    [GetInformationRequest],
    GetInformationResponse
  >,
  'get_conf' : ActorMethod<[], Conf>,
  'get_records' : ActorMethod<[GetRecordsRequest], GetRecordsResponse>,
  'nft_info' : ActorMethod<[InfoRequest], NftInfo>,
  'notify_sale_nft_origyn' : ActorMethod<[SubscriberNotification], Result>,
  'request_offer' : ActorMethod<[OfferRequest, BidRequest], Result_1>,
  'update_canistergeek_information' : ActorMethod<
    [UpdateInformationRequest],
    undefined
  >,
}
