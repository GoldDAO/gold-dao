import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Subaccount],
}
export type Account__1 = { 'account_id' : string } |
  { 'principal' : Principal } |
  { 'extensible' : CandyShared } |
  {
    'account' : {
      'owner' : Principal,
      'sub_account' : [] | [Uint8Array | number[]],
    }
  };
export type Account__2 = { 'account_id' : string } |
  { 'principal' : Principal } |
  { 'extensible' : CandyShared } |
  {
    'account' : {
      'owner' : Principal,
      'sub_account' : [] | [Uint8Array | number[]],
    }
  };
export type Account__3 = { 'account_id' : string } |
  { 'principal' : Principal } |
  { 'extensible' : CandyShared } |
  {
    'account' : {
      'owner' : Principal,
      'sub_account' : [] | [Uint8Array | number[]],
    }
  };
export interface AllocationRecordStable {
  'allocated_space' : bigint,
  'token_id' : string,
  'available_space' : bigint,
  'canister' : Principal,
  'chunks' : Array<bigint>,
  'library_id' : string,
}
export interface ApprovalArgs {
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export type ApprovalError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'CreatexInFuture' : { 'ledger_time' : bigint } } |
  { 'NonExistingTokenId' : null } |
  { 'Unauthorized' : null } |
  { 'TooOld' : null };
export type ApprovalResult = Array<
  {
    'token_id' : bigint,
    'approval_result' : { 'Ok' : bigint } |
      { 'Err' : ApprovalError },
  }
>;
export interface ArchivedTransactionResponse {
  'args' : Array<TransactionRange__1>,
  'callback' : GetTransactionsFn,
}
export type AskConfigShared = [] | [AskFeatureArray];
export type AskFeature = { 'kyc' : Principal } |
  { 'start_price' : bigint } |
  { 'token' : TokenSpec } |
  { 'fee_schema' : string } |
  { 'notify' : Array<Principal> } |
  { 'wait_for_quiet' : WaitForQuietType } |
  { 'reserve' : bigint } |
  { 'start_date' : bigint } |
  { 'min_increase' : MinIncreaseType } |
  { 'allow_list' : Array<Principal> } |
  { 'buy_now' : bigint } |
  { 'fee_accounts' : FeeAccountsParams } |
  { 'nifty_settlement' : NiftySettlementType } |
  { 'atomic' : null } |
  { 'dutch' : DutchParams } |
  { 'ending' : EndingType };
export type AskFeatureArray = Array<AskFeature>;
export type AskSubscribeRequest = {
    'subscribe' : {
      'stake' : [Principal, bigint],
      'filter' : [] | [
        {
          'tokens' : [] | [Array<TokenSpecFilter>],
          'token_ids' : [] | [Array<TokenIDFilter>],
        }
      ],
    }
  } |
  { 'unsubscribe' : [Principal, bigint] };
export type AskSubscribeResponse = boolean;
export interface AuctionConfig {
  'start_price' : bigint,
  'token' : TokenSpec,
  'reserve' : [] | [bigint],
  'start_date' : bigint,
  'min_increase' : MinIncreaseType,
  'allow_list' : [] | [Array<Principal>],
  'buy_now' : [] | [bigint],
  'ending' : { 'date' : bigint } |
    {
      'wait_for_quiet' : {
        'max' : bigint,
        'date' : bigint,
        'fade' : number,
        'extension' : bigint,
      }
    },
}
export interface AuctionStateShared {
  'status' : { 'closed' : null } |
    { 'open' : null } |
    { 'not_started' : null },
  'participants' : Array<[Principal, bigint]>,
  'token' : TokenSpec__1,
  'current_bid_amount' : bigint,
  'winner' : [] | [Account__1],
  'end_date' : bigint,
  'current_config' : BidConfigShared,
  'start_date' : bigint,
  'wait_for_quiet_count' : [] | [bigint],
  'current_escrow' : [] | [EscrowReceipt],
  'allow_list' : [] | [Array<[Principal, boolean]>],
  'min_next_bid' : bigint,
  'config' : PricingConfigShared__1,
}
export interface BalanceResponse {
  'nfts' : Array<string>,
  'offers' : Array<EscrowRecord__1>,
  'sales' : Array<EscrowRecord__1>,
  'stake' : Array<StakeRecord>,
  'multi_canister' : [] | [Array<Principal>],
  'escrow' : Array<EscrowRecord__1>,
}
export type BalanceResult = { 'ok' : BalanceResponse } |
  { 'err' : OrigynError };
export type BearerResult = { 'ok' : Account__1 } |
  { 'err' : OrigynError };
export type BidConfigShared = [] | [Array<BidFeature>];
export type BidFeature = { 'fee_schema' : string } |
  { 'broker' : Account__2 } |
  { 'fee_accounts' : FeeAccountsParams };
export interface BidRequest {
  'config' : BidConfigShared,
  'escrow_record' : EscrowRecord,
}
export interface BidResponse {
  'token_id' : string,
  'txn_type' : {
      'escrow_deposit' : {
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'fee_deposit' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_network_updated' : {
        'network' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'escrow_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_managers_updated' : {
        'managers' : Array<Principal>,
        'extensible' : CandyShared,
      }
    } |
    {
      'auction_bid' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : string,
      }
    } |
    { 'burn' : { 'from' : [] | [Account__2], 'extensible' : CandyShared } } |
    {
      'data' : {
        'hash' : [] | [Uint8Array | number[]],
        'extensible' : CandyShared,
        'data_dapp' : [] | [string],
        'data_path' : [] | [string],
      }
    } |
    {
      'sale_ended' : {
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : [] | [string],
      }
    } |
    {
      'mint' : {
        'to' : Account__2,
        'from' : Account__2,
        'sale' : [] | [{ 'token' : TokenSpec, 'amount' : bigint }],
        'extensible' : CandyShared,
      }
    } |
    {
      'royalty_paid' : {
        'tag' : string,
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'receiver' : Account__2,
        'sale_id' : [] | [string],
      }
    } |
    { 'extensible' : CandyShared } |
    {
      'fee_deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'owner_transfer' : {
        'to' : Account__2,
        'from' : Account__2,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_opened' : {
        'pricing' : PricingConfigShared,
        'extensible' : CandyShared,
        'sale_id' : string,
      }
    } |
    {
      'canister_owner_updated' : {
        'owner' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    },
  'timestamp' : bigint,
  'index' : bigint,
}
export interface BlockType { 'url' : string, 'block_type' : string }
export type Caller = [] | [Principal];
export type CandyShared = { 'Int' : bigint } |
  { 'Map' : Array<[CandyShared, CandyShared]> } |
  { 'Nat' : bigint } |
  { 'Set' : Array<CandyShared> } |
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
  { 'Option' : [] | [CandyShared] } |
  { 'Floats' : Array<number> } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array<CandyShared> } |
  { 'Class' : Array<PropertyShared> };
export type CanisterCyclesAggregatedData = BigUint64Array | bigint[];
export type CanisterHeapMemoryAggregatedData = BigUint64Array | bigint[];
export type CanisterLogFeature = { 'filterMessageByContains' : null } |
  { 'filterMessageByRegex' : null };
