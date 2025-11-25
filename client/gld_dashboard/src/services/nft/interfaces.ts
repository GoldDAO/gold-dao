/* eslint-disable */
import type { Principal } from "@dfinity/principal";
import type { ActorMethod } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";

export interface Account {
  owner: Principal;
  subaccount: [] | [Uint8Array | number[]];
}
export interface ApprovalInfo {
  memo: [] | [Uint8Array | number[]];
  from_subaccount: [] | [Uint8Array | number[]];
  created_at_time: bigint;
  expires_at: [] | [bigint];
  spender: Account;
}
export interface ApproveCollectionArg {
  approval_info: ApprovalInfo;
}
export type ApproveCollectionError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { InvalidSpender: null }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { GenericBatchError: { message: string; error_code: bigint } }
  | { TooOld: null };
export type ApproveCollectionResult =
  | { Ok: bigint }
  | { Err: ApproveCollectionError };
export interface ApproveTokenArg {
  token_id: bigint;
  approval_info: ApprovalInfo;
}
export type ApproveTokenError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { InvalidSpender: null }
  | { NonExistingTokenId: null }
  | { Unauthorized: null }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { GenericBatchError: { message: string; error_code: bigint } }
  | { TooOld: null };
export type ApproveTokenResult = { Ok: bigint } | { Err: ApproveTokenError };
export interface ArchivedBlocks {
  args: Array<GetBlocksRequest>;
  callback: [Principal, string];
}
export interface Args {
  file_path: string;
}
export interface Args_1 {
  principal: Principal;
}
export interface Args_2 {
  permission: Permission;
  principal: Principal;
}
export interface Args_3 {
  permission: Permission;
  principal: Principal;
}
export interface Args_4 {
  file_hash: string;
  file_path: string;
  file_size: bigint;
  chunk_size: [] | [bigint];
}
export interface Args_5 {
  mint_requests: Array<MintRequest>;
}
export interface Args_6 {
  chunk_id: bigint;
  file_path: string;
  chunk_data: Uint8Array | number[];
}
export interface Args_7 {
  supply_cap: [] | [bigint];
  tx_window: [] | [bigint];
  default_take_value: [] | [bigint];
  max_canister_storage_threshold: [] | [bigint];
  logo: [] | [string];
  permitted_drift: [] | [bigint];
  name: [] | [string];
  description: [] | [string];
  max_take_value: [] | [bigint];
  max_update_batch_size: [] | [bigint];
  max_query_batch_size: [] | [bigint];
  max_memo_size: [] | [bigint];
  atomic_batch_transfers: [] | [boolean];
  collection_metadata: [] | [Array<[string, CustomValue]>];
  symbol: [] | [string];
}
export interface Args_8 {
  token_id: bigint;
  metadata: Array<[string, CustomValue]>;
}
export type Args_9 = { Upgrade: UpgradeArgs } | { Init: InitArgs };
export interface BlockWithId {
  id: bigint;
  block: ICRC3Value;
}
export interface BuildVersion {
  major: number;
  minor: number;
  patch: number;
}
export type BurnNftError =
  | { StorageCanisterError: string }
  | { TokenDoesNotExist: null }
  | { ConcurrentManagementCall: null }
  | { NotTokenOwner: null };
export type CancelUploadError = { UploadNotInitialized: null };
export interface ConsentInfo {
  metadata: ConsentMessageMetadata;
  consent_message: ConsentMessage;
}
export type ConsentMessage =
  | {
      LineDisplayMessage: { pages: Array<LineDisplayPage> };
    }
  | { GenericDisplayMessage: string };
export interface ConsentMessageMetadata {
  utc_offset_minutes: [] | [number];
  language: string;
}
export interface ConsentMessageRequest {
  arg: Uint8Array | number[];
  method: string;
  user_preferences: ConsentMessageSpec;
}
export interface ConsentMessageSpec {
  metadata: ConsentMessageMetadata;
  device_spec: [] | [DisplayMessageType];
}
export type CustomValue =
  | { Int: bigint }
  | { Map: Array<[string, ICRC3Value]> }
  | { Nat: bigint }
  | { Blob: Uint8Array | number[] }
  | { Text: string }
  | { Array: Array<ICRC3Value> };
export type DisplayMessageType =
  | { GenericDisplay: null }
  | {
      LineDisplay: {
        characters_per_line: number;
        lines_per_page: number;
      };
    };
export interface Duration {
  secs: bigint;
  nanos: number;
}
export interface ErrorInfo {
  description: string;
}
export type FinalizeUploadError =
  | { IncompleteUpload: null }
  | { FileSizeMismatch: null }
  | { FileHashMismatch: null }
  | { UploadNotStarted: null }
  | { UploadAlreadyFinalized: null };
