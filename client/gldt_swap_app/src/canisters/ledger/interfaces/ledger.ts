import type { Principal } from '@dfinity/principal';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Array<number>],
}
export interface Allowance {
  'allowance' : bigint,
  'expires_at' : [] | [bigint],
}
export interface AllowanceArgs { 'account' : Account, 'spender' : Account }
export interface Approve {
  'fee' : [] | [bigint],
  'from' : Account,
  'memo' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'expected_allowance' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export interface ApproveArgs {
  'fee' : [] | [bigint],
  'memo' : [] | [Array<number>],
  'from_subaccount' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'expected_allowance' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export type ApproveError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'AllowanceChanged' : { 'current_allowance' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'Expired' : { 'ledger_time' : bigint } } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface ArchiveInfo {
  'block_range_end' : bigint,
  'canister_id' : Principal,
  'block_range_start' : bigint,
}
export interface ArchiveOptions {
  'num_blocks_to_archive' : bigint,
  'max_transactions_per_response' : [] | [bigint],
  'trigger_threshold' : bigint,
  'more_controller_ids' : [] | [Array<Principal>],
  'max_message_size_bytes' : [] | [bigint],
  'cycles_for_archive_creation' : [] | [bigint],
  'node_max_memory_size_bytes' : [] | [bigint],
  'controller_id' : Principal,
}
export interface ArchivedBlocks {
  'args' : Array<GetBlocksRequest>,
  'callback' : [Principal, string],
}
export interface ArchivedRange {
  'callback' : [Principal, string],
  'start' : bigint,
  'length' : bigint,
}
export interface ArchivedRange_1 {
  'callback' : [Principal, string],
  'start' : bigint,
  'length' : bigint,
}
export interface BlockRange { 'blocks' : Array<Value> }
export interface BlockWithId { 'id' : bigint, 'block' : ICRC3Value }
export interface Burn {
  'from' : Account,
  'memo' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export interface ChangeArchiveOptions {
  'num_blocks_to_archive' : [] | [bigint],
  'max_transactions_per_response' : [] | [bigint],
  'trigger_threshold' : [] | [bigint],
  'more_controller_ids' : [] | [Array<Principal>],
  'max_message_size_bytes' : [] | [bigint],
  'cycles_for_archive_creation' : [] | [bigint],
  'node_max_memory_size_bytes' : [] | [bigint],
  'controller_id' : [] | [Principal],
}
export type ChangeFeeCollector = { 'SetTo' : Account } |
  { 'Unset' : null };
export interface ConsentInfo {
  'metadata' : ConsentMessageMetadata,
  'consent_message' : ConsentMessage,
}
export type ConsentMessage = {
    'LineDisplayMessage' : { 'pages' : Array<LineDisplayPage> }
  } |
  { 'GenericDisplayMessage' : string };
export interface ConsentMessageMetadata {
  'utc_offset_minutes' : [] | [number],
  'language' : string,
}
export interface ConsentMessageRequest {
  'arg' : Array<number>,
  'method' : string,
  'user_preferences' : ConsentMessageSpec,
}
export interface ConsentMessageSpec {
  'metadata' : ConsentMessageMetadata,
  'device_spec' : [] | [DisplayMessageType],
}
export interface DataCertificate {
  'certificate' : [] | [Array<number>],
  'hash_tree' : Array<number>,
}
export type DisplayMessageType = { 'GenericDisplay' : null } |
  {
    'LineDisplay' : {
      'characters_per_line' : number,
      'lines_per_page' : number,
    }
  };
export interface ErrorInfo { 'description' : string }
export interface FeatureFlags { 'icrc2' : boolean }
export interface GetArchivesArgs { 'from' : [] | [Principal] }
export interface GetBlocksRequest { 'start' : bigint, 'length' : bigint }
export interface GetBlocksResponse {
  'certificate' : [] | [Array<number>],
  'first_index' : bigint,
  'blocks' : Array<Value>,
  'chain_length' : bigint,
  'archived_blocks' : Array<ArchivedRange>,
}
export interface GetBlocksResult {
  'log_length' : bigint,
  'blocks' : Array<BlockWithId>,
  'archived_blocks' : Array<ArchivedBlocks>,
}
export interface GetTransactionsResponse {
  'first_index' : bigint,
  'log_length' : bigint,
  'transactions' : Array<Transaction>,
  'archived_transactions' : Array<ArchivedRange_1>,
}
export interface ICRC3ArchiveInfo {
  'end' : bigint,
  'canister_id' : Principal,
  'start' : bigint,
}
export interface ICRC3DataCertificate {
  'certificate' : Array<number>,
  'hash_tree' : Array<number>,
}
export type ICRC3Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, ICRC3Value]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Array<number> } |
  { 'Text' : string } |
  { 'Array' : Array<ICRC3Value> };
export type Icrc21Error = {
    'GenericError' : { 'description' : string, 'error_code' : bigint }
  } |
  { 'InsufficientPayment' : ErrorInfo } |
  { 'UnsupportedCanisterCall' : ErrorInfo } |
  { 'ConsentMessageUnavailable' : ErrorInfo };
export interface InitArgs {
  'decimals' : [] | [number],
  'token_symbol' : string,
  'transfer_fee' : bigint,
  'metadata' : Array<[string, MetadataValue]>,
  'minting_account' : Account,
  'initial_balances' : Array<[Account, bigint]>,
  'maximum_number_of_accounts' : [] | [bigint],
  'accounts_overflow_trim_quantity' : [] | [bigint],
  'fee_collector_account' : [] | [Account],
  'archive_options' : ArchiveOptions,
  'max_memo_length' : [] | [number],
  'token_name' : string,
  'feature_flags' : [] | [FeatureFlags],
}
export type LedgerArgument = { 'Upgrade' : [] | [UpgradeArgs] } |
  { 'Init' : InitArgs };
export interface LineDisplayPage { 'lines' : Array<string> }
export type MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Array<number> } |
  { 'Text' : string };