export interface CanisterLogMessages {
  'data' : Array<LogMessagesData>,
  'lastAnalyzedMessageTimeNanos' : [] | [Nanos],
}
export interface CanisterLogMessagesInfo {
  'features' : Array<[] | [CanisterLogFeature]>,
  'lastTimeNanos' : [] | [Nanos],
  'count' : number,
  'firstTimeNanos' : [] | [Nanos],
}
export type CanisterLogRequest = { 'getMessagesInfo' : null } |
  { 'getMessages' : GetLogMessagesParameters } |
  { 'getLatestMessages' : GetLatestLogMessagesParameters };
export type CanisterLogResponse = { 'messagesInfo' : CanisterLogMessagesInfo } |
  { 'messages' : CanisterLogMessages };
export type CanisterMemoryAggregatedData = BigUint64Array | bigint[];
export interface CanisterMetrics { 'data' : CanisterMetricsData }
export type CanisterMetricsData = { 'hourly' : Array<HourlyMetricsData> } |
  { 'daily' : Array<DailyMetricsData> };
export type ChunkContent = {
    'remote' : { 'args' : ChunkRequest, 'canister' : Principal }
  } |
  {
    'chunk' : {
      'total_chunks' : bigint,
      'content' : Uint8Array | number[],
      'storage_allocation' : AllocationRecordStable,
      'current_chunk' : [] | [bigint],
    }
  };
export interface ChunkRequest {
  'token_id' : string,
  'chunk' : [] | [bigint],
  'library_id' : string,
}
export type ChunkResult = { 'ok' : ChunkContent } |
  { 'err' : OrigynError };
export interface CollectionInfo {
  'multi_canister_count' : [] | [bigint],
  'managers' : [] | [Array<Principal>],
  'owner' : [] | [Principal],
  'metadata' : [] | [CandyShared],
  'logo' : [] | [string],
  'name' : [] | [string],
  'network' : [] | [Principal],
  'created_at' : [] | [bigint],
  'fields' : [] | [Array<[string, [] | [bigint], [] | [bigint]]>],
  'upgraded_at' : [] | [bigint],
  'token_ids_count' : [] | [bigint],
  'available_space' : [] | [bigint],
  'multi_canister' : [] | [Array<Principal>],
  'token_ids' : [] | [Array<string>],
  'transaction_count' : [] | [bigint],
  'unique_holders' : [] | [bigint],
  'total_supply' : [] | [bigint],
  'symbol' : [] | [string],
  'allocated_storage' : [] | [bigint],
}
export type CollectionMetadata = Array<[string, Value]>;
export type CollectionResult = { 'ok' : CollectionInfo } |
  { 'err' : OrigynError };
export type DIP721BoolResult = { 'Ok' : boolean } |
  { 'Err' : NftError };
export interface DIP721Metadata {
  'logo' : [] | [string],
  'name' : [] | [string],
  'created_at' : bigint,
  'upgraded_at' : bigint,
  'custodians' : Array<Principal>,
  'symbol' : [] | [string],
}
export type DIP721NatResult = { 'Ok' : bigint } |
  { 'Err' : NftError };
export interface DIP721Stats {
  'cycles' : bigint,
  'total_transactions' : bigint,
  'total_unique_holders' : bigint,
  'total_supply' : bigint,
}
export type DIP721SupportedInterface = { 'Burn' : null } |
  { 'Mint' : null } |
  { 'Approval' : null } |
  { 'TransactionHistory' : null };
export type DIP721TokenMetadata = { 'Ok' : TokenMetadata } |
  { 'Err' : NftError };
export type DIP721TokensListMetadata = { 'Ok' : Array<bigint> } |
  { 'Err' : NftError };
export type DIP721TokensMetadata = { 'Ok' : Array<TokenMetadata> } |
  { 'Err' : NftError };
export interface DailyMetricsData {
  'updateCalls' : bigint,
  'canisterHeapMemorySize' : NumericEntity,
  'canisterCycles' : NumericEntity,
  'canisterMemorySize' : NumericEntity,
  'timeMillis' : bigint,
}
export type Data = { 'Int' : bigint } |
  { 'Map' : Array<[CandyShared, CandyShared]> } |
  { 'Nat' : bigint } |
  { 'Set' : Array<CandyShared> } |
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
  { 'Option' : [] | [CandyShared] } |
  { 'Floats' : Array<number> } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array<CandyShared> } |
  { 'Class' : Array<PropertyShared> };
export interface DataCertificate {
  'certificate' : Uint8Array | number[],
  'hash_tree' : Uint8Array | number[],
}
export interface DepositDetail {
  'token' : TokenSpec__1,
  'trx_id' : [] | [TransactionID__1],
  'seller' : Account__1,
  'buyer' : Account__1,
  'amount' : bigint,
  'sale_id' : [] | [string],
}
export interface DepositWithdrawDescription {
  'token' : TokenSpec__1,
  'withdraw_to' : Account__1,
  'buyer' : Account__1,
  'amount' : bigint,
}
export interface DistributeSaleRequest { 'seller' : [] | [Account__1] }
export type DistributeSaleResponse = Array<Result>;
export interface DutchParams {
  'time_unit' : { 'day' : bigint } |
    { 'hour' : bigint } |
    { 'minute' : bigint },
  'decay_type' : { 'flat' : bigint } |
    { 'percent' : number },
}
export type EXTAccountIdentifier = string;
export type EXTBalance = bigint;
export interface EXTBalanceRequest {
  'token' : EXTTokenIdentifier,
  'user' : EXTUser,
}
export type EXTBalanceResult = { 'ok' : EXTBalance } |
  { 'err' : EXTCommonError };
export type EXTBearerResult = { 'ok' : EXTAccountIdentifier } |
  { 'err' : EXTCommonError };
export type EXTCommonError = { 'InvalidToken' : EXTTokenIdentifier } |
  { 'Other' : string };
export type EXTMemo = Uint8Array | number[];
export type EXTMetadata = {
    'fungible' : {
      'decimals' : number,
      'metadata' : [] | [Uint8Array | number[]],
      'name' : string,
      'symbol' : string,
    }
  } |
  { 'nonfungible' : { 'metadata' : [] | [Uint8Array | number[]] } };
export type EXTMetadataResult = { 'ok' : EXTMetadata } |
  { 'err' : EXTCommonError };
export type EXTSubAccount = Uint8Array | number[];
export type EXTTokenIdentifier = string;
export type EXTTokensResponse = [
  number,
  [] | [{ 'locked' : [] | [bigint], 'seller' : Principal, 'price' : bigint }],
  [] | [Uint8Array | number[]],
];
export type EXTTokensResult = { 'ok' : Array<EXTTokensResponse> } |
  { 'err' : EXTCommonError };
export interface EXTTransferRequest {
  'to' : EXTUser,
  'token' : EXTTokenIdentifier,
  'notify' : boolean,
  'from' : EXTUser,
  'memo' : EXTMemo,
  'subaccount' : [] | [EXTSubAccount],
  'amount' : EXTBalance,
}
export type EXTTransferResponse = { 'ok' : EXTBalance } |
  {
    'err' : { 'CannotNotify' : EXTAccountIdentifier } |
      { 'InsufficientBalance' : null } |
      { 'InvalidToken' : EXTTokenIdentifier } |
      { 'Rejected' : null } |
      { 'Unauthorized' : EXTAccountIdentifier } |
      { 'Other' : string }
  };
export type EXTUser = { 'principal' : Principal } |
  { 'address' : string };