export interface FinalizeUploadResp {
  url: string;
}
export type GetAllUploadsError = { StorageCanisterError: string };
export interface GetBlocksRequest {
  start: bigint;
  length: bigint;
}
export interface GetBlocksResult {
  log_length: bigint;
  blocks: Array<BlockWithId>;
  archived_blocks: Array<ArchivedBlocks>;
}
export type GetUploadStatusError =
  | { StorageCanisterError: string }
  | { UploadNotFound: null };
export type GetUserPermissionsError =
  | { DefaultError: string }
  | { UserNotFound: null };
export type GrantPermissionError =
  | { ConcurrentManagementCall: null }
  | { DefaultError: string };
export type HasPermissionError = { DefaultError: string };
export interface ICRC3ArchiveInfo {
  end: bigint;
  canister_id: Principal;
  start: bigint;
}
export interface ICRC3DataCertificate {
  certificate: Uint8Array | number[];
  hash_tree: Uint8Array | number[];
}
export interface ICRC3Properties {
  max_blocks_per_response: bigint;
  initial_cycles: bigint;
  tx_window: Duration;
  max_transactions_to_purge: bigint;
  max_memory_size_bytes: bigint;
  ttl_for_non_archived_transactions: Duration;
  max_transactions_in_window: bigint;
  max_unarchived_transactions: bigint;
  reserved_cycles: bigint;
}
export type ICRC3Value =
  | { Int: bigint }
  | { Map: Array<[string, ICRC3Value]> }
  | { Nat: bigint }
  | { Blob: Uint8Array | number[] }
  | { Text: string }
  | { Array: Array<ICRC3Value> };
export type Icrc21Error =
  | {
      GenericError: { description: string; error_code: bigint };
    }
  | { InsufficientPayment: ErrorInfo }
  | { UnsupportedCanisterCall: ErrorInfo }
  | { ConsentMessageUnavailable: ErrorInfo };
export interface InitApprovalsArg {
  max_approvals_per_token_or_collection: [] | [bigint];
  max_revoke_approvals: [] | [bigint];
}
export interface InitArgs {
  permissions: PermissionManager;
  supply_cap: [] | [bigint];
  tx_window: [] | [bigint];
  test_mode: boolean;
  default_take_value: [] | [bigint];
  max_canister_storage_threshold: [] | [bigint];
  logo: [] | [string];
  permitted_drift: [] | [bigint];
  name: string;
  description: [] | [string];
  version: BuildVersion;
  max_take_value: [] | [bigint];
  max_update_batch_size: [] | [bigint];
  max_query_batch_size: [] | [bigint];
  commit_hash: string;
  max_memo_size: [] | [bigint];
  atomic_batch_transfers: [] | [boolean];
  collection_metadata: Array<[string, CustomValue]>;
  symbol: string;
  approval_init: InitApprovalsArg;
}
export type InitUploadError =
  | { NotEnoughStorage: null }
  | { FileAlreadyExists: null }
  | { InvalidChunkSize: null };
export interface IsApprovedArg {
  token_id: bigint;
  from_subaccount: [] | [Uint8Array | number[]];
  spender: Account;
}
export interface LineDisplayPage {
  lines: Array<string>;
}
export type MintError =
  | { TokenAlreadyExists: null }
  | { StorageCanisterError: string }
  | { ExceedMaxAllowedSupplyCap: null }
  | { InvalidMemo: null }
  | { ConcurrentManagementCall: null };
export interface MintRequest {
  metadata: Array<[string, ICRC3Value]>;
  memo: [] | [Uint8Array | number[]];
  token_owner: Account;
}
export type Permission =
  | { UpdateMetadata: null }
  | { Minting: null }
  | { UpdateCollectionMetadata: null }
  | { UpdateUploads: null }
  | { ManageAuthorities: null }
  | { ReadUploads: null };
export interface PermissionManager {
  user_permissions: Array<[Principal, Array<Permission>]>;
}
export type Result = { Ok: null } | { Err: BurnNftError };
export type Result_1 = { Ok: {} } | { Err: CancelUploadError };
export type Result_10 =
  | { Ok: Array<[] | [ApproveTokenResult]> }
  | { Err: ApproveTokenError };
export type Result_11 =
  | {
      Ok: Array<[] | [RevokeCollectionApprovalResult]>;
    }
  | { Err: RevokeCollectionApprovalError };
export type Result_12 =
  | { Ok: Array<[] | [RevokeTokenApprovalResponse]> }
  | { Err: RevokeTokenApprovalError };
export type Result_13 =
  | { Ok: Array<[] | [TransferFromResult]> }
  | { Err: TransferFromError };
