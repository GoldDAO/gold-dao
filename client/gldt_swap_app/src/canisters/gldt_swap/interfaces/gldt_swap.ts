/* eslint-disable @typescript-eslint/no-empty-object-type */
import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
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
export interface ArchiveCanister {
  'canister_id' : Principal,
  'end_index' : [] | [bigint],
  'start_index' : bigint,
}
export type ArchiveDownReason = { 'UpgradingArchivesFailed' : string } |
  { 'NoArchiveCanisters' : string } |
  { 'Upgrading' : null } |
  { 'InitializingFirstArchiveFailed' : string } |
  { 'ActiveSwapCapacityFull' : null } |
  { 'LowOrigynToken' : string };
export interface Args { 'page' : bigint, 'limit' : bigint }
export interface Args_1 {
  'page' : bigint,
  'user' : Principal,
  'limit' : bigint,
}
export interface Args_2 { 'nft_id' : bigint, 'nft_canister_id' : Principal }
export type Args_3 = { 'Upgrade' : UpgradeArgs } |
  { 'Init' : InitArgs };
export type BidFailError = { 'UnexpectedError' : string } |
  { 'CallError' : string } |
  { 'TransferFailed' : string };
export interface BuildVersion {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export type BurnError = { 'CallError' : string };
export type DepositRecoveryError = { 'CantRecover' : string } |
  { 'CallError' : string };
export type EscrowError = { 'ApproveError' : ApproveError } |
  { 'UnexpectedError' : ImpossibleErrorReason } |
  { 'TransferFailed' : TransferFailReason } |
  { 'RequestFailed' : string };
export type FeeTransferError = { 'TransferError' : TransferError } |
  { 'CallError' : string };
export type GetHistoricSwapsByUserError = { 'LimitTooLarge' : string } |
  { 'LimitTooSmall' : string } |
  { 'QueryCanisterError' : string };
export type GetHistoricSwapsError = { 'LimitTooLarge' : string };
export type GetNftMetaDetailErrorReason = { 'CantFindNFT' : string } |
  { 'NoMetaDetails' : null } |
  { 'UnexpectedError' : string };
export interface GldtNumTokens { 'value_with_fee' : bigint, 'value' : bigint }
export type ImpossibleErrorReason = { 'AmountNotFound' : null } |
  { 'NFTResponseInvalid' : null } |
  { 'PrincipalNotFound' : null };
export interface InitArgs {
  'test_mode' : boolean,
  'ogy_ledger_id' : Principal,
  'authorized_principals' : Array<Principal>,
  'version' : BuildVersion,
  'gldnft_canisters' : Array<[Principal, NftCanisterConf]>,
  'gldt_ledger_id' : Principal,
  'commit_hash' : string,
}
export type LockError = { 'NftAlreadyLocked' : Array<bigint> } |
  { 'UnexpectedError' : {} } |
  { 'NftNotLocked' : null };
export type MintError = { 'UnexpectedError' : ImpossibleErrorReason } |
  { 'TransferFailed' : TransferFailReason };
export interface NftCanisterConf { 'grams' : number }
export type NftInvalidError = { 'InvalidNftOwner' : string } |
  { 'AlreadyLocked' : null } |
  { 'CantGetOrigynID' : string } |
  { 'InvalidNFTCollectionPrincipal' : null } |
  { 'InvalidTokenAmount' : null } |
  { 'CantGetNatIdOfNft' : null };
export type NftTransferError = { 'FailedToGetOgyFeeAllowance' : string } |
  { 'ApprovalError' : ApproveError } |
  { 'ApprovalCallError' : string } |
  { 'InvalidFee' : string } |
  { 'UnexpectedError' : ImpossibleErrorReason } |
  { 'CallError' : string } |
  { 'TransferFailed' : string };
export type NftValidationError = { 'WeightParseError' : null } |
  { 'CanisterInvalid' : null } |
  { 'CantGetOrigynID' : string } |
  { 'CantVerifySwapCanisterOwnsNft' : null } |
  { 'InvalidGldtTokensFromWeight' : null } |
  { 'InvalidNftWeight' : null } |
  { 'NotOwnedBySwapCanister' : null };
export type NotificationError = { 'InvalidSaleSubaccount' : null } |
  { 'InvalidTokenSpec' : null } |
  { 'TimeoutInvalid' : string } |
  { 'InvalidEscrowSubaccount' : string } |
  { 'TooManyPrincipalsInAllowList' : null } |
  { 'OrigynStringIdDoesNotMatch' : string } |
  { 'SellerIsNotPrincipalOrAccount' : string } |
  { 'SellerAndReceiverDoesNotMatch' : string } |
  { 'InvalidCustomAskFeature' : null } |
  { 'InvalidTokenAmount' : null } |
  { 'InvalidPricingConfig' : null } |
  { 'CollectionDoesNotMatch' : string } |
  { 'AllowListDoesNotContainCorrectPrincipal' : null };
export type RefundError = { 'CallError' : string } |
  { 'TransferFailed' : TransferError };
export type RemoveIntentToSwapError = { 'InvalidSwapType' : string } |
  { 'InvalidUser' : null } |
  { 'SwapNotFound' : null } |
  { 'InProgress' : null };
export type Result = { 'Ok' : Array<[[bigint, bigint], SwapInfo]> } |
  { 'Err' : GetHistoricSwapsError };
export type Result_1 = { 'Ok' : Array<[[bigint, bigint], SwapInfo]> } |
  { 'Err' : GetHistoricSwapsByUserError };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : RemoveIntentToSwapError };