export interface EndSaleResponse {
  'token_id' : string,
  'txn_type' : {
      'escrow_deposit' : {
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'fee_deposit' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_network_updated' : {
        'network' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'escrow_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_managers_updated' : {
        'managers' : Array<Principal>,
        'extensible' : CandyShared,
      }
    } |
    {
      'auction_bid' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : string,
      }
    } |
    { 'burn' : { 'from' : [] | [Account__2], 'extensible' : CandyShared } } |
    {
      'data' : {
        'hash' : [] | [Uint8Array | number[]],
        'extensible' : CandyShared,
        'data_dapp' : [] | [string],
        'data_path' : [] | [string],
      }
    } |
    {
      'sale_ended' : {
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : [] | [string],
      }
    } |
    {
      'mint' : {
        'to' : Account__2,
        'from' : Account__2,
        'sale' : [] | [{ 'token' : TokenSpec, 'amount' : bigint }],
        'extensible' : CandyShared,
      }
    } |
    {
      'royalty_paid' : {
        'tag' : string,
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'receiver' : Account__2,
        'sale_id' : [] | [string],
      }
    } |
    { 'extensible' : CandyShared } |
    {
      'fee_deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'owner_transfer' : {
        'to' : Account__2,
        'from' : Account__2,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_opened' : {
        'pricing' : PricingConfigShared,
        'extensible' : CandyShared,
        'sale_id' : string,
      }
    } |
    {
      'canister_owner_updated' : {
        'owner' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    },
  'timestamp' : bigint,
  'index' : bigint,
}
export type EndingType = { 'date' : bigint } |
  { 'timeout' : bigint };
export type Errors = { 'nyi' : null } |
  { 'storage_configuration_error' : null } |
  { 'escrow_withdraw_payment_failed' : null } |
  { 'token_not_found' : null } |
  { 'owner_not_found' : null } |
  { 'content_not_found' : null } |
  { 'auction_ended' : null } |
  { 'out_of_range' : null } |
  { 'sale_id_does_not_match' : null } |
  { 'sale_not_found' : null } |
  { 'kyc_fail' : null } |
  { 'item_not_owned' : null } |
  { 'property_not_found' : null } |
  { 'validate_trx_wrong_host' : null } |
  { 'withdraw_too_large' : null } |
  { 'content_not_deserializable' : null } |
  { 'bid_too_low' : null } |
  { 'validate_deposit_wrong_amount' : null } |
  { 'existing_sale_found' : null } |
  { 'noop' : null } |
  { 'asset_mismatch' : null } |
  { 'escrow_cannot_be_removed' : null } |
  { 'deposit_burned' : null } |
  { 'cannot_restage_minted_token' : null } |
  { 'cannot_find_status_in_metadata' : null } |
  { 'receipt_data_mismatch' : null } |
  { 'validate_deposit_failed' : null } |
  { 'unreachable' : null } |
  { 'unauthorized_access' : null } |
  { 'item_already_minted' : null } |
  { 'no_escrow_found' : null } |
  { 'escrow_owner_not_the_owner' : null } |
  { 'improper_interface' : null } |
  { 'app_id_not_found' : null } |
  { 'token_non_transferable' : null } |
  { 'kyc_error' : null } |
  { 'sale_not_over' : null } |
  { 'escrow_not_large_enough' : null } |
  { 'update_class_error' : null } |
  { 'malformed_metadata' : null } |
  { 'token_id_mismatch' : null } |
  { 'id_not_found_in_metadata' : null } |
  { 'auction_not_started' : null } |
  { 'low_fee_balance' : null } |
  { 'library_not_found' : null } |
  { 'attempt_to_stage_system_data' : null } |
  { 'no_fee_accounts_provided' : null } |
  { 'validate_deposit_wrong_buyer' : null } |
  { 'not_enough_storage' : null } |
  { 'sales_withdraw_payment_failed' : null };
export interface EscrowReceipt {
  'token' : TokenSpec,
  'token_id' : string,
  'seller' : Account__2,
  'buyer' : Account__2,
  'amount' : bigint,
}
export interface EscrowReceipt__1 {
  'token' : TokenSpec,
  'token_id' : string,
  'seller' : Account__2,
  'buyer' : Account__2,
  'amount' : bigint,
}
export interface EscrowRecord {
  'token' : TokenSpec__2,
  'token_id' : string,
  'seller' : Account__3,
  'lock_to_date' : [] | [bigint],
  'buyer' : Account__3,
  'amount' : bigint,
  'sale_id' : [] | [string],
  'account_hash' : [] | [Uint8Array | number[]],
}
export interface EscrowRecord__1 {
  'token' : TokenSpec__1,
  'token_id' : string,
  'seller' : Account__1,
  'lock_to_date' : [] | [bigint],
  'buyer' : Account__1,
  'amount' : bigint,
  'sale_id' : [] | [string],
  'account_hash' : [] | [Uint8Array | number[]],
}
export interface EscrowRequest {
  'token_id' : string,
  'deposit' : DepositDetail,
  'lock_to_date' : [] | [bigint],
}
export interface EscrowResponse {
  'balance' : bigint,
  'receipt' : EscrowReceipt,
  'transaction' : TransactionRecord,
}
export type FeeAccountsParams = Array<FeeName>;
export interface FeeDepositRequest {
  'token' : TokenSpec__1,
  'account' : Account__1,
}
export interface FeeDepositResponse {
  'balance' : bigint,
  'transaction' : TransactionRecord,
}
export interface FeeDepositWithdrawDescription {
  'status' : { 'locked' : { 'token_id' : string, 'sale_id' : string } } |
    { 'unlocked' : null },
  'token' : TokenSpec__1,
  'withdraw_to' : Account__1,
  'account' : Account__1,
  'amount' : bigint,
}
export type FeeName = string;
export type GenericValue = { 'Nat64Content' : bigint } |
  { 'Nat32Content' : number } |
  { 'BoolContent' : boolean } |
  { 'Nat8Content' : number } |
  { 'Int64Content' : bigint } |
  { 'IntContent' : bigint } |
  { 'NatContent' : bigint } |
  { 'Nat16Content' : number } |
  { 'Int32Content' : number } |
  { 'Int8Content' : number } |
  { 'FloatContent' : number } |
  { 'Int16Content' : number } |
  { 'BlobContent' : Uint8Array | number[] } |
  { 'NestedContent' : Vec } |
  { 'Principal' : Principal } |
  { 'TextContent' : string };
export interface GetArchivesArgs { 'from' : [] | [Principal] }
export type GetArchivesResult = Array<GetArchivesResultItem>;
export interface GetArchivesResultItem {
  'end' : bigint,
  'canister_id' : Principal,
  'start' : bigint,
}
export interface GetLatestLogMessagesParameters {
  'upToTimeNanos' : [] | [Nanos],
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
  'fromTimeNanos' : [] | [Nanos],
}
export interface GetMetricsParameters {
  'dateToMillis' : bigint,
  'granularity' : MetricsGranularity,
  'dateFromMillis' : bigint,
}
export type GetTransactionsFn = ActorMethod<
  [Array<TransactionRange__1>],
  GetTransactionsResult__1
>;
export interface GetTransactionsResult {
  'log_length' : bigint,
  'blocks' : Array<{ 'id' : bigint, 'block' : Value__1 }>,
  'archived_blocks' : Array<ArchivedTransactionResponse>,
}
export interface GetTransactionsResult__1 {
  'log_length' : bigint,
  'blocks' : Array<{ 'id' : bigint, 'block' : Value__1 }>,
  'archived_blocks' : Array<ArchivedTransactionResponse>,
}
export type GovernanceRequest = {
    'update_system_var' : {
      'key' : string,
      'val' : CandyShared,
      'token_id' : string,
    }
  } |
  { 'clear_shared_wallets' : string };