export interface Mint {
  'to' : Account,
  'memo' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type Result = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Result_1 = { 'Ok' : ConsentInfo } |
  { 'Err' : Icrc21Error };
export type Result_2 = { 'Ok' : bigint } |
  { 'Err' : ApproveError };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : TransferFromError };
export interface StandardRecord { 'url' : string, 'name' : string }
export interface SupportedBlockType { 'url' : string, 'block_type' : string }
export interface Transaction {
  'burn' : [] | [Burn],
  'kind' : string,
  'mint' : [] | [Mint],
  'approve' : [] | [Approve],
  'timestamp' : bigint,
  'transfer' : [] | [Transfer],
}
export interface TransactionRange { 'transactions' : Array<Transaction> }
export interface Transfer {
  'to' : Account,
  'fee' : [] | [bigint],
  'from' : Account,
  'memo' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export interface TransferArg {
  'to' : Account,
  'fee' : [] | [bigint],
  'memo' : [] | [Array<number>],
  'from_subaccount' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface TransferFromArgs {
  'to' : Account,
  'fee' : [] | [bigint],
  'spender_subaccount' : [] | [Array<number>],
  'from' : Account,
  'memo' : [] | [Array<number>],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type TransferFromError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'InsufficientAllowance' : { 'allowance' : bigint } } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface UpgradeArgs {
  'change_archive_options' : [] | [ChangeArchiveOptions],
  'token_symbol' : [] | [string],
  'transfer_fee' : [] | [bigint],
  'metadata' : [] | [Array<[string, MetadataValue]>],
  'maximum_number_of_accounts' : [] | [bigint],
  'accounts_overflow_trim_quantity' : [] | [bigint],
  'change_fee_collector' : [] | [ChangeFeeCollector],
  'max_memo_length' : [] | [number],
  'token_name' : [] | [string],
  'feature_flags' : [] | [FeatureFlags],
}
export type Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value]> } |
  { 'Nat' : bigint } |
  { 'Nat64' : bigint } |
  { 'Blob' : Array<number> } |
  { 'Text' : string } |
  { 'Array' : Vec };
export type Vec = Array<
  { 'Int' : bigint } |
    { 'Map' : Array<[string, Value]> } |
    { 'Nat' : bigint } |
    { 'Nat64' : bigint } |
    { 'Blob' : Array<number> } |
    { 'Text' : string } |
    { 'Array' : Vec }
>;
export default interface _SERVICE {
  'archives' : () => Promise<Array<ArchiveInfo>>,
  'get_blocks' : (arg_0: GetBlocksRequest) => Promise<GetBlocksResponse>,
  'get_data_certificate' : () => Promise<DataCertificate>,
  'get_transactions' : (arg_0: GetBlocksRequest) => Promise<
      GetTransactionsResponse
    >,
  'icrc10_supported_standards' : () => Promise<Array<StandardRecord>>,
  'icrc1_balance_of' : (arg_0: Account) => Promise<bigint>,
  'icrc1_decimals' : () => Promise<number>,
  'icrc1_fee' : () => Promise<bigint>,
  'icrc1_metadata' : () => Promise<Array<[string, MetadataValue]>>,
  'icrc1_minting_account' : () => Promise<[] | [Account]>,
  'icrc1_name' : () => Promise<string>,
  'icrc1_supported_standards' : () => Promise<Array<StandardRecord>>,
  'icrc1_symbol' : () => Promise<string>,
  'icrc1_total_supply' : () => Promise<bigint>,
  'icrc1_transfer' : (arg_0: TransferArg) => Promise<Result>,
  'icrc21_canister_call_consent_message' : (
      arg_0: ConsentMessageRequest,
    ) => Promise<Result_1>,
  'icrc2_allowance' : (arg_0: AllowanceArgs) => Promise<Allowance>,
  'icrc2_approve' : (arg_0: ApproveArgs) => Promise<Result_2>,
  'icrc2_transfer_from' : (arg_0: TransferFromArgs) => Promise<Result_3>,
  'icrc3_get_archives' : (arg_0: GetArchivesArgs) => Promise<
      Array<ICRC3ArchiveInfo>
    >,
  'icrc3_get_blocks' : (arg_0: Array<GetBlocksRequest>) => Promise<
      GetBlocksResult
    >,
  'icrc3_get_tip_certificate' : () => Promise<[] | [ICRC3DataCertificate]>,
  'icrc3_supported_block_types' : () => Promise<Array<SupportedBlockType>>,
}