export type Result_3 = { 'Ok' : Array<[bigint, bigint]> } |
  { 'Err' : SwapNftForTokensErrors };
export type Result_4 = { 'Ok' : [bigint, bigint] } |
  { 'Err' : SwapTokensForNftRequestErrors };
export type ServiceDownReason = { 'ArchiveRelated' : ArchiveDownReason } |
  { 'Initializing' : null } |
  { 'ActiveSwapCapacityFull' : null } |
  { 'LowOrigynToken' : string };
export interface SwapDetailForward {
  'nft_id' : bigint,
  'status' : SwapStatusForward,
  'escrow_sub_account' : Uint8Array | number[],
  'nft_id_string' : string,
  'created_at' : bigint,
  'gldt_receiver' : Account,
  'tokens_to_mint' : GldtNumTokens,
  'nft_canister' : Principal,
  'index' : bigint,
  'sale_id' : string,
}
export interface SwapDetailReverse {
  'nft_id' : bigint,
  'status' : SwapStatusReverse,
  'tokens_to_receive' : GldtNumTokens,
  'nft_id_string' : string,
  'user' : Principal,
  'created_at' : bigint,
  'swap_fee' : bigint,
  'nft_canister' : Principal,
  'index' : bigint,
  'transfer_fees' : bigint,
}
export type SwapErrorForward = {
    'DepositRecoveryFailed' : DepositRecoveryError
  } |
  { 'BidFailed' : BidFailError } |
  { 'UnexpectedError' : ImpossibleErrorReason } |
  { 'NotificationFailed' : NotificationError } |
  { 'MintFailed' : MintError } |
  { 'Expired' : null };
export type SwapErrorReverse = { 'FeeTransferFailed' : FeeTransferError } |
  { 'EscrowFailed' : EscrowError } |
  { 'LockFailed' : LockError } |
  { 'Refunded' : SwapStatusReverse } |
  { 'NftValidationFailed' : Array<NftValidationError> } |
  { 'BurnFailed' : BurnError } |
  { 'NftTransferFailed' : NftTransferError };
export type SwapInfo = { 'Forward' : SwapDetailForward } |
  { 'Reverse' : SwapDetailReverse };