export type GovernanceResponse = { 'update_system_var' : boolean } |
  { 'clear_shared_wallets' : boolean };
export type GovernanceResult = { 'ok' : GovernanceResponse } |
  { 'err' : OrigynError };
export interface HTTPResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export type HeaderField = [string, string];
export type HistoryResult = { 'ok' : Array<TransactionRecord> } |
  { 'err' : OrigynError };
export interface HourlyMetricsData {
  'updateCalls' : UpdateCallsAggregatedData,
  'canisterHeapMemorySize' : CanisterHeapMemoryAggregatedData,
  'canisterCycles' : CanisterCyclesAggregatedData,
  'canisterMemorySize' : CanisterMemoryAggregatedData,
  'timeMillis' : bigint,
}
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
}
export interface ICTokenSpec {
  'id' : [] | [bigint],
  'fee' : [] | [bigint],
  'decimals' : bigint,
  'canister' : Principal,
  'standard' : { 'ICRC1' : null } |
    { 'EXTFungible' : null } |
    { 'DIP20' : null } |
    { 'Other' : CandyShared } |
    { 'Ledger' : null },
  'symbol' : string,
}
export interface ICTokenSpec__1 {
  'id' : [] | [bigint],
  'fee' : [] | [bigint],
  'decimals' : bigint,
  'canister' : Principal,
  'standard' : { 'ICRC1' : null } |
    { 'EXTFungible' : null } |
    { 'DIP20' : null } |
    { 'Other' : CandyShared } |
    { 'Ledger' : null },
  'symbol' : string,
}
export type IndexType = { 'Stable' : null } |
  { 'StableTyped' : null } |
  { 'Managed' : null };
export type InstantConfigShared = [] | [Array<InstantFeature>];
export type InstantFeature = { 'fee_schema' : string } |
  { 'fee_accounts' : FeeAccountsParams } |
  { 'transfer' : null };
export interface LogMessagesData {
  'data' : Data,
  'timeNanos' : Nanos,
  'message' : string,
  'caller' : Caller,
}
export type ManageCollectionCommand = { 'UpdateOwner' : Principal } |
  { 'UpdateManagers' : Array<Principal> } |
  { 'UpdateMetadata' : [string, [] | [CandyShared], boolean] } |
  { 'UpdateAnnounceCanister' : [] | [Principal] } |
  { 'UpdateNetwork' : [] | [Principal] } |
  { 'UpdateSymbol' : [] | [string] } |
  { 'UpdateLogo' : [] | [string] } |
  { 'UpdateName' : [] | [string] };
export type ManageSaleRequest = { 'bid' : BidRequest } |
  { 'escrow_deposit' : EscrowRequest } |
  { 'fee_deposit' : FeeDepositRequest } |
  { 'recognize_escrow' : EscrowRequest } |
  { 'withdraw' : WithdrawRequest } |
  { 'ask_subscribe' : AskSubscribeRequest } |
  { 'end_sale' : string } |
  { 'refresh_offers' : [] | [Account__1] } |
  { 'distribute_sale' : DistributeSaleRequest } |
  { 'open_sale' : string };
export type ManageSaleResponse = { 'bid' : BidResponse } |
  { 'escrow_deposit' : EscrowResponse } |
  { 'fee_deposit' : FeeDepositResponse } |
  { 'recognize_escrow' : RecognizeEscrowResponse } |
  { 'withdraw' : WithdrawResponse } |
  { 'ask_subscribe' : AskSubscribeResponse } |
  { 'end_sale' : EndSaleResponse } |
  { 'refresh_offers' : Array<EscrowRecord__1> } |
  { 'distribute_sale' : DistributeSaleResponse } |
  { 'open_sale' : boolean };
export type ManageSaleResult = { 'ok' : ManageSaleResponse } |
  { 'err' : OrigynError };
export type ManageStorageRequest = {
    'add_storage_canisters' : Array<
      [Principal, bigint, [bigint, bigint, bigint]]
    >
  } |
  {
    'configure_storage' : { 'stableBtree' : [] | [bigint] } |
      { 'heap' : [] | [bigint] }
  };
export type ManageStorageResponse = {
    'add_storage_canisters' : [bigint, bigint]
  } |
  { 'configure_storage' : [bigint, bigint] };
export type ManageStorageResult = { 'ok' : ManageStorageResponse } |
  { 'err' : OrigynError };
export interface MarketTransferRequest {
  'token_id' : string,
  'sales_config' : SalesConfig,
}
export interface MarketTransferRequestReponse {
  'token_id' : string,
  'txn_type' : {
      'escrow_deposit' : {
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'fee_deposit' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_network_updated' : {
        'network' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'escrow_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_managers_updated' : {
        'managers' : Array<Principal>,
        'extensible' : CandyShared,
      }
    } |
    {
      'auction_bid' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : string,
      }
    } |
    { 'burn' : { 'from' : [] | [Account__2], 'extensible' : CandyShared } } |
    {
      'data' : {
        'hash' : [] | [Uint8Array | number[]],
        'extensible' : CandyShared,
        'data_dapp' : [] | [string],
        'data_path' : [] | [string],
      }
    } |
    {
      'sale_ended' : {
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : [] | [string],
      }
    } |
    {
      'mint' : {
        'to' : Account__2,
        'from' : Account__2,
        'sale' : [] | [{ 'token' : TokenSpec, 'amount' : bigint }],
        'extensible' : CandyShared,
      }
    } |
    {
      'royalty_paid' : {
        'tag' : string,
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'receiver' : Account__2,
        'sale_id' : [] | [string],
      }
    } |
    { 'extensible' : CandyShared } |
    {
      'fee_deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'owner_transfer' : {
        'to' : Account__2,
        'from' : Account__2,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_opened' : {
        'pricing' : PricingConfigShared,
        'extensible' : CandyShared,
        'sale_id' : string,
      }
    } |
    {
      'canister_owner_updated' : {
        'owner' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    },
  'timestamp' : bigint,
  'index' : bigint,
}
export type MarketTransferResult = { 'ok' : MarketTransferRequestReponse } |
  { 'err' : OrigynError };
export type MetricsGranularity = { 'hourly' : null } |
  { 'daily' : null };
export type MinIncreaseType = { 'amount' : bigint } |
  { 'percentage' : number };
export interface NFTBackupChunk {
  'sales_balances' : StableSalesBalances,
  'offers' : StableOffers,
  'collection_data' : StableCollectionData,
  'nft_ledgers' : StableNftLedger,
  'canister' : Principal,
  'allocations' : Array<[[string, string], AllocationRecordStable]>,
  'nft_sales' : Array<[string, SaleStatusShared]>,
  'buckets' : Array<[Principal, StableBucketData]>,
  'escrow_balances' : StableEscrowBalances,
}
export type NFTInfoResult = { 'ok' : NFTInfoStable } |
  { 'err' : OrigynError };
export interface NFTInfoStable {
  'metadata' : CandyShared,
  'current_sale' : [] | [SaleStatusShared],
}
export type NFTUpdateAppResult = { 'ok' : NFTUpdateMetadataNodeResponse } |
  { 'err' : OrigynError };
