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
export interface Args {
  'user_principal' : [] | [Principal],
  'limit' : bigint,
  'start' : bigint,
}
export type Args_1 = { 'Upgrade' : UpgradeArgs } |
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
export type EscrowError = { 'ApproveError' : ApproveError } |
  { 'UnexpectedError' : ImpossibleErrorReason } |
  { 'TransferFailed' : TransferFailReason } |
  { 'RequestFailed' : string };
export type FeeTransferError = { 'TransferError' : TransferError } |
  { 'CallError' : string };
export interface GldtNumTokens { 'value_with_fee' : bigint, 'value' : bigint }
export type ImpossibleErrorReason = { 'AmountNotFound' : null } |
  { 'NFTResponseInvalid' : null } |
  { 'PrincipalNotFound' : null };
export interface InitArgs {
  'test_mode' : boolean,
  'authorized_principals' : Array<Principal>,
  'commit_hash' : string,
}
export type LockError = { 'NftAlreadyLocked' : Array<bigint> } |
  { 'UnexpectedError' : {} } |
  { 'NftNotLocked' : null };
export type MintError = { 'UnexpectedError' : ImpossibleErrorReason } |
  { 'TransferFailed' : TransferFailReason };
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
export type SwapErrorForward = { 'BidFailed' : BidFailError } |
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
export type SwapStatusForward = { 'Failed' : SwapErrorForward } |
  { 'Init' : null } |
  { 'MintRequest' : null } |
  { 'Complete' : null } |
  { 'BidFail' : BidFailError } |
  { 'BidRequest' : null } |
  { 'NotificationFailed' : NotificationError } |
  { 'BurnFeesRequest' : null } |
  { 'BurnFeesFailed' : MintError } |
  { 'MintFailed' : MintError };
export type SwapStatusReverse = { 'FeeTransferFailed' : FeeTransferError } |
  { 'Failed' : SwapErrorReverse } |
  { 'EscrowFailed' : EscrowError } |
  { 'Init' : null } |
  { 'Complete' : null } |
  { 'BurnFailed' : BurnError } |
  { 'RefundRequest' : null } |
  { 'NftTransferRequest' : null } |
  { 'NftTransferFailed' : NftTransferError } |
  { 'BurnRequest' : null } |
  { 'FeeTransferRequest' : null } |
  { 'RefundFailed' : RefundError } |
  { 'EscrowRequest' : null };
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
  'archive_swap' : ActorMethod<[[[bigint, bigint], SwapInfo]], null>,
  'archive_swaps' : ActorMethod<[Array<[[bigint, bigint], SwapInfo]>], null>,
  'get_archive_size' : ActorMethod<[null], bigint>,
  'get_archive_swap' : ActorMethod<
    [[bigint, bigint]],
    [] | [[[bigint, bigint], SwapInfo]]
  >,
  'get_archive_swaps' : ActorMethod<
    [Args],
    Array<[[bigint, bigint], SwapInfo]>
  >,
  'get_swap_bulk' : ActorMethod<[Array<[bigint, bigint]>], Array<SwapInfo>>,
  'get_swap_indexes_for_user' : ActorMethod<
    [Principal],
    [] | [Array<[bigint, bigint]>]
  >,
  'get_version' : ActorMethod<[null], BuildVersion>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