export type SwapNftForTokensErrors = { 'Limit' : string } |
  { 'ContainsDuplicates' : string } |
  { 'ContainsInvalidNftCanister' : string } |
  {
    'NftValidationErrors' : [
      Array<bigint>,
      Array<[bigint, Array<NftInvalidError>]>,
    ]
  } |
  { 'ServiceDown' : ServiceDownReason };
export type SwapStatusForward = {
    'DepositRecoveryFailed' : [SwapStatusForward, DepositRecoveryError]
  } |
  { 'Failed' : SwapErrorForward } |
  { 'DepositRecoveryInProgress' : SwapStatusForward } |
  { 'BidInProgress' : null } |
  { 'Init' : null } |
  { 'MintRequest' : null } |
  { 'DepositRecoveryRequest' : SwapStatusForward } |
  { 'Complete' : null } |
  { 'BidFail' : BidFailError } |
  { 'BidRequest' : null } |
  { 'NotificationFailed' : NotificationError } |
  { 'MintInProgress' : null } |
  { 'BurnFeesInProgress' : null } |
  { 'BurnFeesRequest' : null } |
  { 'BurnFeesFailed' : MintError } |
  { 'NotificationInProgress' : null } |
  { 'MintFailed' : MintError };
export type SwapStatusReverse = { 'NftTransferRequestInProgress' : null } |
  { 'FeeTransferFailed' : FeeTransferError } |
  { 'Failed' : SwapErrorReverse } |
  { 'EscrowFailed' : EscrowError } |
  { 'Init' : null } |
  { 'BurnRequestInProgress' : null } |
  { 'EscrowRequestInProgress' : null } |
  { 'Complete' : null } |
  { 'BurnFailed' : BurnError } |
  { 'RefundRequestInProgress' : null } |
  { 'RefundRequest' : null } |
  { 'NftTransferRequest' : null } |
  { 'FeeTransferRequestInProgress' : null } |
  { 'NftTransferFailed' : NftTransferError } |
  { 'BurnRequest' : null } |
  { 'FeeTransferRequest' : null } |
  { 'RefundFailed' : RefundError } |
  { 'EscrowRequest' : null };
export type SwapTokensForNftRequestErrors = {
    'GetNftMetaDetailError' : GetNftMetaDetailErrorReason
  } |
  { 'CantForgeSwapId' : null } |
  { 'NftLocked' : LockError } |
  { 'NftValidationErrors' : Array<NftValidationError> } |
  { 'NotOwnedBySwapCanister' : null } |
  { 'ServiceDown' : ServiceDownReason } |
  { 'SwapCreationError' : null };
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
export type TransferFailReason = { 'TransferError' : TransferError } |
  { 'TransferFromError' : TransferFromError } |
  { 'CallError' : string };
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
  'version' : BuildVersion,
  'commit_hash' : string,
}
export interface _SERVICE {
  'get_active_swap_ids_by_user' : ActorMethod<
    [[] | [Principal]],
    Array<[bigint, bigint]>
  >,
  'get_active_swaps' : ActorMethod<[null], Array<[[bigint, bigint], SwapInfo]>>,
  'get_active_swaps_by_user' : ActorMethod<
    [[] | [Principal]],
    Array<[[bigint, bigint], SwapInfo]>
  >,
  'get_archive_canisters' : ActorMethod<[null], Array<ArchiveCanister>>,
  'get_historic_swaps' : ActorMethod<[Args], Result>,
  'get_historic_swaps_by_user' : ActorMethod<[Args_1], Result_1>,
  'get_history_total' : ActorMethod<[[] | [Principal]], bigint>,
  'get_swap' : ActorMethod<
    [[bigint, bigint]],
    [] | [[[bigint, bigint], SwapInfo]]
  >,
  'remove_intent_to_swap' : ActorMethod<[[bigint, bigint]], Result_2>,
  'swap_nft_for_tokens' : ActorMethod<[Array<[bigint, Principal]>], Result_3>,
  'swap_tokens_for_nft' : ActorMethod<[Args_2], Result_4>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