export interface NFTUpdateMetadataNode {
  'token_id' : string,
  'value' : CandyShared,
  '_system' : boolean,
  'field_id' : string,
}
export interface NFTUpdateMetadataNodeResponse {
  'property_new' : PropertyShared,
  'property_old' : [] | [PropertyShared],
}
export type NFTUpdateRequest = {
    'update' : {
      'token_id' : string,
      'update' : UpdateRequestShared,
      'app_id' : string,
    }
  } |
  { 'replace' : { 'token_id' : string, 'data' : CandyShared } };
export type NFTUpdateResponse = boolean;
export type NFTUpdateResult = { 'ok' : NFTUpdateResponse } |
  { 'err' : OrigynError };
export type Nanos = bigint;
export type NftError = { 'UnauthorizedOperator' : null } |
  { 'SelfTransfer' : null } |
  { 'TokenNotFound' : null } |
  { 'UnauthorizedOwner' : null } |
  { 'TxNotFound' : null } |
  { 'SelfApprove' : null } |
  { 'OperatorNotFound' : null } |
  { 'ExistedNFT' : null } |
  { 'OwnerNotFound' : null } |
  { 'Other' : string };
export interface Nft_Canister {
  '__advance_time' : ActorMethod<[bigint], bigint>,
  '__set_time_mode' : ActorMethod<
    [{ 'test' : null } | { 'standard' : null }],
    boolean
  >,
  '__supports' : ActorMethod<[], Array<[string, string]>>,
  '__version' : ActorMethod<[], string>,
  'back_up' : ActorMethod<
    [bigint],
    { 'eof' : NFTBackupChunk } |
      { 'data' : NFTBackupChunk }
  >,
  'balance' : ActorMethod<[EXTBalanceRequest], EXTBalanceResult>,
  'balanceEXT' : ActorMethod<[EXTBalanceRequest], EXTBalanceResult>,
  'balance_of_batch_nft_origyn' : ActorMethod<
    [Array<Account__1>],
    Array<BalanceResult>
  >,
  'balance_of_nft_origyn' : ActorMethod<[Account__1], BalanceResult>,
  'balance_of_secure_batch_nft_origyn' : ActorMethod<
    [Array<Account__1>],
    Array<BalanceResult>
  >,
  'balance_of_secure_nft_origyn' : ActorMethod<[Account__1], BalanceResult>,
  'bearer' : ActorMethod<[EXTTokenIdentifier], EXTBearerResult>,
  'bearerEXT' : ActorMethod<[EXTTokenIdentifier], EXTBearerResult>,
  'bearer_batch_nft_origyn' : ActorMethod<[Array<string>], Array<BearerResult>>,
  'bearer_batch_secure_nft_origyn' : ActorMethod<
    [Array<string>],
    Array<BearerResult>
  >,
  'bearer_nft_origyn' : ActorMethod<[string], BearerResult>,
  'bearer_secure_nft_origyn' : ActorMethod<[string], BearerResult>,
  'canister_status' : ActorMethod<
    [{ 'canister_id' : canister_id }],
    canister_status
  >,
  'chunk_nft_origyn' : ActorMethod<[ChunkRequest], ChunkResult>,
  'chunk_secure_nft_origyn' : ActorMethod<[ChunkRequest], ChunkResult>,
  'collectCanisterMetrics' : ActorMethod<[], undefined>,
  'collection_nft_origyn' : ActorMethod<
    [[] | [Array<[string, [] | [bigint], [] | [bigint]]>]],
    CollectionResult
  >,
  'collection_secure_nft_origyn' : ActorMethod<
    [[] | [Array<[string, [] | [bigint], [] | [bigint]]>]],
    CollectionResult
  >,
  'collection_update_batch_nft_origyn' : ActorMethod<
    [Array<ManageCollectionCommand>],
    Array<OrigynBoolResult>
  >,
  'collection_update_nft_origyn' : ActorMethod<
    [ManageCollectionCommand],
    OrigynBoolResult
  >,
  'count_unlisted_tokens_of' : ActorMethod<[Account], bigint>,
  'cycles' : ActorMethod<[], bigint>,
  'dip721_balance_of' : ActorMethod<[Principal], bigint>,
  'dip721_custodians' : ActorMethod<[], Array<Principal>>,
  'dip721_is_approved_for_all' : ActorMethod<
    [Principal, Principal],
    DIP721BoolResult
  >,
  'dip721_logo' : ActorMethod<[], [] | [string]>,
  'dip721_metadata' : ActorMethod<[], DIP721Metadata>,
  'dip721_name' : ActorMethod<[], [] | [string]>,
  'dip721_operator_token_identifiers' : ActorMethod<
    [Principal],
    DIP721TokensListMetadata
  >,
  'dip721_operator_token_metadata' : ActorMethod<
    [Principal],
    DIP721TokensMetadata
  >,
  'dip721_owner_of' : ActorMethod<[bigint], OwnerOfResponse>,
  'dip721_owner_token_identifiers' : ActorMethod<
    [Principal],
    DIP721TokensListMetadata
  >,
  'dip721_owner_token_metadata' : ActorMethod<
    [Principal],
    DIP721TokensMetadata
  >,
  'dip721_stats' : ActorMethod<[], DIP721Stats>,
  'dip721_supported_interfaces' : ActorMethod<
    [],
    Array<DIP721SupportedInterface>
  >,
  'dip721_symbol' : ActorMethod<[], [] | [string]>,
  'dip721_token_metadata' : ActorMethod<[bigint], DIP721TokenMetadata>,
  'dip721_total_supply' : ActorMethod<[], bigint>,
  'dip721_total_transactions' : ActorMethod<[], bigint>,
  'dip721_transfer' : ActorMethod<[Principal, bigint], DIP721NatResult>,
  'dip721_transfer_from' : ActorMethod<
    [Principal, Principal, bigint],
    DIP721NatResult
  >,
  'getCanisterLog' : ActorMethod<
    [[] | [CanisterLogRequest]],
    [] | [CanisterLogResponse]
  >,
  'getCanisterMetrics' : ActorMethod<
    [GetMetricsParameters],
    [] | [CanisterMetrics]
  >,
  'getEXTTokenIdentifier' : ActorMethod<[string], string>,
  'get_access_key' : ActorMethod<[], OrigynTextResult>,
  'get_halt' : ActorMethod<[], boolean>,
  'get_nat_as_token_id_origyn' : ActorMethod<[bigint], string>,
  'get_tip' : ActorMethod<[], Tip>,
  'get_token_id_as_nat' : ActorMethod<[string], bigint>,
  'governance_batch_nft_origyn' : ActorMethod<
    [Array<GovernanceRequest>],
    Array<GovernanceResult>
  >,
  'governance_nft_origyn' : ActorMethod<[GovernanceRequest], GovernanceResult>,
  'history_batch_nft_origyn' : ActorMethod<
    [Array<[string, [] | [bigint], [] | [bigint]]>],
    Array<HistoryResult>
  >,
  'history_batch_secure_nft_origyn' : ActorMethod<
    [Array<[string, [] | [bigint], [] | [bigint]]>],
    Array<HistoryResult>
  >,
  'history_nft_origyn' : ActorMethod<
    [string, [] | [bigint], [] | [bigint]],
    HistoryResult
  >,
  'history_secure_nft_origyn' : ActorMethod<
    [string, [] | [bigint], [] | [bigint]],
    HistoryResult
  >,
  'http_access_key' : ActorMethod<[], OrigynTextResult>,
  'http_request' : ActorMethod<[HttpRequest], HTTPResponse>,
  'http_request_streaming_callback' : ActorMethod<
    [StreamingCallbackToken],
    StreamingCallbackResponse
  >,
  'icrc3_get_archives' : ActorMethod<[GetArchivesArgs], GetArchivesResult>,
  'icrc3_get_blocks' : ActorMethod<
    [Array<TransactionRange>],
    GetTransactionsResult
  >,
  'icrc3_get_tip_certificate' : ActorMethod<[], [] | [DataCertificate]>,
  'icrc3_supported_block_types' : ActorMethod<[], Array<BlockType>>,
  'icrc7_approve' : ActorMethod<[ApprovalArgs], ApprovalResult>,
  'icrc7_atomic_batch_transfers' : ActorMethod<[], [] | [boolean]>,
  'icrc7_balance_of' : ActorMethod<[Array<Account>], Array<bigint>>,
  'icrc7_collection_metadata' : ActorMethod<[], CollectionMetadata>,
  'icrc7_default_take_value' : ActorMethod<[], [] | [bigint]>,
  'icrc7_description' : ActorMethod<[], [] | [string]>,
  'icrc7_logo' : ActorMethod<[], [] | [string]>,
  'icrc7_max_approvals_per_token_or_collection' : ActorMethod<
    [],
    [] | [bigint]
  >,
  'icrc7_max_memo_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_query_batch_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_revoke_approvals' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_take_value' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_update_batch_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_name' : ActorMethod<[], string>,
  'icrc7_owner_of' : ActorMethod<[Array<bigint>], Array<[] | [Account]>>,
  'icrc7_permitted_drift' : ActorMethod<[], [] | [bigint]>,
  'icrc7_supply_cap' : ActorMethod<[], [] | [bigint]>,
  'icrc7_supported_standards' : ActorMethod<[], Array<SupportedStandard>>,
  'icrc7_symbol' : ActorMethod<[], string>,
  'icrc7_token_metadata' : ActorMethod<
    [Array<bigint>],
    Array<[] | [Array<[string, Value]>]>
  >,
  'icrc7_tokens' : ActorMethod<[[] | [bigint], [] | [number]], Array<bigint>>,
  'icrc7_tokens_of' : ActorMethod<
    [Account, [] | [bigint], [] | [number]],
    Array<bigint>
  >,
  'icrc7_total_supply' : ActorMethod<[], bigint>,
  'icrc7_transfer' : ActorMethod<[Array<TransferArgs>], TransferResult>,
  'icrc7_transfer_fee' : ActorMethod<[bigint], [] | [bigint]>,
  'icrc7_tx_window' : ActorMethod<[], [] | [bigint]>,
  'manage_storage_nft_origyn' : ActorMethod<
    [ManageStorageRequest],
    ManageStorageResult
  >,
  'market_transfer_batch_nft_origyn' : ActorMethod<
    [Array<MarketTransferRequest>],
    Array<MarketTransferResult>
  >,
  'market_transfer_nft_origyn' : ActorMethod<
    [MarketTransferRequest],
    MarketTransferResult
  >,
  'metadata' : ActorMethod<[], DIP721Metadata>,
  'metadataExt' : ActorMethod<[EXTTokenIdentifier], EXTMetadataResult>,
  'mint_batch_nft_origyn' : ActorMethod<
    [Array<[string, Account__1]>],
    Array<OrigynTextResult>
  >,
  'mint_nft_origyn' : ActorMethod<[string, Account__1], OrigynTextResult>,
  'nftStreamingCallback' : ActorMethod<
    [StreamingCallbackToken],
    StreamingCallbackResponse
  >,
  'nft_batch_origyn' : ActorMethod<[Array<string>], Array<NFTInfoResult>>,
  'nft_batch_secure_origyn' : ActorMethod<
    [Array<string>],
    Array<NFTInfoResult>
  >,
  'nft_origyn' : ActorMethod<[string], NFTInfoResult>,
  'nft_secure_origyn' : ActorMethod<[string], NFTInfoResult>,
  'operaterTokenMetadata' : ActorMethod<[Principal], DIP721TokensMetadata>,
  'ownerOf' : ActorMethod<[bigint], OwnerOfResponse>,
  'ownerTokenMetadata' : ActorMethod<[Principal], DIP721TokensMetadata>,
  'sale_batch_nft_origyn' : ActorMethod<
    [Array<ManageSaleRequest>],
    Array<ManageSaleResult>
  >,
  'sale_info_batch_nft_origyn' : ActorMethod<
    [Array<SaleInfoRequest>],
    Array<SaleInfoResult>
  >,
  'sale_info_batch_secure_nft_origyn' : ActorMethod<
    [Array<SaleInfoRequest>],
    Array<SaleInfoResult>
  >,
  'sale_info_nft_origyn' : ActorMethod<[SaleInfoRequest], SaleInfoResult>,
  'sale_info_secure_nft_origyn' : ActorMethod<
    [SaleInfoRequest],
    SaleInfoResult
  >,
  'sale_nft_origyn' : ActorMethod<[ManageSaleRequest], ManageSaleResult>,
  'set_data_harvester' : ActorMethod<[bigint], undefined>,
  'set_halt' : ActorMethod<[boolean], undefined>,
  'share_wallet_nft_origyn' : ActorMethod<
    [ShareWalletRequest],
    OwnerUpdateResult
  >,
  'stage_batch_nft_origyn' : ActorMethod<
    [Array<{ 'metadata' : CandyShared }>],
    Array<OrigynTextResult>
  >,
  'stage_library_batch_nft_origyn' : ActorMethod<
    [Array<StageChunkArg>],
    Array<StageLibraryResult>
  >,
  'stage_library_nft_origyn' : ActorMethod<[StageChunkArg], StageLibraryResult>,
  'stage_nft_origyn' : ActorMethod<
    [{ 'metadata' : CandyShared }],
    OrigynTextResult
  >,
  'state_size' : ActorMethod<[], StateSize>,
  'storage_info_nft_origyn' : ActorMethod<[], StorageMetricsResult>,
  'storage_info_secure_nft_origyn' : ActorMethod<[], StorageMetricsResult>,
  'tokens_ext' : ActorMethod<[string], EXTTokensResult>,
  'transfer' : ActorMethod<[EXTTransferRequest], EXTTransferResponse>,
  'transferDip721' : ActorMethod<[Principal, bigint], DIP721NatResult>,
  'transferEXT' : ActorMethod<[EXTTransferRequest], EXTTransferResponse>,
  'transferFrom' : ActorMethod<[Principal, Principal, bigint], DIP721NatResult>,
  'transferFromDip721' : ActorMethod<
    [Principal, Principal, bigint],
    DIP721NatResult
  >,
  'unlisted_tokens_of' : ActorMethod<
    [Account, [] | [bigint], [] | [number]],
    Array<bigint>
  >,
  'update_app_nft_origyn' : ActorMethod<[NFTUpdateRequest], NFTUpdateResult>,
  'update_icrc3' : ActorMethod<[Array<UpdateSetting>], Array<boolean>>,
  'update_metadata_node' : ActorMethod<
    [NFTUpdateMetadataNode],
    NFTUpdateAppResult
  >,
  'wallet_receive' : ActorMethod<[], bigint>,
  'whoami' : ActorMethod<[], Principal>,
}
export interface NiftySettlementType {
  'fixed' : boolean,
  'interestRatePerSecond' : number,
  'duration' : [] | [bigint],
  'expiration' : [] | [bigint],
  'lenderOffer' : boolean,
}
export interface NumericEntity {
  'avg' : bigint,
  'max' : bigint,
  'min' : bigint,
  'first' : bigint,
  'last' : bigint,
}
export type OrigynBoolResult = { 'ok' : boolean } |
  { 'err' : OrigynError };