export type Result_14 = { Ok: bigint } | { Err: TransferError };
export type Result_15 = { Ok: {} } | { Err: InitUploadError };
export type Result_16 = { Ok: bigint } | { Err: MintError };
export type Result_17 = { Ok: {} } | { Err: StoreChunkError };
export type Result_18 = { Ok: null } | { Err: UpdateCollectionMetadataError };
export type Result_19 = { Ok: bigint } | { Err: UpdateNftMetadataError };
export type Result_2 =
  | { Ok: FinalizeUploadResp }
  | { Err: FinalizeUploadError };
export type Result_3 =
  | { Ok: Array<[string, UploadState]> }
  | { Err: GetAllUploadsError };
export type Result_4 = { Ok: UploadState } | { Err: GetUploadStatusError };
export type Result_5 =
  | { Ok: Array<Permission> }
  | { Err: GetUserPermissionsError };
export type Result_6 = { Ok: null } | { Err: GrantPermissionError };
export type Result_7 = { Ok: boolean } | { Err: HasPermissionError };
export type Result_8 = { Ok: ConsentInfo } | { Err: Icrc21Error };
export type Result_9 =
  | { Ok: Array<[] | [ApproveCollectionResult]> }
  | { Err: ApproveCollectionError };
export interface RevokeCollectionApprovalArg {
  memo: [] | [Uint8Array | number[]];
  from_subaccount: [] | [Uint8Array | number[]];
  created_at_time: [] | [bigint];
  spender: [] | [Account];
}
export type RevokeCollectionApprovalError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { ApprovalDoesNotExist: null }
  | { GenericBatchError: { message: string; error_code: bigint } }
  | { TooOld: null };
export type RevokeCollectionApprovalResult =
  | { Ok: bigint }
  | { Err: RevokeCollectionApprovalError };
export interface RevokeTokenApprovalArg {
  token_id: bigint;
  memo: [] | [Uint8Array | number[]];
  from_subaccount: [] | [Uint8Array | number[]];
  created_at_time: [] | [bigint];
  spender: [] | [Account];
}
export type RevokeTokenApprovalError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { NonExistingTokenId: null }
  | { Unauthorized: null }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { ApprovalDoesNotExist: null }
  | { GenericBatchError: { message: string; error_code: bigint } }
  | { TooOld: null };
export type RevokeTokenApprovalResponse =
  | { Ok: bigint }
  | { Err: RevokeTokenApprovalError };
export type StoreChunkError =
  | { InvalidFileHash: null }
  | { InvalidFilePath: null }
  | { InvalidFileSize: null }
  | { InvalidChunkId: null }
  | { UploadNotInitialized: null }
  | { InvalidChunkData: null }
  | { InvalidFileFormat: null }
  | { UploadAlreadyFinalized: null };
export interface SupportedBlockType {
  url: string;
  block_type: string;
}
export interface SupportedStandard {
  url: string;
  name: string;
}
export interface TransferArg {
  to: Account;
  token_id: bigint;
  memo: [] | [Uint8Array | number[]];
  from_subaccount: [] | [Uint8Array | number[]];
  created_at_time: [] | [bigint];
}
export type TransferError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { Duplicate: { duplicate_of: bigint } }
  | { NonExistingTokenId: null }
  | { Unauthorized: null }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { InvalidRecipient: null }
  | { GenericBatchError: { message: string; error_code: bigint } }
  | { TooOld: null };
export interface TransferFromArg {
  to: Account;
  spender_subaccount: [] | [Uint8Array | number[]];
  token_id: bigint;
  from: Account;
  memo: [] | [Uint8Array | number[]];
  created_at_time: [] | [bigint];
}
export type TransferFromError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { Duplicate: { duplicate_of: bigint } }
  | { NonExistingTokenId: null }
  | { Unauthorized: null }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { InvalidRecipient: null }
  | { GenericBatchError: { message: string; error_code: bigint } }
  | { TooOld: null };
export type TransferFromResult = { Ok: bigint } | { Err: TransferFromError };
export type UpdateCollectionMetadataError =
  | {
      StorageCanisterError: string;
    }
  | { ConcurrentManagementCall: null };
export type UpdateNftMetadataError =
  | { StorageCanisterError: string }
  | { TokenDoesNotExist: null }
  | { ConcurrentManagementCall: null };
export interface UpgradeArgs {
  version: BuildVersion;
  commit_hash: string;
}
export type UploadState =
  | { Init: null }
  | { Finalized: null }
  | { InProgress: null };
