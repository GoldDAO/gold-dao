import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner': Principal,
  'subaccount': [] | [Uint8Array | number[]],
}
export interface ArchivedBlocks {
  'args': Array<GetBlocksRequest>,
  'callback': [Principal, string],
}
export interface Args {
  'principal': [] | [Principal],
  'canister_id': Principal,
}
export type Args_1 = { 'Upgrade': UpgradeArgs } |
{ 'Init': InitArgs };
export interface BlockWithId { 'id': bigint, 'block': ICRC3Value }
export interface BuildVersion {
  'major': number,
  'minor': number,
  'patch': number,
}
export interface CustomFractionalizationConfig {
  'per_token_config': Array<[bigint, GeneralFractionalizationConfig]>,
}
export interface Duration { 'secs': bigint, 'nanos': number }
export type FractionalizationConfig = {
  'Custom': CustomFractionalizationConfig
} |
{ 'General': GeneralFractionalizationConfig };
export type GeneralError = { 'InvalidConfig': string } |
{ 'InvalidNftCanister': string } |
{ 'TransactionAddError': string } |
{ 'TransferError': string } |
{ 'UserIsNotNftOwner': string } |
{ 'AlreadyProcessing': string } |
{ 'TransactionPreparationError': string } |
{ 'InvalidPrincipal': string } |
{ 'NotAuthorized': string } |
{ 'EmptyArgs': string } |
{ 'CallError': string } |
{ 'CanisterIsNotNftOwner': string } |
{ 'ConfigNotFound': string } |
{ 'InvalidPercentage': string };
export interface GeneralFractionalizationConfig {
  'division': bigint,
  'ledger_id': Principal,
  'swap_fee': bigint,
}
export interface GetBlocksRequest { 'start': bigint, 'length': bigint }
export interface GetBlocksResult {
  'log_length': bigint,
  'blocks': Array<BlockWithId>,
  'archived_blocks': Array<ArchivedBlocks>,
}
export interface ICRC3ArchiveInfo {
  'end': bigint,
  'canister_id': Principal,
  'start': bigint,
}
export interface ICRC3Config {
  'constants': ICRC3Properties,
  'supported_blocks': Array<SupportedBlockType>,
}
export interface ICRC3DataCertificate {
  'certificate': Uint8Array | number[],
  'hash_tree': Uint8Array | number[],
}
export interface ICRC3Properties {
  'max_blocks_per_response': bigint,
  'initial_cycles': bigint,
  'tx_window': Duration,
  'max_transactions_to_purge': bigint,
  'max_memory_size_bytes': bigint,
  'ttl_for_non_archived_transactions': Duration,
  'max_transactions_in_window': bigint,
  'max_unarchived_transactions': bigint,
  'reserved_cycles': bigint,
}
export type ICRC3Value = { 'Int': bigint } |
{ 'Map': Array<[string, ICRC3Value]> } |
{ 'Nat': bigint } |
{ 'Blob': Uint8Array | number[] } |
{ 'Text': string } |
{ 'Array': Array<ICRC3Value> };
export interface Icrc28TrustedOriginsResponse {
  'trusted_origins': Array<string>,
}
export interface InitArgs {
  'test_mode': boolean,
  'authorized_principals': Array<Principal>,
  'version': BuildVersion,
  'icrc3_config': ICRC3Config,
  'commit_hash': string,
  'swap_configs': Array<SwapCanisterConfig>,
}
export interface Nft { 'id': bigint, 'canister_id': Principal }
export type Response = { 'Success': null } |
{ 'InternalError': string };
export type Result = { 'Ok': Array<[Principal, Array<bigint>]> } |
{ 'Err': GeneralError };
export type Result_1 = { 'Ok': Array<bigint> } |
{ 'Err': GeneralError };
export type Result_2 = { 'Ok': Array<bigint> } |
{ 'Err': SwapNftForTokensErrors };
export type Result_3 = { 'Ok': Array<bigint> } |
{ 'Err': SwapTokensForNftErrors };
export interface SupportedBlockType { 'url': string, 'block_type': string }
export interface SupportedStandard { 'url': string, 'name': string }
export interface SwapCanisterConfig {
  'icrc7_canister_id': Principal,
  'fractionalization_config': FractionalizationConfig,
}
export interface SwapInfo {
  'nft': Nft,
  'status': SwapStatus,
  'created_at': bigint,
  'tokens_amount': GeneralFractionalizationConfig,
  'user_account': Account,
  'index': bigint,
  'swap_type': SwapType,
}
export type SwapNftForTokensErrors = { 'Limit': string } |
{ 'GeneralError': GeneralError } |
{ 'Retry': [bigint, string] } |
{ 'CantBeAnonymous': string };
export type SwapStatus = { 'Burned': null } |
{ 'Failed': string } |
{ 'NftTransferredFrom': null } |
{ 'Init': null } |
{ 'NftTransferred': null } |
{ 'Complete': null } |
{ 'BurnFailed': string } |
{ 'ReimburseFailed': string } |
{ 'Minted': null } |
{ 'NftTransferFailed': string } |
{ 'NftTransferFromFailed': string } |
{ 'Reimbursed': null } |
{ 'MintFailed': string };
export type SwapTokensForNftErrors = { 'Limit': string } |
{ 'GeneralError': GeneralError } |
{ 'Retry': [bigint, string] } |
{ 'NotOwnedBySwapCanister': null } |
{ 'SwapCreationError': null };
export type SwapType = { 'Forward': null } |
{ 'Reverse': null };
export interface UpgradeArgs {
  'version': BuildVersion,
  'commit_hash': string,
}
export interface icrc21_consent_info {
  'metadata': icrc21_consent_message_metadata,
  'consent_message': icrc21_consent_message,
}
export interface icrc21_consent_message {
  'generic_display_message': string,
  'fields_display_message': icrc21_field_display_message,
}
export interface icrc21_consent_message_metadata {
  'utc_offset_minutes': [] | [number],
  'language': string,
}
export interface icrc21_consent_message_request {
  'arg': Uint8Array | number[],
  'method': string,
  'user_preferences': icrc21_consent_message_spec,
}
export type icrc21_consent_message_response = { 'Ok': icrc21_consent_info } |
{ 'Err': icrc21_error };
export interface icrc21_consent_message_spec {
  'metadata': icrc21_consent_message_metadata,
  'device_spec': [] | [icrc21_device_spec],
}
export type icrc21_device_spec = { 'GenericDisplay': null } |
{ 'FieldsDisplay': null };
export type icrc21_error = { 'GenericError': icrc21_error_info } |
{ 'InsufficientPayment': icrc21_generic_error } |
{ 'UnsupportedCanisterCall': icrc21_error_info } |
{ 'ConsentMessageUnavailable': icrc21_error_info };
export interface icrc21_error_info { 'description': string }
export interface icrc21_field_display_message {
  'fields': Array<[string, string]>,
  'intent': string,
}
export interface icrc21_generic_error {
  'description': string,
  'error_code': bigint,
}
export interface _SERVICE {
  'commit': ActorMethod<[], undefined>,
  'get_active_swap_ids_by_user': ActorMethod<
    [[] | [Principal]],
    Array<bigint>
  >,
  'get_active_swaps': ActorMethod<[null], Array<[bigint, SwapInfo]>>,
  'get_active_swaps_by_ids': ActorMethod<
    [Array<bigint>],
    Array<[bigint, SwapInfo]>
  >,
  'get_active_swaps_by_user': ActorMethod<
    [[] | [Principal]],
    Array<[bigint, SwapInfo]>
  >,
  'get_available_nfts': ActorMethod<[[] | [Principal]], Result>,
  'get_available_nfts_for_canister': ActorMethod<[Args], Result_1>,
  'get_swap_configs': ActorMethod<[null], Array<SwapCanisterConfig>>,
  'icrc10_supported_standards': ActorMethod<[], Array<SupportedStandard>>,
  'icrc21_canister_call_consent_message': ActorMethod<
    [icrc21_consent_message_request],
    icrc21_consent_message_response
  >,
  'icrc28_trusted_origins': ActorMethod<[], Icrc28TrustedOriginsResponse>,
  'icrc3_get_archives': ActorMethod<[null], Array<ICRC3ArchiveInfo>>,
  'icrc3_get_blocks': ActorMethod<[Array<GetBlocksRequest>], GetBlocksResult>,
  'icrc3_get_properties': ActorMethod<[null], ICRC3Properties>,
  'icrc3_get_tip_certificate': ActorMethod<[null], ICRC3DataCertificate>,
  'icrc3_supported_block_types': ActorMethod<
    [null],
    Array<SupportedBlockType>
  >,
  'set_buyback_canister': ActorMethod<[[] | [Account]], Response>,
  'swap_nft_for_tokens': ActorMethod<[Array<Nft>], Result_2>,
  'swap_tokens_for_nft': ActorMethod<[Array<Nft>], Result_3>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];