export interface OrigynError {
  'text' : string,
  'error' : Errors,
  'number' : number,
  'flag_point' : string,
}
export type OrigynTextResult = { 'ok' : string } |
  { 'err' : OrigynError };
export type OwnerOfResponse = { 'Ok' : [] | [Principal] } |
  { 'Err' : NftError };
export interface OwnerTransferResponse {
  'transaction' : TransactionRecord,
  'assets' : Array<CandyShared>,
}
export type OwnerUpdateResult = { 'ok' : OwnerTransferResponse } |
  { 'err' : OrigynError };
export type PricingConfigShared = { 'ask' : AskConfigShared } |
  { 'extensible' : CandyShared } |
  { 'instant' : InstantConfigShared } |
  { 'auction' : AuctionConfig };
export type PricingConfigShared__1 = { 'ask' : AskConfigShared } |
  { 'extensible' : CandyShared } |
  { 'instant' : InstantConfigShared } |
  { 'auction' : AuctionConfig };
export interface PropertyShared {
  'value' : CandyShared,
  'name' : string,
  'immutable' : boolean,
}
export interface RecognizeEscrowResponse {
  'balance' : bigint,
  'receipt' : EscrowReceipt,
  'transaction' : [] | [TransactionRecord],
}
export interface RejectDescription {
  'token' : TokenSpec__1,
  'token_id' : string,
  'seller' : Account__1,
  'buyer' : Account__1,
}
export type Result = { 'ok' : ManageSaleResponse } |
  { 'err' : OrigynError };