export interface _SERVICE {
  burn_nft: ActorMethod<[bigint], Result>;
  cancel_upload: ActorMethod<[Args], Result_1>;
  finalize_upload: ActorMethod<[Args], Result_2>;
  get_all_storage_subcanisters: ActorMethod<[], Array<Principal>>;
  get_all_uploads: ActorMethod<[[] | [bigint], [] | [bigint]], Result_3>;
  get_upload_status: ActorMethod<[string], Result_4>;
  get_user_permissions: ActorMethod<[Args_1], Result_5>;
  grant_permission: ActorMethod<[Args_2], Result_6>;
  has_permission: ActorMethod<[Args_3], Result_7>;
  icrc10_supported_standards: ActorMethod<[], Array<SupportedStandard>>;
  icrc21_canister_call_consent_message: ActorMethod<
    [ConsentMessageRequest],
    Result_8
  >;
  icrc37_approve_collection: ActorMethod<
    [Array<ApproveCollectionArg>],
    Result_9
  >;
  icrc37_approve_tokens: ActorMethod<[Array<ApproveTokenArg>], Result_10>;
  icrc37_get_collection_approvals: ActorMethod<
    [Account, [] | [ApproveCollectionArg], [] | [bigint]],
    Array<ApproveCollectionArg>
  >;
  icrc37_get_token_approvals: ActorMethod<
    [bigint, [] | [ApproveTokenArg], [] | [bigint]],
    Array<ApproveTokenArg>
  >;
  icrc37_is_approved: ActorMethod<[Array<IsApprovedArg>], Array<boolean>>;
  icrc37_max_approvals_per_token_or_collection: ActorMethod<[], [] | [bigint]>;
  icrc37_max_revoke_approvals: ActorMethod<[], [] | [bigint]>;
  icrc37_revoke_collection_approvals: ActorMethod<
    [Array<RevokeCollectionApprovalArg>],
    Result_11
  >;
  icrc37_revoke_token_approvals: ActorMethod<
    [Array<RevokeTokenApprovalArg>],
    Result_12
  >;
  icrc37_transfer_from: ActorMethod<[Array<TransferFromArg>], Result_13>;
  icrc3_get_archives: ActorMethod<[null], Array<ICRC3ArchiveInfo>>;
  icrc3_get_blocks: ActorMethod<[Array<GetBlocksRequest>], GetBlocksResult>;
  icrc3_get_properties: ActorMethod<[null], ICRC3Properties>;
  icrc3_get_tip_certificate: ActorMethod<[null], ICRC3DataCertificate>;
  icrc3_supported_block_types: ActorMethod<[null], Array<SupportedBlockType>>;
  icrc7_atomic_batch_transfers: ActorMethod<[], [] | [boolean]>;
  icrc7_balance_of: ActorMethod<[Array<Account>], Array<bigint>>;
  icrc7_collection_metadata: ActorMethod<[], Array<[string, ICRC3Value]>>;
  icrc7_default_take_value: ActorMethod<[], [] | [bigint]>;
  icrc7_description: ActorMethod<[], [] | [string]>;
  icrc7_logo: ActorMethod<[], [] | [string]>;
  icrc7_max_memo_size: ActorMethod<[], [] | [bigint]>;
  icrc7_max_query_batch_size: ActorMethod<[], [] | [bigint]>;
  icrc7_max_take_value: ActorMethod<[], [] | [bigint]>;
  icrc7_max_update_batch_size: ActorMethod<[], [] | [bigint]>;
  icrc7_name: ActorMethod<[], string>;
  icrc7_owner_of: ActorMethod<[Array<bigint>], Array<[] | [Account]>>;
  icrc7_permitted_drift: ActorMethod<[], [] | [bigint]>;
  icrc7_supply_cap: ActorMethod<[], [] | [bigint]>;
  icrc7_symbol: ActorMethod<[], string>;
  icrc7_token_metadata: ActorMethod<
    [Array<bigint>],
    Array<[] | [Array<[string, ICRC3Value]>]>
  >;
  icrc7_tokens: ActorMethod<[[] | [bigint], [] | [bigint]], Array<bigint>>;
  icrc7_tokens_of: ActorMethod<
    [Account, [] | [bigint], [] | [bigint]],
    Array<bigint>
  >;
  icrc7_total_supply: ActorMethod<[], bigint>;
  icrc7_transfer: ActorMethod<[Array<TransferArg>], Array<[] | [Result_14]>>;
  icrc7_tx_window: ActorMethod<[], [] | [bigint]>;
  init_upload: ActorMethod<[Args_4], Result_15>;
  mint: ActorMethod<[Args_5], Result_16>;
  revoke_permission: ActorMethod<[Args_3], Result_6>;
  store_chunk: ActorMethod<[Args_6], Result_17>;
  update_collection_metadata: ActorMethod<[Args_7], Result_18>;
  update_nft_metadata: ActorMethod<[Args_8], Result_19>;
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
