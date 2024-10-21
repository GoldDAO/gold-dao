export const idlFactory = ({ IDL }) => {
  const SwapStatusForward = IDL.Rec();
  const SwapStatusReverse = IDL.Rec();
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const NftCanisterConf = IDL.Record({ grams: IDL.Nat16 });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    ogy_ledger_id: IDL.Principal,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    gldnft_canisters: IDL.Vec(IDL.Tuple(IDL.Principal, NftCanisterConf)),
    gldt_ledger_id: IDL.Principal,
    commit_hash: IDL.Text,
  });
  const Args_3 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  const DepositRecoveryError = IDL.Variant({
    CantRecover: IDL.Text,
    CallError: IDL.Text,
  });
  const BidFailError = IDL.Variant({
    UnexpectedError: IDL.Text,
    CallError: IDL.Text,
    TransferFailed: IDL.Text,
  });
  const ImpossibleErrorReason = IDL.Variant({
    AmountNotFound: IDL.Null,
    NFTResponseInvalid: IDL.Null,
    PrincipalNotFound: IDL.Null,
  });
  const NotificationError = IDL.Variant({
    InvalidSaleSubaccount: IDL.Null,
    InvalidTokenSpec: IDL.Null,
    TimeoutInvalid: IDL.Text,
    InvalidEscrowSubaccount: IDL.Text,
    TooManyPrincipalsInAllowList: IDL.Null,
    OrigynStringIdDoesNotMatch: IDL.Text,
    SellerIsNotPrincipalOrAccount: IDL.Text,
    SellerAndReceiverDoesNotMatch: IDL.Text,
    InvalidCustomAskFeature: IDL.Null,
    InvalidTokenAmount: IDL.Null,
    InvalidPricingConfig: IDL.Null,
    CollectionDoesNotMatch: IDL.Text,
    AllowListDoesNotContainCorrectPrincipal: IDL.Null,
  });
  const TransferError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    TemporarilyUnavailable: IDL.Null,
    BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }),
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    BadFee: IDL.Record({ expected_fee: IDL.Nat }),
    CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    TooOld: IDL.Null,
    InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
  });
  const TransferFromError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    TemporarilyUnavailable: IDL.Null,
    InsufficientAllowance: IDL.Record({ allowance: IDL.Nat }),
    BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }),
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    BadFee: IDL.Record({ expected_fee: IDL.Nat }),
    CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    TooOld: IDL.Null,
    InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
  });
  const TransferFailReason = IDL.Variant({
    TransferError: TransferError,
    TransferFromError: TransferFromError,
    CallError: IDL.Text,
  });
  const MintError = IDL.Variant({
    UnexpectedError: ImpossibleErrorReason,
    TransferFailed: TransferFailReason,
  });
  const SwapErrorForward = IDL.Variant({
    DepositRecoveryFailed: DepositRecoveryError,
    BidFailed: BidFailError,
    UnexpectedError: ImpossibleErrorReason,
    NotificationFailed: NotificationError,
    MintFailed: MintError,
    Expired: IDL.Null,
  });
  SwapStatusForward.fill(
    IDL.Variant({
      DepositRecoveryFailed: IDL.Tuple(SwapStatusForward, DepositRecoveryError),
      Failed: SwapErrorForward,
      DepositRecoveryInProgress: SwapStatusForward,
      BidInProgress: IDL.Null,
      Init: IDL.Null,
      MintRequest: IDL.Null,
      DepositRecoveryRequest: SwapStatusForward,
      Complete: IDL.Null,
      BidFail: BidFailError,
      BidRequest: IDL.Null,
      NotificationFailed: NotificationError,
      MintInProgress: IDL.Null,
      BurnFeesInProgress: IDL.Null,
      BurnFeesRequest: IDL.Null,
      BurnFeesFailed: MintError,
      NotificationInProgress: IDL.Null,
      MintFailed: MintError,
    })
  );
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const GldtNumTokens = IDL.Record({
    value_with_fee: IDL.Nat,
    value: IDL.Nat,
  });
  const SwapDetailForward = IDL.Record({
    nft_id: IDL.Nat,
    status: SwapStatusForward,
    escrow_sub_account: IDL.Vec(IDL.Nat8),
    nft_id_string: IDL.Text,
    created_at: IDL.Nat64,
    gldt_receiver: Account,
    tokens_to_mint: GldtNumTokens,
    nft_canister: IDL.Principal,
    index: IDL.Nat,
    sale_id: IDL.Text,
  });
  const FeeTransferError = IDL.Variant({
    TransferError: TransferError,
    CallError: IDL.Text,
  });
  const ApproveError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    TemporarilyUnavailable: IDL.Null,
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    BadFee: IDL.Record({ expected_fee: IDL.Nat }),
    AllowanceChanged: IDL.Record({ current_allowance: IDL.Nat }),
    CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    TooOld: IDL.Null,
    Expired: IDL.Record({ ledger_time: IDL.Nat64 }),
    InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
  });
  const EscrowError = IDL.Variant({
    ApproveError: ApproveError,
    UnexpectedError: ImpossibleErrorReason,
    TransferFailed: TransferFailReason,
    RequestFailed: IDL.Text,
  });
  const LockError = IDL.Variant({
    NftAlreadyLocked: IDL.Vec(IDL.Nat),
    UnexpectedError: IDL.Record({}),
    NftNotLocked: IDL.Null,
  });
  const NftValidationError = IDL.Variant({
    WeightParseError: IDL.Null,
    CanisterInvalid: IDL.Null,
    CantGetOrigynID: IDL.Text,
    CantVerifySwapCanisterOwnsNft: IDL.Null,
    InvalidGldtTokensFromWeight: IDL.Null,
    InvalidNftWeight: IDL.Null,
    NotOwnedBySwapCanister: IDL.Null,
  });
  const BurnError = IDL.Variant({ CallError: IDL.Text });
  const NftTransferError = IDL.Variant({
    FailedToGetOgyFeeAllowance: IDL.Text,
    ApprovalError: ApproveError,
    ApprovalCallError: IDL.Text,
    InvalidFee: IDL.Text,
    UnexpectedError: ImpossibleErrorReason,
    CallError: IDL.Text,
    TransferFailed: IDL.Text,
  });
  const SwapErrorReverse = IDL.Variant({
    FeeTransferFailed: FeeTransferError,
    EscrowFailed: EscrowError,
    LockFailed: LockError,
    Refunded: SwapStatusReverse,
    NftValidationFailed: IDL.Vec(NftValidationError),
    BurnFailed: BurnError,
    NftTransferFailed: NftTransferError,
  });
  const RefundError = IDL.Variant({
    CallError: IDL.Text,
    TransferFailed: TransferError,
  });
  SwapStatusReverse.fill(
    IDL.Variant({
      NftTransferRequestInProgress: IDL.Null,
      FeeTransferFailed: FeeTransferError,
      Failed: SwapErrorReverse,
      EscrowFailed: EscrowError,
      Init: IDL.Null,
      BurnRequestInProgress: IDL.Null,
      EscrowRequestInProgress: IDL.Null,
      Complete: IDL.Null,
      BurnFailed: BurnError,
      RefundRequestInProgress: IDL.Null,
      RefundRequest: IDL.Null,
      NftTransferRequest: IDL.Null,
      FeeTransferRequestInProgress: IDL.Null,
      NftTransferFailed: NftTransferError,
      BurnRequest: IDL.Null,
      FeeTransferRequest: IDL.Null,
      RefundFailed: RefundError,
      EscrowRequest: IDL.Null,
    })
  );
  const SwapDetailReverse = IDL.Record({
    nft_id: IDL.Nat,
    status: SwapStatusReverse,
    tokens_to_receive: GldtNumTokens,
    nft_id_string: IDL.Text,
    user: IDL.Principal,
    created_at: IDL.Nat64,
    swap_fee: IDL.Nat,
    nft_canister: IDL.Principal,
    index: IDL.Nat,
    transfer_fees: IDL.Nat,
  });
  const SwapInfo = IDL.Variant({
    Forward: SwapDetailForward,
    Reverse: SwapDetailReverse,
  });
  const ArchiveCanister = IDL.Record({
    canister_id: IDL.Principal,
    end_index: IDL.Opt(IDL.Nat),
    start_index: IDL.Nat,
  });
  const Args = IDL.Record({ page: IDL.Nat64, limit: IDL.Nat64 });
  const GetHistoricSwapsError = IDL.Variant({ LimitTooLarge: IDL.Text });
  const Result = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo)),
    Err: GetHistoricSwapsError,
  });
  const Args_1 = IDL.Record({
    page: IDL.Nat64,
    user: IDL.Principal,
    limit: IDL.Nat64,
  });
  const GetHistoricSwapsByUserError = IDL.Variant({
    LimitTooLarge: IDL.Text,
    LimitTooSmall: IDL.Text,
    QueryCanisterError: IDL.Text,
  });
  const Result_1 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo)),
    Err: GetHistoricSwapsByUserError,
  });
  const RemoveIntentToSwapError = IDL.Variant({
    InvalidSwapType: IDL.Text,
    InvalidUser: IDL.Null,
    SwapNotFound: IDL.Null,
    InProgress: IDL.Null,
  });
  const Result_2 = IDL.Variant({
    Ok: IDL.Null,
    Err: RemoveIntentToSwapError,
  });
  const NftInvalidError = IDL.Variant({
    InvalidNftOwner: IDL.Text,
    AlreadyLocked: IDL.Null,
    CantGetOrigynID: IDL.Text,
    InvalidNFTCollectionPrincipal: IDL.Null,
    InvalidTokenAmount: IDL.Null,
    CantGetNatIdOfNft: IDL.Null,
  });
  const ArchiveDownReason = IDL.Variant({
    UpgradingArchivesFailed: IDL.Text,
    NoArchiveCanisters: IDL.Text,
    Upgrading: IDL.Null,
    InitializingFirstArchiveFailed: IDL.Text,
    ActiveSwapCapacityFull: IDL.Null,
    LowOrigynToken: IDL.Text,
  });
  const ServiceDownReason = IDL.Variant({
    ArchiveRelated: ArchiveDownReason,
    Initializing: IDL.Null,
    ActiveSwapCapacityFull: IDL.Null,
    LowOrigynToken: IDL.Text,
  });
  const SwapNftForTokensErrors = IDL.Variant({
    Limit: IDL.Text,
    ContainsDuplicates: IDL.Text,
    ContainsInvalidNftCanister: IDL.Text,
    NftValidationErrors: IDL.Tuple(
      IDL.Vec(IDL.Nat),
      IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Vec(NftInvalidError)))
    ),
    ServiceDown: ServiceDownReason,
  });
  const Result_3 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat)),
    Err: SwapNftForTokensErrors,
  });
  const Args_2 = IDL.Record({
    nft_id: IDL.Nat,
    nft_canister_id: IDL.Principal,
  });
  const GetNftMetaDetailErrorReason = IDL.Variant({
    CantFindNFT: IDL.Text,
    NoMetaDetails: IDL.Null,
    UnexpectedError: IDL.Text,
  });
  const SwapTokensForNftRequestErrors = IDL.Variant({
    GetNftMetaDetailError: GetNftMetaDetailErrorReason,
    CantForgeSwapId: IDL.Null,
    NftLocked: LockError,
    NftValidationErrors: IDL.Vec(NftValidationError),
    NotOwnedBySwapCanister: IDL.Null,
    ServiceDown: ServiceDownReason,
    SwapCreationError: IDL.Null,
  });
  const Result_4 = IDL.Variant({
    Ok: IDL.Tuple(IDL.Nat, IDL.Nat),
    Err: SwapTokensForNftRequestErrors,
  });
  return IDL.Service({
    get_active_swap_ids_by_user: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat))],
      ["query"]
    ),
    get_active_swaps: IDL.Func(
      [IDL.Null],
      [IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo))],
      ["query"]
    ),
    get_active_swaps_by_user: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo))],
      ["query"]
    ),
    get_archive_canisters: IDL.Func(
      [IDL.Null],
      [IDL.Vec(ArchiveCanister)],
      ["query"]
    ),
    get_historic_swaps: IDL.Func([Args], [Result], []),
    get_historic_swaps_by_user: IDL.Func([Args_1], [Result_1], []),
    get_history_total: IDL.Func([IDL.Opt(IDL.Principal)], [IDL.Nat], []),
    get_swap: IDL.Func(
      [IDL.Tuple(IDL.Nat, IDL.Nat)],
      [IDL.Opt(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo))],
      []
    ),
    remove_intent_to_swap: IDL.Func(
      [IDL.Tuple(IDL.Nat, IDL.Nat)],
      [Result_2],
      []
    ),
    swap_nft_for_tokens: IDL.Func(
      [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Principal))],
      [Result_3],
      []
    ),
    swap_tokens_for_nft: IDL.Func([Args_2], [Result_4], []),
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
  const NftCanisterConf = IDL.Record({ grams: IDL.Nat16 });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    ogy_ledger_id: IDL.Principal,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    gldnft_canisters: IDL.Vec(IDL.Tuple(IDL.Principal, NftCanisterConf)),
    gldt_ledger_id: IDL.Principal,
    commit_hash: IDL.Text,
  });
  const Args_3 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  return [Args_3];
};