export type SaleInfoRequest = { 'status' : string } |
  { 'fee_deposit_info' : [] | [Account__1] } |
  { 'active' : [] | [[bigint, bigint]] } |
  { 'deposit_info' : [] | [Account__1] } |
  { 'history' : [] | [[bigint, bigint]] } |
  { 'escrow_info' : EscrowReceipt };
export type SaleInfoResponse = { 'status' : [] | [SaleStatusShared] } |
  { 'fee_deposit_info' : SubAccountInfo } |
  {
    'active' : {
      'eof' : boolean,
      'records' : Array<[string, [] | [SaleStatusShared]]>,
      'count' : bigint,
    }
  } |
  { 'deposit_info' : SubAccountInfo } |
  {
    'history' : {
      'eof' : boolean,
      'records' : Array<[] | [SaleStatusShared]>,
      'count' : bigint,
    }
  } |
  { 'escrow_info' : SubAccountInfo };
export type SaleInfoResult = { 'ok' : SaleInfoResponse } |
  { 'err' : OrigynError };
export interface SaleStatusShared {
  'token_id' : string,
  'sale_type' : { 'auction' : AuctionStateShared },
  'broker_id' : [] | [Principal],
  'original_broker_id' : [] | [Principal],
  'sale_id' : string,
}
export interface SalesConfig {
  'broker_id' : [] | [Account__2],
  'pricing' : PricingConfigShared,
  'escrow_receipt' : [] | [EscrowReceipt__1],
}
export interface ShareWalletRequest {
  'to' : Account__1,
  'token_id' : string,
  'from' : Account__1,
}
export interface StableBucketData {
  'principal' : Principal,
  'allocated_space' : bigint,
  'date_added' : bigint,
  'version' : [bigint, bigint, bigint],
  'b_gateway' : boolean,
  'available_space' : bigint,
  'allocations' : Array<[[string, string], bigint]>,
}
export interface StableCollectionData {
  'active_bucket' : [] | [Principal],
  'managers' : Array<Principal>,
  'owner' : Principal,
  'metadata' : [] | [CandyShared],
  'logo' : [] | [string],
  'name' : [] | [string],
  'network' : [] | [Principal],
  'available_space' : bigint,
  'symbol' : [] | [string],
  'allocated_storage' : bigint,
}
export type StableEscrowBalances = Array<
  [Account__1, Account__1, string, EscrowRecord__1]
>;
export type StableNftLedger = Array<[string, TransactionRecord]>;
export type StableOffers = Array<[Account__1, Account__1, bigint]>;
export type StableSalesBalances = Array<
  [Account__1, Account__1, string, EscrowRecord__1]
>;
export interface StageChunkArg {
  'content' : Uint8Array | number[],
  'token_id' : string,
  'chunk' : bigint,
  'filedata' : CandyShared,
  'library_id' : string,
}
export interface StageLibraryResponse { 'canister' : Principal }
export type StageLibraryResult = { 'ok' : StageLibraryResponse } |
  { 'err' : OrigynError };
export interface StakeRecord {
  'staker' : Account__1,
  'token_id' : string,
  'amount' : bigint,
}
export interface StateSize {
  'sales_balances' : bigint,
  'offers' : bigint,
  'nft_ledgers' : bigint,
  'allocations' : bigint,
  'nft_sales' : bigint,
  'buckets' : bigint,
  'escrow_balances' : bigint,
}
export interface StorageMetrics {
  'gateway' : Principal,
  'available_space' : bigint,
  'allocations' : Array<AllocationRecordStable>,
  'allocated_storage' : bigint,
}
export type StorageMetricsResult = { 'ok' : StorageMetrics } |
  { 'err' : OrigynError };
export interface StreamingCallbackResponse {
  'token' : [] | [StreamingCallbackToken],
  'body' : Uint8Array | number[],
}
export interface StreamingCallbackToken {
  'key' : string,
  'index' : bigint,
  'content_encoding' : string,
}
export type StreamingStrategy = {
    'Callback' : {
      'token' : StreamingCallbackToken,
      'callback' : [Principal, string],
    }
  };
export interface SubAccountInfo {
  'account_id' : Uint8Array | number[],
  'principal' : Principal,
  'account_id_text' : string,
  'account' : {
    'principal' : Principal,
    'sub_account' : Uint8Array | number[],
  },
}
export type Subaccount = Uint8Array | number[];
export interface SupportedStandard { 'url' : string, 'name' : string }
export interface Tip {
  'last_block_index' : Uint8Array | number[],
  'hash_tree' : Uint8Array | number[],
  'last_block_hash' : Uint8Array | number[],
}
export interface TokenIDFilter {
  'filter_type' : { 'allow' : null } |
    { 'block' : null },
  'token_id' : string,
  'tokens' : Array<
    {
      'token' : TokenSpec__1,
      'min_amount' : [] | [bigint],
      'max_amount' : [] | [bigint],
    }
  >,
}
export interface TokenMetadata {
  'transferred_at' : [] | [bigint],
  'transferred_by' : [] | [Principal],
  'owner' : [] | [Principal],
  'operator' : [] | [Principal],
  'approved_at' : [] | [bigint],
  'approved_by' : [] | [Principal],
  'properties' : Array<[string, GenericValue]>,
  'is_burned' : boolean,
  'token_identifier' : bigint,
  'burned_at' : [] | [bigint],
  'burned_by' : [] | [Principal],
  'minted_at' : bigint,
  'minted_by' : Principal,
}
export type TokenSpec = { 'ic' : ICTokenSpec } |
  { 'extensible' : CandyShared };
