export const idlFactory = ({ IDL }) => {
  const GetBlocksResult = IDL.Rec();
  const ICRC3Value = IDL.Rec();
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const Duration = IDL.Record({ secs: IDL.Nat64, nanos: IDL.Nat32 });
  const ICRC3Properties = IDL.Record({
    max_blocks_per_response: IDL.Nat,
    initial_cycles: IDL.Nat,
    tx_window: Duration,
    max_transactions_to_purge: IDL.Nat,
    max_memory_size_bytes: IDL.Nat,
    ttl_for_non_archived_transactions: Duration,
    max_transactions_in_window: IDL.Nat,
    max_unarchived_transactions: IDL.Nat,
    reserved_cycles: IDL.Nat,
  });
  const SupportedBlockType = IDL.Record({
    url: IDL.Text,
    block_type: IDL.Text,
  });
  const ICRC3Config = IDL.Record({
    constants: ICRC3Properties,
    supported_blocks: IDL.Vec(SupportedBlockType),
  });
  const GeneralFractionalizationConfig = IDL.Record({
    division: IDL.Nat64,
    ledger_id: IDL.Principal,
    swap_fee: IDL.Nat,
  });
  const CustomFractionalizationConfig = IDL.Record({
    per_token_config: IDL.Vec(
      IDL.Tuple(IDL.Nat, GeneralFractionalizationConfig)
    ),
  });
  const FractionalizationConfig = IDL.Variant({
    Custom: CustomFractionalizationConfig,
    General: GeneralFractionalizationConfig,
  });
  const SwapCanisterConfig = IDL.Record({
    icrc7_canister_id: IDL.Principal,
    fractionalization_config: FractionalizationConfig,
  });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    icrc3_config: ICRC3Config,
    commit_hash: IDL.Text,
    swap_configs: IDL.Vec(SwapCanisterConfig),
  });
  const Args_1 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  const Nft = IDL.Record({ id: IDL.Nat, canister_id: IDL.Principal });
  const SwapStatus = IDL.Variant({
    Burned: IDL.Null,
    Failed: IDL.Text,
    NftTransferredFrom: IDL.Null,
    Init: IDL.Null,
    NftTransferred: IDL.Null,
    Complete: IDL.Null,
    BurnFailed: IDL.Text,
    ReimburseFailed: IDL.Text,
    Minted: IDL.Null,
    NftTransferFailed: IDL.Text,
    NftTransferFromFailed: IDL.Text,
    Reimbursed: IDL.Null,
    MintFailed: IDL.Text,
  });
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const SwapType = IDL.Variant({ Forward: IDL.Null, Reverse: IDL.Null });
  const SwapInfo = IDL.Record({
    nft: Nft,
    status: SwapStatus,
    created_at: IDL.Nat64,
    tokens_amount: GeneralFractionalizationConfig,
    user_account: Account,
    index: IDL.Nat,
    swap_type: SwapType,
  });
  const GeneralError = IDL.Variant({
    InvalidConfig: IDL.Text,
    InvalidNftCanister: IDL.Text,
    TransactionAddError: IDL.Text,
    TransferError: IDL.Text,
    UserIsNotNftOwner: IDL.Text,
    AlreadyProcessing: IDL.Text,
    TransactionPreparationError: IDL.Text,
    InvalidPrincipal: IDL.Text,
    NotAuthorized: IDL.Text,
    EmptyArgs: IDL.Text,
    CallError: IDL.Text,
    CanisterIsNotNftOwner: IDL.Text,
    ConfigNotFound: IDL.Text,
    InvalidPercentage: IDL.Text,
  });
  const Result = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Vec(IDL.Nat))),
    Err: GeneralError,
  });
  const Args = IDL.Record({
    principal: IDL.Opt(IDL.Principal),
    canister_id: IDL.Principal,
  });
  const Result_1 = IDL.Variant({
    Ok: IDL.Vec(IDL.Nat),
    Err: GeneralError,
  });
  const SupportedStandard = IDL.Record({ url: IDL.Text, name: IDL.Text });
  const icrc21_consent_message_metadata = IDL.Record({
    utc_offset_minutes: IDL.Opt(IDL.Int16),
    language: IDL.Text,
  });
  const icrc21_device_spec = IDL.Variant({
    GenericDisplay: IDL.Null,
    FieldsDisplay: IDL.Null,
  });
  const icrc21_consent_message_spec = IDL.Record({
    metadata: icrc21_consent_message_metadata,
    device_spec: IDL.Opt(icrc21_device_spec),
  });
  const icrc21_consent_message_request = IDL.Record({
    arg: IDL.Vec(IDL.Nat8),
    method: IDL.Text,
    user_preferences: icrc21_consent_message_spec,
  });
  const icrc21_field_display_message = IDL.Record({
    fields: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    intent: IDL.Text,
  });
  const icrc21_consent_message = IDL.Record({
    generic_display_message: IDL.Text,
    fields_display_message: icrc21_field_display_message,
  });
  const icrc21_consent_info = IDL.Record({
    metadata: icrc21_consent_message_metadata,
    consent_message: icrc21_consent_message,
  });
  const icrc21_error_info = IDL.Record({ description: IDL.Text });
  const icrc21_generic_error = IDL.Record({
    description: IDL.Text,
    error_code: IDL.Nat64,
  });
  const icrc21_error = IDL.Variant({
    GenericError: icrc21_error_info,
    InsufficientPayment: icrc21_generic_error,
    UnsupportedCanisterCall: icrc21_error_info,
    ConsentMessageUnavailable: icrc21_error_info,
  });
  const icrc21_consent_message_response = IDL.Variant({
    Ok: icrc21_consent_info,
    Err: icrc21_error,
  });
  const Icrc28TrustedOriginsResponse = IDL.Record({
    trusted_origins: IDL.Vec(IDL.Text),
  });
  const ICRC3ArchiveInfo = IDL.Record({
    end: IDL.Nat,
    canister_id: IDL.Principal,
    start: IDL.Nat,
  });
  const GetBlocksRequest = IDL.Record({
    start: IDL.Nat,
    length: IDL.Nat,
  });
  ICRC3Value.fill(
    IDL.Variant({
      Int: IDL.Int,
      Map: IDL.Vec(IDL.Tuple(IDL.Text, ICRC3Value)),
      Nat: IDL.Nat,
      Blob: IDL.Vec(IDL.Nat8),
      Text: IDL.Text,
      Array: IDL.Vec(ICRC3Value),
    })
  );
  const BlockWithId = IDL.Record({ id: IDL.Nat, block: ICRC3Value });
  const ArchivedBlocks = IDL.Record({
    args: IDL.Vec(GetBlocksRequest),
    callback: IDL.Func(
      [IDL.Vec(GetBlocksRequest)],
      [GetBlocksResult],
      ["query"]
    ),
  });
  GetBlocksResult.fill(
    IDL.Record({
      log_length: IDL.Nat,
      blocks: IDL.Vec(BlockWithId),
      archived_blocks: IDL.Vec(ArchivedBlocks),
    })
  );
  const ICRC3DataCertificate = IDL.Record({
    certificate: IDL.Vec(IDL.Nat8),
    hash_tree: IDL.Vec(IDL.Nat8),
  });
  const Response = IDL.Variant({
    Success: IDL.Null,
    InternalError: IDL.Text,
  });
  const SwapNftForTokensErrors = IDL.Variant({
    Limit: IDL.Text,
    GeneralError: GeneralError,
    Retry: IDL.Tuple(IDL.Nat64, IDL.Text),
    CantBeAnonymous: IDL.Text,
  });
  const Result_2 = IDL.Variant({
    Ok: IDL.Vec(IDL.Nat),
    Err: SwapNftForTokensErrors,
  });
  const SwapTokensForNftErrors = IDL.Variant({
    Limit: IDL.Text,
    GeneralError: GeneralError,
    Retry: IDL.Tuple(IDL.Nat64, IDL.Text),
    NotOwnedBySwapCanister: IDL.Null,
    SwapCreationError: IDL.Null,
  });
  const Result_3 = IDL.Variant({
    Ok: IDL.Vec(IDL.Nat),
    Err: SwapTokensForNftErrors,
  });
  return IDL.Service({
    commit: IDL.Func([], [], []),
    get_active_swap_ids_by_user: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Vec(IDL.Nat)],
      ["query"]
    ),
    get_active_swaps: IDL.Func(
      [IDL.Null],
      [IDL.Vec(IDL.Tuple(IDL.Nat, SwapInfo))],
      ["query"]
    ),
    get_active_swaps_by_ids: IDL.Func(
      [IDL.Vec(IDL.Nat)],
      [IDL.Vec(IDL.Tuple(IDL.Nat, SwapInfo))],
      ["query"]
    ),
    get_active_swaps_by_user: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Vec(IDL.Tuple(IDL.Nat, SwapInfo))],
      ["query"]
    ),
    get_available_nfts: IDL.Func([IDL.Opt(IDL.Principal)], [Result], []),
    get_available_nfts_for_canister: IDL.Func([Args], [Result_1], []),
    get_swap_configs: IDL.Func(
      [IDL.Null],
      [IDL.Vec(SwapCanisterConfig)],
      ["query"]
    ),
    icrc10_supported_standards: IDL.Func(
      [],
      [IDL.Vec(SupportedStandard)],
      ["query"]
    ),
    icrc21_canister_call_consent_message: IDL.Func(
      [icrc21_consent_message_request],
      [icrc21_consent_message_response],
      ["query"]
    ),
    icrc28_trusted_origins: IDL.Func([], [Icrc28TrustedOriginsResponse], []),
    icrc3_get_archives: IDL.Func(
      [IDL.Null],
      [IDL.Vec(ICRC3ArchiveInfo)],
      ["query"]
    ),
    icrc3_get_blocks: IDL.Func(
      [IDL.Vec(GetBlocksRequest)],
      [GetBlocksResult],
      ["query"]
    ),
    icrc3_get_properties: IDL.Func([IDL.Null], [ICRC3Properties], ["query"]),
    icrc3_get_tip_certificate: IDL.Func(
      [IDL.Null],
      [ICRC3DataCertificate],
      ["query"]
    ),
    icrc3_supported_block_types: IDL.Func(
      [IDL.Null],
      [IDL.Vec(SupportedBlockType)],
      ["query"]
    ),
    set_buyback_canister: IDL.Func([IDL.Opt(Account)], [Response], []),
    swap_nft_for_tokens: IDL.Func([IDL.Vec(Nft)], [Result_2], []),
    swap_tokens_for_nft: IDL.Func([IDL.Vec(Nft)], [Result_3], []),
  });
};
export const init = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const Duration = IDL.Record({ secs: IDL.Nat64, nanos: IDL.Nat32 });
  const ICRC3Properties = IDL.Record({
    max_blocks_per_response: IDL.Nat,
    initial_cycles: IDL.Nat,
    tx_window: Duration,
    max_transactions_to_purge: IDL.Nat,
    max_memory_size_bytes: IDL.Nat,
    ttl_for_non_archived_transactions: Duration,
    max_transactions_in_window: IDL.Nat,
    max_unarchived_transactions: IDL.Nat,
    reserved_cycles: IDL.Nat,
  });
  const SupportedBlockType = IDL.Record({
    url: IDL.Text,
    block_type: IDL.Text,
  });
  const ICRC3Config = IDL.Record({
    constants: ICRC3Properties,
    supported_blocks: IDL.Vec(SupportedBlockType),
  });
  const GeneralFractionalizationConfig = IDL.Record({
    division: IDL.Nat64,
    ledger_id: IDL.Principal,
    swap_fee: IDL.Nat,
  });
  const CustomFractionalizationConfig = IDL.Record({
    per_token_config: IDL.Vec(
      IDL.Tuple(IDL.Nat, GeneralFractionalizationConfig)
    ),
  });
  const FractionalizationConfig = IDL.Variant({
    Custom: CustomFractionalizationConfig,
    General: GeneralFractionalizationConfig,
  });
  const SwapCanisterConfig = IDL.Record({
    icrc7_canister_id: IDL.Principal,
    fractionalization_config: FractionalizationConfig,
  });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    icrc3_config: ICRC3Config,
    commit_hash: IDL.Text,
    swap_configs: IDL.Vec(SwapCanisterConfig),
  });
  const Args_1 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  return [Args_1];
};
