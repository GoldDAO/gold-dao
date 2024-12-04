import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ActivitySnapshot {
  'principals_active_during_snapshot' : bigint,
  'accounts_active_during_snapshot' : bigint,
  'total_unique_accounts' : bigint,
  'end_time' : bigint,
  'start_time' : bigint,
  'total_unique_principals' : bigint,
}
export interface GetAccountHistoryArgs { 'days' : bigint, 'account' : string }
export interface GetHoldersArgs { 'offset' : bigint, 'limit' : bigint }
export interface HistoryData { 'balance' : bigint }
export interface HolderBalanceResponse { 'data' : Overview, 'holder' : string }
export type IndexerType = { 'DfinityIcrc2' : null } |
  { 'DfinityIcrc3' : null } |
  { 'DfinityIcp' : null };
export interface InitArgs { 'admin' : string, 'test_mode' : boolean }
export interface InitLedgerArgs {
  'index_type' : IndexerType,
  'target' : TargetArgs,
}
export interface LogEntry { 'text' : string, 'timestamp' : string }
export interface MemoryData { 'memory' : bigint, 'heap_memory' : bigint }
export interface Metrics {
  'total_errors' : bigint,
  'total_api_requests' : bigint,
}
export interface Overview {
  'balance' : bigint,
  'sent' : [number, bigint],
  'last_active' : bigint,
  'first_active' : bigint,
  'received' : [number, bigint],
  'max_balance' : bigint,
}
export interface ProcessedTX {
  'hash' : string,
  'to_account' : string,
  'tx_value' : bigint,
  'from_account' : string,
  'block' : bigint,
  'tx_fee' : [] | [bigint],
  'tx_time' : bigint,
  'tx_type' : string,
  'spender' : [] | [string],
}
export interface TargetArgs {
  'daily_size' : number,
  'target_ledger' : string,
  'hourly_size' : number,
}
export interface TimeChunkStats {
  'mint_count' : bigint,
  'transfer_count' : bigint,
  'end_time' : bigint,
  'start_time' : bigint,
  'burn_count' : bigint,
  'approve_count' : bigint,
  'total_count' : bigint,
}
export interface TimeStats {
  'top_transfers' : Array<ProcessedTX>,
  'total_unique_accounts' : bigint,
  'top_burns' : Array<ProcessedTX>,
  'mint_stats' : TotCntAvg,
  'total_transaction_average' : number,
  'most_active_principals' : Array<[string, bigint]>,
  'transfer_stats' : TotCntAvg,
  'top_mints' : Array<ProcessedTX>,
  'total_transaction_value' : bigint,
  'most_active_accounts' : Array<[string, bigint]>,
  'count_over_time' : Array<TimeChunkStats>,
  'total_transaction_count' : bigint,
  'total_unique_principals' : bigint,
  'burn_stats' : TotCntAvg,
  'approve_stats' : TotCntAvg,
}
export interface TotCntAvg {
  'count' : bigint,
  'average' : number,
  'total_value' : bigint,
}
export interface TotalHolderResponse {
  'total_accounts' : bigint,
  'total_principals' : bigint,
}
export interface WorkingStats {
  'metrics' : Metrics,
  'next_block' : bigint,
  'last_update_time' : bigint,
  'ledger_tip_of_chain' : bigint,
  'timer_active' : boolean,
  'is_upto_date' : boolean,
  'directory_count' : bigint,
  'is_busy' : boolean,
}
export interface _SERVICE {
  'add_admin' : ActorMethod<[string], string>,
  'add_authorised' : ActorMethod<[string], string>,
  'deposit_cycles' : ActorMethod<[], undefined>,
  'get_account_history' : ActorMethod<
    [GetAccountHistoryArgs],
    Array<[bigint, HistoryData]>
  >,
  'get_account_holders' : ActorMethod<
    [GetHoldersArgs],
    Array<HolderBalanceResponse>
  >,
  'get_account_overview' : ActorMethod<[string], [] | [Overview]>,
  'get_activity_stats' : ActorMethod<[bigint], Array<ActivitySnapshot>>,
  'get_all_admins' : ActorMethod<[], Array<string>>,
  'get_all_authorised' : ActorMethod<[], Array<string>>,
  'get_canister_version' : ActorMethod<[], string>,
  'get_cycles_balance' : ActorMethod<[], bigint>,
  'get_daily_stats' : ActorMethod<[], TimeStats>,
  'get_hourly_stats' : ActorMethod<[], TimeStats>,
  'get_logs' : ActorMethod<[], [] | [Array<LogEntry>]>,
  'get_memory_stats' : ActorMethod<[], MemoryData>,
  'get_principal_history' : ActorMethod<
    [GetAccountHistoryArgs],
    Array<[bigint, HistoryData]>
  >,
  'get_principal_holders' : ActorMethod<
    [GetHoldersArgs],
    Array<HolderBalanceResponse>
  >,
  'get_principal_overview' : ActorMethod<[string], [] | [Overview]>,
  'get_top_account_holders' : ActorMethod<
    [bigint],
    Array<HolderBalanceResponse>
  >,
  'get_top_principal_holders' : ActorMethod<
    [bigint],
    Array<HolderBalanceResponse>
  >,
  'get_total_holders' : ActorMethod<[], TotalHolderResponse>,
  'get_working_stats' : ActorMethod<[], WorkingStats>,
  'init_target_ledger' : ActorMethod<[InitLedgerArgs], string>,
  'remove_admin' : ActorMethod<[string], string>,
  'remove_authorised' : ActorMethod<[string], string>,
  'self_call' : ActorMethod<[], undefined>,
  'self_call2' : ActorMethod<[], undefined>,
  'start_processing_timer' : ActorMethod<[bigint], string>,
  'stop_all_timers' : ActorMethod<[], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];