export interface TokenSpecFilter {
  'token' : TokenSpec__1,
  'filter_type' : { 'allow' : null } |
    { 'block' : null },
}
export type TokenSpec__1 = { 'ic' : ICTokenSpec__1 } |
  { 'extensible' : CandyShared };
export type TokenSpec__2 = { 'ic' : ICTokenSpec } |
  { 'extensible' : CandyShared };
export type TransactionID = { 'nat' : bigint } |
  { 'text' : string } |
  { 'extensible' : CandyShared };
export type TransactionID__1 = { 'nat' : bigint } |
  { 'text' : string } |
  { 'extensible' : CandyShared };
export interface TransactionRange { 'start' : bigint, 'length' : bigint }
export interface TransactionRange__1 { 'start' : bigint, 'length' : bigint }
export interface TransactionRecord {
  'token_id' : string,
  'txn_type' : {
      'escrow_deposit' : {
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'fee_deposit' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_network_updated' : {
        'network' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'escrow_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_managers_updated' : {
        'managers' : Array<Principal>,
        'extensible' : CandyShared,
      }
    } |
    {
      'auction_bid' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : string,
      }
    } |
    { 'burn' : { 'from' : [] | [Account__2], 'extensible' : CandyShared } } |
    {
      'data' : {
        'hash' : [] | [Uint8Array | number[]],
        'extensible' : CandyShared,
        'data_dapp' : [] | [string],
        'data_path' : [] | [string],
      }
    } |
    {
      'sale_ended' : {
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : [] | [string],
      }
    } |
    {
      'mint' : {
        'to' : Account__2,
        'from' : Account__2,
        'sale' : [] | [{ 'token' : TokenSpec, 'amount' : bigint }],
        'extensible' : CandyShared,
      }
    } |
    {
      'royalty_paid' : {
        'tag' : string,
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'receiver' : Account__2,
        'sale_id' : [] | [string],
      }
    } |
    { 'extensible' : CandyShared } |
    {
      'fee_deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'owner_transfer' : {
        'to' : Account__2,
        'from' : Account__2,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_opened' : {
        'pricing' : PricingConfigShared,
        'extensible' : CandyShared,
        'sale_id' : string,
      }
    } |
    {
      'canister_owner_updated' : {
        'owner' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    },
  'timestamp' : bigint,
  'index' : bigint,
}
export interface TransferArgs {
  'to' : Account,
  'token_id' : bigint,
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
}
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'NonExistingTokenId' : null } |
  { 'Unauthorized' : null } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null };
export type TransferResult = Array<[] | [TransferResultItem]>;
export interface TransferResultItem {
  'token_id' : bigint,
  'transfer_result' : { 'Ok' : bigint } |
    { 'Err' : TransferError },
}
export type UpdateCallsAggregatedData = BigUint64Array | bigint[];
export type UpdateModeShared = { 'Set' : CandyShared } |
  { 'Lock' : CandyShared } |
  { 'Next' : Array<UpdateShared> };
export interface UpdateRequestShared {
  'id' : string,
  'update' : Array<UpdateShared>,
}
export type UpdateSetting = { 'maxRecordsToArchive' : bigint } |
  { 'archiveIndexType' : IndexType } |
  { 'maxArchivePages' : bigint } |
  { 'settleToRecords' : bigint } |
  { 'archiveCycles' : bigint } |
  { 'maxActiveRecords' : bigint } |
  { 'maxRecordsInArchiveInstance' : bigint } |
  { 'archiveControllers' : [] | [[] | [Array<Principal>]] };
export interface UpdateShared { 'mode' : UpdateModeShared, 'name' : string }
export type Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value> };
export type Value__1 = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value__1]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value__1> };
export type Vec = Array<
  [
    string,
    { 'Nat64Content' : bigint } |
      { 'Nat32Content' : number } |
      { 'BoolContent' : boolean } |
      { 'Nat8Content' : number } |
      { 'Int64Content' : bigint } |
      { 'IntContent' : bigint } |
      { 'NatContent' : bigint } |
      { 'Nat16Content' : number } |
      { 'Int32Content' : number } |
      { 'Int8Content' : number } |
      { 'FloatContent' : number } |
      { 'Int16Content' : number } |
      { 'BlobContent' : Uint8Array | number[] } |
      { 'NestedContent' : Vec } |
      { 'Principal' : Principal } |
      { 'TextContent' : string },
  ]
>;
export interface WaitForQuietType {
  'max' : bigint,
  'fade' : number,
  'extension' : bigint,
}
export interface WithdrawDescription {
  'token' : TokenSpec__1,
  'token_id' : string,
  'seller' : Account__1,
  'withdraw_to' : Account__1,
  'buyer' : Account__1,
  'amount' : bigint,
}
export type WithdrawRequest = { 'reject' : RejectDescription } |
  { 'fee_deposit' : FeeDepositWithdrawDescription } |
  { 'sale' : WithdrawDescription } |
  { 'deposit' : DepositWithdrawDescription } |
  { 'escrow' : WithdrawDescription };
export interface WithdrawResponse {
  'token_id' : string,
  'txn_type' : {
      'escrow_deposit' : {
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'fee_deposit' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_network_updated' : {
        'network' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'escrow_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'canister_managers_updated' : {
        'managers' : Array<Principal>,
        'extensible' : CandyShared,
      }
    } |
    {
      'auction_bid' : {
        'token' : TokenSpec,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : string,
      }
    } |
    { 'burn' : { 'from' : [] | [Account__2], 'extensible' : CandyShared } } |
    {
      'data' : {
        'hash' : [] | [Uint8Array | number[]],
        'extensible' : CandyShared,
        'data_dapp' : [] | [string],
        'data_path' : [] | [string],
      }
    } |
    {
      'sale_ended' : {
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'sale_id' : [] | [string],
      }
    } |
    {
      'mint' : {
        'to' : Account__2,
        'from' : Account__2,
        'sale' : [] | [{ 'token' : TokenSpec, 'amount' : bigint }],
        'extensible' : CandyShared,
      }
    } |
    {
      'royalty_paid' : {
        'tag' : string,
        'token' : TokenSpec,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
        'receiver' : Account__2,
        'sale_id' : [] | [string],
      }
    } |
    { 'extensible' : CandyShared } |
    {
      'fee_deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'account' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'owner_transfer' : {
        'to' : Account__2,
        'from' : Account__2,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_opened' : {
        'pricing' : PricingConfigShared,
        'extensible' : CandyShared,
        'sale_id' : string,
      }
    } |
    {
      'canister_owner_updated' : {
        'owner' : Principal,
        'extensible' : CandyShared,
      }
    } |
    {
      'sale_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'token_id' : string,
        'trx_id' : TransactionID,
        'seller' : Account__2,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    } |
    {
      'deposit_withdraw' : {
        'fee' : bigint,
        'token' : TokenSpec,
        'trx_id' : TransactionID,
        'extensible' : CandyShared,
        'buyer' : Account__2,
        'amount' : bigint,
      }
    },
  'timestamp' : bigint,
  'index' : bigint,
}
export type canister_id = Principal;
export interface canister_status {
  'status' : { 'stopped' : null } |
    { 'stopping' : null } |
    { 'running' : null },
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : definite_canister_settings,
  'module_hash' : [] | [Uint8Array | number[]],
}
export interface definite_canister_settings {
  'freezing_threshold' : bigint,
  'controllers' : [] | [Array<Principal>],
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface _SERVICE extends Nft_Canister {}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
