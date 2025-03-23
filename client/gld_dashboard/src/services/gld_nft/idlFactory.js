export const idlFactory = ({ IDL }) => {
  const ArchivedTransactionResponse = IDL.Rec();
  const CandyShared = IDL.Rec();
  const ManageSaleResponse = IDL.Rec();
  const UpdateShared = IDL.Rec();
  const Value = IDL.Rec();
  const Value__1 = IDL.Rec();
  const Vec = IDL.Rec();
  const PropertyShared = IDL.Record({
    value: CandyShared,
    name: IDL.Text,
    immutable: IDL.Bool,
  });
  CandyShared.fill(
    IDL.Variant({
      Int: IDL.Int,
      Map: IDL.Vec(IDL.Tuple(CandyShared, CandyShared)),
      Nat: IDL.Nat,
      Set: IDL.Vec(CandyShared),
      Nat16: IDL.Nat16,
      Nat32: IDL.Nat32,
      Nat64: IDL.Nat64,
      Blob: IDL.Vec(IDL.Nat8),
      Bool: IDL.Bool,
      Int8: IDL.Int8,
      Ints: IDL.Vec(IDL.Int),
      Nat8: IDL.Nat8,
      Nats: IDL.Vec(IDL.Nat),
      Text: IDL.Text,
      Bytes: IDL.Vec(IDL.Nat8),
      Int16: IDL.Int16,
      Int32: IDL.Int32,
      Int64: IDL.Int64,
      Option: IDL.Opt(CandyShared),
      Floats: IDL.Vec(IDL.Float64),
      Float: IDL.Float64,
      Principal: IDL.Principal,
      Array: IDL.Vec(CandyShared),
      Class: IDL.Vec(PropertyShared),
    })
  );
  const Account__1 = IDL.Variant({
    account_id: IDL.Text,
    principal: IDL.Principal,
    extensible: CandyShared,
    account: IDL.Record({
      owner: IDL.Principal,
      sub_account: IDL.Opt(IDL.Vec(IDL.Nat8)),
    }),
  });
  const ICTokenSpec__1 = IDL.Record({
    id: IDL.Opt(IDL.Nat),
    fee: IDL.Opt(IDL.Nat),
    decimals: IDL.Nat,
    canister: IDL.Principal,
    standard: IDL.Variant({
      ICRC1: IDL.Null,
      EXTFungible: IDL.Null,
      DIP20: IDL.Null,
      Other: CandyShared,
      Ledger: IDL.Null,
    }),
    symbol: IDL.Text,
  });
  const TokenSpec__1 = IDL.Variant({
    ic: ICTokenSpec__1,
    extensible: CandyShared,
  });
  const EscrowRecord__1 = IDL.Record({
    token: TokenSpec__1,
    token_id: IDL.Text,
    seller: Account__1,
    lock_to_date: IDL.Opt(IDL.Int),
    buyer: Account__1,
    amount: IDL.Nat,
    sale_id: IDL.Opt(IDL.Text),
    account_hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const StableSalesBalances = IDL.Vec(
    IDL.Tuple(Account__1, Account__1, IDL.Text, EscrowRecord__1)
  );
  const StableOffers = IDL.Vec(IDL.Tuple(Account__1, Account__1, IDL.Int));
  const StableCollectionData = IDL.Record({
    active_bucket: IDL.Opt(IDL.Principal),
    managers: IDL.Vec(IDL.Principal),
    owner: IDL.Principal,
    metadata: IDL.Opt(CandyShared),
    logo: IDL.Opt(IDL.Text),
    name: IDL.Opt(IDL.Text),
    network: IDL.Opt(IDL.Principal),
    available_space: IDL.Nat,
    symbol: IDL.Opt(IDL.Text),
    allocated_storage: IDL.Nat,
  });
  const ICTokenSpec = IDL.Record({
    id: IDL.Opt(IDL.Nat),
    fee: IDL.Opt(IDL.Nat),
    decimals: IDL.Nat,
    canister: IDL.Principal,
    standard: IDL.Variant({
      ICRC1: IDL.Null,
      EXTFungible: IDL.Null,
      DIP20: IDL.Null,
      Other: CandyShared,
      Ledger: IDL.Null,
    }),
    symbol: IDL.Text,
  });
  const TokenSpec = IDL.Variant({
    ic: ICTokenSpec,
    extensible: CandyShared,
  });
  const TransactionID = IDL.Variant({
    nat: IDL.Nat,
    text: IDL.Text,
    extensible: CandyShared,
  });
  const Account__2 = IDL.Variant({
    account_id: IDL.Text,
    principal: IDL.Principal,
    extensible: CandyShared,
    account: IDL.Record({
      owner: IDL.Principal,
      sub_account: IDL.Opt(IDL.Vec(IDL.Nat8)),
    }),
  });
  const WaitForQuietType = IDL.Record({
    max: IDL.Nat,
    fade: IDL.Float64,
    extension: IDL.Nat64,
  });
  const MinIncreaseType = IDL.Variant({
    amount: IDL.Nat,
    percentage: IDL.Float64,
  });
  const FeeName = IDL.Text;
  const FeeAccountsParams = IDL.Vec(FeeName);
  const NiftySettlementType = IDL.Record({
    fixed: IDL.Bool,
    interestRatePerSecond: IDL.Float64,
    duration: IDL.Opt(IDL.Int),
    expiration: IDL.Opt(IDL.Int),
    lenderOffer: IDL.Bool,
  });
  const DutchParams = IDL.Record({
    time_unit: IDL.Variant({
      day: IDL.Nat,
      hour: IDL.Nat,
      minute: IDL.Nat,
    }),
    decay_type: IDL.Variant({ flat: IDL.Nat, percent: IDL.Float64 }),
  });
  const EndingType = IDL.Variant({ date: IDL.Int, timeout: IDL.Nat });
  const AskFeature = IDL.Variant({
    kyc: IDL.Principal,
    start_price: IDL.Nat,
    token: TokenSpec,
    fee_schema: IDL.Text,
    notify: IDL.Vec(IDL.Principal),
    wait_for_quiet: WaitForQuietType,
    reserve: IDL.Nat,
    start_date: IDL.Int,
    min_increase: MinIncreaseType,
    allow_list: IDL.Vec(IDL.Principal),
    buy_now: IDL.Nat,
    fee_accounts: FeeAccountsParams,
    nifty_settlement: NiftySettlementType,
    atomic: IDL.Null,
    dutch: DutchParams,
    ending: EndingType,
  });
  const AskFeatureArray = IDL.Vec(AskFeature);
  const AskConfigShared = IDL.Opt(AskFeatureArray);
  const InstantFeature = IDL.Variant({
    fee_schema: IDL.Text,
    fee_accounts: FeeAccountsParams,
    transfer: IDL.Null,
  });
  const InstantConfigShared = IDL.Opt(IDL.Vec(InstantFeature));
  const AuctionConfig = IDL.Record({
    start_price: IDL.Nat,
    token: TokenSpec,
    reserve: IDL.Opt(IDL.Nat),
    start_date: IDL.Int,
    min_increase: MinIncreaseType,
    allow_list: IDL.Opt(IDL.Vec(IDL.Principal)),
    buy_now: IDL.Opt(IDL.Nat),
    ending: IDL.Variant({
      date: IDL.Int,
      wait_for_quiet: IDL.Record({
        max: IDL.Nat,
        date: IDL.Int,
        fade: IDL.Float64,
        extension: IDL.Nat64,
      }),
    }),
  });
  const PricingConfigShared = IDL.Variant({
    ask: AskConfigShared,
    extensible: CandyShared,
    instant: InstantConfigShared,
    auction: AuctionConfig,
  });
  const TransactionRecord = IDL.Record({
    token_id: IDL.Text,
    txn_type: IDL.Variant({
      escrow_deposit: IDL.Record({
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      fee_deposit: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      canister_network_updated: IDL.Record({
        network: IDL.Principal,
        extensible: CandyShared,
      }),
      escrow_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      canister_managers_updated: IDL.Record({
        managers: IDL.Vec(IDL.Principal),
        extensible: CandyShared,
      }),
      auction_bid: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Text,
      }),
      burn: IDL.Record({
        from: IDL.Opt(Account__2),
        extensible: CandyShared,
      }),
      data: IDL.Record({
        hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
        extensible: CandyShared,
        data_dapp: IDL.Opt(IDL.Text),
        data_path: IDL.Opt(IDL.Text),
      }),
      sale_ended: IDL.Record({
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Opt(IDL.Text),
      }),
      mint: IDL.Record({
        to: Account__2,
        from: Account__2,
        sale: IDL.Opt(IDL.Record({ token: TokenSpec, amount: IDL.Nat })),
        extensible: CandyShared,
      }),
      royalty_paid: IDL.Record({
        tag: IDL.Text,
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        receiver: Account__2,
        sale_id: IDL.Opt(IDL.Text),
      }),
      extensible: CandyShared,
      fee_deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      owner_transfer: IDL.Record({
        to: Account__2,
        from: Account__2,
        extensible: CandyShared,
      }),
      sale_opened: IDL.Record({
        pricing: PricingConfigShared,
        extensible: CandyShared,
        sale_id: IDL.Text,
      }),
      canister_owner_updated: IDL.Record({
        owner: IDL.Principal,
        extensible: CandyShared,
      }),
      sale_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
    }),
    timestamp: IDL.Int,
    index: IDL.Nat,
  });
  const StableNftLedger = IDL.Vec(IDL.Tuple(IDL.Text, TransactionRecord));
  const AllocationRecordStable = IDL.Record({
    allocated_space: IDL.Nat,
    token_id: IDL.Text,
    available_space: IDL.Nat,
    canister: IDL.Principal,
    chunks: IDL.Vec(IDL.Nat),
    library_id: IDL.Text,
  });
  const BidFeature = IDL.Variant({
    fee_schema: IDL.Text,
    broker: Account__2,
    fee_accounts: FeeAccountsParams,
  });
  const BidConfigShared = IDL.Opt(IDL.Vec(BidFeature));
  const EscrowReceipt = IDL.Record({
    token: TokenSpec,
    token_id: IDL.Text,
    seller: Account__2,
    buyer: Account__2,
    amount: IDL.Nat,
  });
  const PricingConfigShared__1 = IDL.Variant({
    ask: AskConfigShared,
    extensible: CandyShared,
    instant: InstantConfigShared,
    auction: AuctionConfig,
  });
  const AuctionStateShared = IDL.Record({
    status: IDL.Variant({
      closed: IDL.Null,
      open: IDL.Null,
      not_started: IDL.Null,
    }),
    participants: IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Int)),
    token: TokenSpec__1,
    current_bid_amount: IDL.Nat,
    winner: IDL.Opt(Account__1),
    end_date: IDL.Int,
    current_config: BidConfigShared,
    start_date: IDL.Int,
    wait_for_quiet_count: IDL.Opt(IDL.Nat),
    current_escrow: IDL.Opt(EscrowReceipt),
    allow_list: IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Bool))),
    min_next_bid: IDL.Nat,
    config: PricingConfigShared__1,
  });
  const SaleStatusShared = IDL.Record({
    token_id: IDL.Text,
    sale_type: IDL.Variant({ auction: AuctionStateShared }),
    broker_id: IDL.Opt(IDL.Principal),
    original_broker_id: IDL.Opt(IDL.Principal),
    sale_id: IDL.Text,
  });
  const StableBucketData = IDL.Record({
    principal: IDL.Principal,
    allocated_space: IDL.Nat,
    date_added: IDL.Int,
    version: IDL.Tuple(IDL.Nat, IDL.Nat, IDL.Nat),
    b_gateway: IDL.Bool,
    available_space: IDL.Nat,
    allocations: IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Text), IDL.Int)),
  });
  const StableEscrowBalances = IDL.Vec(
    IDL.Tuple(Account__1, Account__1, IDL.Text, EscrowRecord__1)
  );
  const NFTBackupChunk = IDL.Record({
    sales_balances: StableSalesBalances,
    offers: StableOffers,
    collection_data: StableCollectionData,
    nft_ledgers: StableNftLedger,
    canister: IDL.Principal,
    allocations: IDL.Vec(
      IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Text), AllocationRecordStable)
    ),
    nft_sales: IDL.Vec(IDL.Tuple(IDL.Text, SaleStatusShared)),
    buckets: IDL.Vec(IDL.Tuple(IDL.Principal, StableBucketData)),
    escrow_balances: StableEscrowBalances,
  });
  const EXTTokenIdentifier = IDL.Text;
  const EXTUser = IDL.Variant({
    principal: IDL.Principal,
    address: IDL.Text,
  });
  const EXTBalanceRequest = IDL.Record({
    token: EXTTokenIdentifier,
    user: EXTUser,
  });
  const EXTBalance = IDL.Nat;
  const EXTCommonError = IDL.Variant({
    InvalidToken: EXTTokenIdentifier,
    Other: IDL.Text,
  });
  const EXTBalanceResult = IDL.Variant({
    ok: EXTBalance,
    err: EXTCommonError,
  });
  const StakeRecord = IDL.Record({
    staker: Account__1,
    token_id: IDL.Text,
    amount: IDL.Nat,
  });
  const BalanceResponse = IDL.Record({
    nfts: IDL.Vec(IDL.Text),
    offers: IDL.Vec(EscrowRecord__1),
    sales: IDL.Vec(EscrowRecord__1),
    stake: IDL.Vec(StakeRecord),
    multi_canister: IDL.Opt(IDL.Vec(IDL.Principal)),
    escrow: IDL.Vec(EscrowRecord__1),
  });
  const Errors = IDL.Variant({
    nyi: IDL.Null,
    storage_configuration_error: IDL.Null,
    escrow_withdraw_payment_failed: IDL.Null,
    token_not_found: IDL.Null,
    owner_not_found: IDL.Null,
    content_not_found: IDL.Null,
    auction_ended: IDL.Null,
    out_of_range: IDL.Null,
    sale_id_does_not_match: IDL.Null,
    sale_not_found: IDL.Null,
    kyc_fail: IDL.Null,
    item_not_owned: IDL.Null,
    property_not_found: IDL.Null,
    validate_trx_wrong_host: IDL.Null,
    withdraw_too_large: IDL.Null,
    content_not_deserializable: IDL.Null,
    bid_too_low: IDL.Null,
    validate_deposit_wrong_amount: IDL.Null,
    existing_sale_found: IDL.Null,
    noop: IDL.Null,
    asset_mismatch: IDL.Null,
    escrow_cannot_be_removed: IDL.Null,
    deposit_burned: IDL.Null,
    cannot_restage_minted_token: IDL.Null,
    cannot_find_status_in_metadata: IDL.Null,
    receipt_data_mismatch: IDL.Null,
    validate_deposit_failed: IDL.Null,
    unreachable: IDL.Null,
    unauthorized_access: IDL.Null,
    item_already_minted: IDL.Null,
    no_escrow_found: IDL.Null,
    escrow_owner_not_the_owner: IDL.Null,
    improper_interface: IDL.Null,
    app_id_not_found: IDL.Null,
    token_non_transferable: IDL.Null,
    kyc_error: IDL.Null,
    sale_not_over: IDL.Null,
    escrow_not_large_enough: IDL.Null,
    update_class_error: IDL.Null,
    malformed_metadata: IDL.Null,
    token_id_mismatch: IDL.Null,
    id_not_found_in_metadata: IDL.Null,
    auction_not_started: IDL.Null,
    low_fee_balance: IDL.Null,
    library_not_found: IDL.Null,
    attempt_to_stage_system_data: IDL.Null,
    no_fee_accounts_provided: IDL.Null,
    validate_deposit_wrong_buyer: IDL.Null,
    not_enough_storage: IDL.Null,
    sales_withdraw_payment_failed: IDL.Null,
  });
  const OrigynError = IDL.Record({
    text: IDL.Text,
    error: Errors,
    number: IDL.Nat32,
    flag_point: IDL.Text,
  });
  const BalanceResult = IDL.Variant({
    ok: BalanceResponse,
    err: OrigynError,
  });
  const EXTAccountIdentifier = IDL.Text;
  const EXTBearerResult = IDL.Variant({
    ok: EXTAccountIdentifier,
    err: EXTCommonError,
  });
  const BearerResult = IDL.Variant({ ok: Account__1, err: OrigynError });
  const canister_id = IDL.Principal;
  const definite_canister_settings = IDL.Record({
    freezing_threshold: IDL.Nat,
    controllers: IDL.Opt(IDL.Vec(IDL.Principal)),
    memory_allocation: IDL.Nat,
    compute_allocation: IDL.Nat,
  });
  const canister_status = IDL.Record({
    status: IDL.Variant({
      stopped: IDL.Null,
      stopping: IDL.Null,
      running: IDL.Null,
    }),
    memory_size: IDL.Nat,
    cycles: IDL.Nat,
    settings: definite_canister_settings,
    module_hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const ChunkRequest = IDL.Record({
    token_id: IDL.Text,
    chunk: IDL.Opt(IDL.Nat),
    library_id: IDL.Text,
  });
  const ChunkContent = IDL.Variant({
    remote: IDL.Record({
      args: ChunkRequest,
      canister: IDL.Principal,
    }),
    chunk: IDL.Record({
      total_chunks: IDL.Nat,
      content: IDL.Vec(IDL.Nat8),
      storage_allocation: AllocationRecordStable,
      current_chunk: IDL.Opt(IDL.Nat),
    }),
  });
  const ChunkResult = IDL.Variant({ ok: ChunkContent, err: OrigynError });
  const CollectionInfo = IDL.Record({
    multi_canister_count: IDL.Opt(IDL.Nat),
    managers: IDL.Opt(IDL.Vec(IDL.Principal)),
    owner: IDL.Opt(IDL.Principal),
    metadata: IDL.Opt(CandyShared),
    logo: IDL.Opt(IDL.Text),
    name: IDL.Opt(IDL.Text),
    network: IDL.Opt(IDL.Principal),
    created_at: IDL.Opt(IDL.Nat64),
    fields: IDL.Opt(
      IDL.Vec(IDL.Tuple(IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)))
    ),
    upgraded_at: IDL.Opt(IDL.Nat64),
    token_ids_count: IDL.Opt(IDL.Nat),
    available_space: IDL.Opt(IDL.Nat),
    multi_canister: IDL.Opt(IDL.Vec(IDL.Principal)),
    token_ids: IDL.Opt(IDL.Vec(IDL.Text)),
    transaction_count: IDL.Opt(IDL.Nat),
    unique_holders: IDL.Opt(IDL.Nat),
    total_supply: IDL.Opt(IDL.Nat),
    symbol: IDL.Opt(IDL.Text),
    allocated_storage: IDL.Opt(IDL.Nat),
  });
  const CollectionResult = IDL.Variant({
    ok: CollectionInfo,
    err: OrigynError,
  });
  const ManageCollectionCommand = IDL.Variant({
    UpdateOwner: IDL.Principal,
    UpdateManagers: IDL.Vec(IDL.Principal),
    UpdateMetadata: IDL.Tuple(IDL.Text, IDL.Opt(CandyShared), IDL.Bool),
    UpdateAnnounceCanister: IDL.Opt(IDL.Principal),
    UpdateNetwork: IDL.Opt(IDL.Principal),
    UpdateSymbol: IDL.Opt(IDL.Text),
    UpdateLogo: IDL.Opt(IDL.Text),
    UpdateName: IDL.Opt(IDL.Text),
  });
  const OrigynBoolResult = IDL.Variant({
    ok: IDL.Bool,
    err: OrigynError,
  });
  const Subaccount = IDL.Vec(IDL.Nat8);
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(Subaccount),
  });
  const NftError = IDL.Variant({
    UnauthorizedOperator: IDL.Null,
    SelfTransfer: IDL.Null,
    TokenNotFound: IDL.Null,
    UnauthorizedOwner: IDL.Null,
    TxNotFound: IDL.Null,
    SelfApprove: IDL.Null,
    OperatorNotFound: IDL.Null,
    ExistedNFT: IDL.Null,
    OwnerNotFound: IDL.Null,
    Other: IDL.Text,
  });
  const DIP721BoolResult = IDL.Variant({ Ok: IDL.Bool, Err: NftError });
  const DIP721Metadata = IDL.Record({
    logo: IDL.Opt(IDL.Text),
    name: IDL.Opt(IDL.Text),
    created_at: IDL.Nat64,
    upgraded_at: IDL.Nat64,
    custodians: IDL.Vec(IDL.Principal),
    symbol: IDL.Opt(IDL.Text),
  });
  const DIP721TokensListMetadata = IDL.Variant({
    Ok: IDL.Vec(IDL.Nat),
    Err: NftError,
  });
  Vec.fill(
    IDL.Vec(
      IDL.Tuple(
        IDL.Text,
        IDL.Variant({
          Nat64Content: IDL.Nat64,
          Nat32Content: IDL.Nat32,
          BoolContent: IDL.Bool,
          Nat8Content: IDL.Nat8,
          Int64Content: IDL.Int64,
          IntContent: IDL.Int,
          NatContent: IDL.Nat,
          Nat16Content: IDL.Nat16,
          Int32Content: IDL.Int32,
          Int8Content: IDL.Int8,
          FloatContent: IDL.Float64,
          Int16Content: IDL.Int16,
          BlobContent: IDL.Vec(IDL.Nat8),
          NestedContent: Vec,
          Principal: IDL.Principal,
          TextContent: IDL.Text,
        })
      )
    )
  );
  const GenericValue = IDL.Variant({
    Nat64Content: IDL.Nat64,
    Nat32Content: IDL.Nat32,
    BoolContent: IDL.Bool,
    Nat8Content: IDL.Nat8,
    Int64Content: IDL.Int64,
    IntContent: IDL.Int,
    NatContent: IDL.Nat,
    Nat16Content: IDL.Nat16,
    Int32Content: IDL.Int32,
    Int8Content: IDL.Int8,
    FloatContent: IDL.Float64,
    Int16Content: IDL.Int16,
    BlobContent: IDL.Vec(IDL.Nat8),
    NestedContent: Vec,
    Principal: IDL.Principal,
    TextContent: IDL.Text,
  });
  const TokenMetadata = IDL.Record({
    transferred_at: IDL.Opt(IDL.Nat64),
    transferred_by: IDL.Opt(IDL.Principal),
    owner: IDL.Opt(IDL.Principal),
    operator: IDL.Opt(IDL.Principal),
    approved_at: IDL.Opt(IDL.Nat64),
    approved_by: IDL.Opt(IDL.Principal),
    properties: IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    is_burned: IDL.Bool,
    token_identifier: IDL.Nat,
    burned_at: IDL.Opt(IDL.Nat64),
    burned_by: IDL.Opt(IDL.Principal),
    minted_at: IDL.Nat64,
    minted_by: IDL.Principal,
  });
  const DIP721TokensMetadata = IDL.Variant({
    Ok: IDL.Vec(TokenMetadata),
    Err: NftError,
  });
  const OwnerOfResponse = IDL.Variant({
    Ok: IDL.Opt(IDL.Principal),
    Err: NftError,
  });
  const DIP721Stats = IDL.Record({
    cycles: IDL.Nat,
    total_transactions: IDL.Nat,
    total_unique_holders: IDL.Nat,
    total_supply: IDL.Nat,
  });
  const DIP721SupportedInterface = IDL.Variant({
    Burn: IDL.Null,
    Mint: IDL.Null,
    Approval: IDL.Null,
    TransactionHistory: IDL.Null,
  });
  const DIP721TokenMetadata = IDL.Variant({
    Ok: TokenMetadata,
    Err: NftError,
  });
  const DIP721NatResult = IDL.Variant({ Ok: IDL.Nat, Err: NftError });
  const GetLogMessagesFilter = IDL.Record({
    analyzeCount: IDL.Nat32,
    messageRegex: IDL.Opt(IDL.Text),
    messageContains: IDL.Opt(IDL.Text),
  });
  const Nanos = IDL.Nat64;
  const GetLogMessagesParameters = IDL.Record({
    count: IDL.Nat32,
    filter: IDL.Opt(GetLogMessagesFilter),
    fromTimeNanos: IDL.Opt(Nanos),
  });
  const GetLatestLogMessagesParameters = IDL.Record({
    upToTimeNanos: IDL.Opt(Nanos),
    count: IDL.Nat32,
    filter: IDL.Opt(GetLogMessagesFilter),
  });
  const CanisterLogRequest = IDL.Variant({
    getMessagesInfo: IDL.Null,
    getMessages: GetLogMessagesParameters,
    getLatestMessages: GetLatestLogMessagesParameters,
  });
  const CanisterLogFeature = IDL.Variant({
    filterMessageByContains: IDL.Null,
    filterMessageByRegex: IDL.Null,
  });
  const CanisterLogMessagesInfo = IDL.Record({
    features: IDL.Vec(IDL.Opt(CanisterLogFeature)),
    lastTimeNanos: IDL.Opt(Nanos),
    count: IDL.Nat32,
    firstTimeNanos: IDL.Opt(Nanos),
  });
  const Data = IDL.Variant({
    Int: IDL.Int,
    Map: IDL.Vec(IDL.Tuple(CandyShared, CandyShared)),
    Nat: IDL.Nat,
    Set: IDL.Vec(CandyShared),
    Nat16: IDL.Nat16,
    Nat32: IDL.Nat32,
    Nat64: IDL.Nat64,
    Blob: IDL.Vec(IDL.Nat8),
    Bool: IDL.Bool,
    Int8: IDL.Int8,
    Ints: IDL.Vec(IDL.Int),
    Nat8: IDL.Nat8,
    Nats: IDL.Vec(IDL.Nat),
    Text: IDL.Text,
    Bytes: IDL.Vec(IDL.Nat8),
    Int16: IDL.Int16,
    Int32: IDL.Int32,
    Int64: IDL.Int64,
    Option: IDL.Opt(CandyShared),
    Floats: IDL.Vec(IDL.Float64),
    Float: IDL.Float64,
    Principal: IDL.Principal,
    Array: IDL.Vec(CandyShared),
    Class: IDL.Vec(PropertyShared),
  });
  const Caller = IDL.Opt(IDL.Principal);
  const LogMessagesData = IDL.Record({
    data: Data,
    timeNanos: Nanos,
    message: IDL.Text,
    caller: Caller,
  });
  const CanisterLogMessages = IDL.Record({
    data: IDL.Vec(LogMessagesData),
    lastAnalyzedMessageTimeNanos: IDL.Opt(Nanos),
  });
  const CanisterLogResponse = IDL.Variant({
    messagesInfo: CanisterLogMessagesInfo,
    messages: CanisterLogMessages,
  });
  const MetricsGranularity = IDL.Variant({
    hourly: IDL.Null,
    daily: IDL.Null,
  });
  const GetMetricsParameters = IDL.Record({
    dateToMillis: IDL.Nat,
    granularity: MetricsGranularity,
    dateFromMillis: IDL.Nat,
  });
  const UpdateCallsAggregatedData = IDL.Vec(IDL.Nat64);
  const CanisterHeapMemoryAggregatedData = IDL.Vec(IDL.Nat64);
  const CanisterCyclesAggregatedData = IDL.Vec(IDL.Nat64);
  const CanisterMemoryAggregatedData = IDL.Vec(IDL.Nat64);
  const HourlyMetricsData = IDL.Record({
    updateCalls: UpdateCallsAggregatedData,
    canisterHeapMemorySize: CanisterHeapMemoryAggregatedData,
    canisterCycles: CanisterCyclesAggregatedData,
    canisterMemorySize: CanisterMemoryAggregatedData,
    timeMillis: IDL.Int,
  });
  const NumericEntity = IDL.Record({
    avg: IDL.Nat64,
    max: IDL.Nat64,
    min: IDL.Nat64,
    first: IDL.Nat64,
    last: IDL.Nat64,
  });
  const DailyMetricsData = IDL.Record({
    updateCalls: IDL.Nat64,
    canisterHeapMemorySize: NumericEntity,
    canisterCycles: NumericEntity,
    canisterMemorySize: NumericEntity,
    timeMillis: IDL.Int,
  });
  const CanisterMetricsData = IDL.Variant({
    hourly: IDL.Vec(HourlyMetricsData),
    daily: IDL.Vec(DailyMetricsData),
  });
  const CanisterMetrics = IDL.Record({ data: CanisterMetricsData });
  const OrigynTextResult = IDL.Variant({
    ok: IDL.Text,
    err: OrigynError,
  });
  const Tip = IDL.Record({
    last_block_index: IDL.Vec(IDL.Nat8),
    hash_tree: IDL.Vec(IDL.Nat8),
    last_block_hash: IDL.Vec(IDL.Nat8),
  });
  const GovernanceRequest = IDL.Variant({
    update_system_var: IDL.Record({
      key: IDL.Text,
      val: CandyShared,
      token_id: IDL.Text,
    }),
    clear_shared_wallets: IDL.Text,
  });
  const GovernanceResponse = IDL.Variant({
    update_system_var: IDL.Bool,
    clear_shared_wallets: IDL.Bool,
  });
  const GovernanceResult = IDL.Variant({
    ok: GovernanceResponse,
    err: OrigynError,
  });
  const HistoryResult = IDL.Variant({
    ok: IDL.Vec(TransactionRecord),
    err: OrigynError,
  });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    url: IDL.Text,
    method: IDL.Text,
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(HeaderField),
  });
  const StreamingCallbackToken = IDL.Record({
    key: IDL.Text,
    index: IDL.Nat,
    content_encoding: IDL.Text,
  });
  const StreamingStrategy = IDL.Variant({
    Callback: IDL.Record({
      token: StreamingCallbackToken,
      callback: IDL.Func([], [], []),
    }),
  });
  const HTTPResponse = IDL.Record({
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(HeaderField),
    streaming_strategy: IDL.Opt(StreamingStrategy),
    status_code: IDL.Nat16,
  });
  const StreamingCallbackResponse = IDL.Record({
    token: IDL.Opt(StreamingCallbackToken),
    body: IDL.Vec(IDL.Nat8),
  });
  const GetArchivesArgs = IDL.Record({ from: IDL.Opt(IDL.Principal) });
  const GetArchivesResultItem = IDL.Record({
    end: IDL.Nat,
    canister_id: IDL.Principal,
    start: IDL.Nat,
  });
  const GetArchivesResult = IDL.Vec(GetArchivesResultItem);
  const TransactionRange = IDL.Record({
    start: IDL.Nat,
    length: IDL.Nat,
  });
  Value__1.fill(
    IDL.Variant({
      Int: IDL.Int,
      Map: IDL.Vec(IDL.Tuple(IDL.Text, Value__1)),
      Nat: IDL.Nat,
      Blob: IDL.Vec(IDL.Nat8),
      Text: IDL.Text,
      Array: IDL.Vec(Value__1),
    })
  );
  const TransactionRange__1 = IDL.Record({
    start: IDL.Nat,
    length: IDL.Nat,
  });
  const GetTransactionsResult__1 = IDL.Record({
    log_length: IDL.Nat,
    blocks: IDL.Vec(IDL.Record({ id: IDL.Nat, block: Value__1 })),
    archived_blocks: IDL.Vec(ArchivedTransactionResponse),
  });
  const GetTransactionsFn = IDL.Func(
    [IDL.Vec(TransactionRange__1)],
    [GetTransactionsResult__1],
    ["query"]
  );
  ArchivedTransactionResponse.fill(
    IDL.Record({
      args: IDL.Vec(TransactionRange__1),
      callback: GetTransactionsFn,
    })
  );
  const GetTransactionsResult = IDL.Record({
    log_length: IDL.Nat,
    blocks: IDL.Vec(IDL.Record({ id: IDL.Nat, block: Value__1 })),
    archived_blocks: IDL.Vec(ArchivedTransactionResponse),
  });
  const DataCertificate = IDL.Record({
    certificate: IDL.Vec(IDL.Nat8),
    hash_tree: IDL.Vec(IDL.Nat8),
  });
  const BlockType = IDL.Record({ url: IDL.Text, block_type: IDL.Text });
  const ApprovalArgs = IDL.Record({
    memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
    from_subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
    created_at_time: IDL.Opt(IDL.Nat64),
    expires_at: IDL.Opt(IDL.Nat64),
    spender: Account,
  });
  const ApprovalError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    CreatexInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    NonExistingTokenId: IDL.Null,
    Unauthorized: IDL.Null,
    TooOld: IDL.Null,
  });
  const ApprovalResult = IDL.Vec(
    IDL.Record({
      token_id: IDL.Nat,
      approval_result: IDL.Variant({
        Ok: IDL.Nat,
        Err: ApprovalError,
      }),
    })
  );
  Value.fill(
    IDL.Variant({
      Int: IDL.Int,
      Map: IDL.Vec(IDL.Tuple(IDL.Text, Value)),
      Nat: IDL.Nat,
      Blob: IDL.Vec(IDL.Nat8),
      Text: IDL.Text,
      Array: IDL.Vec(Value),
    })
  );
  const CollectionMetadata = IDL.Vec(IDL.Tuple(IDL.Text, Value));
  const SupportedStandard = IDL.Record({ url: IDL.Text, name: IDL.Text });
  const TransferArgs = IDL.Record({
    to: Account,
    token_id: IDL.Nat,
    memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
    from_subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
    created_at_time: IDL.Opt(IDL.Nat64),
  });
  const TransferError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    NonExistingTokenId: IDL.Null,
    Unauthorized: IDL.Null,
    CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    TooOld: IDL.Null,
  });
  const TransferResultItem = IDL.Record({
    token_id: IDL.Nat,
    transfer_result: IDL.Variant({ Ok: IDL.Nat, Err: TransferError }),
  });
  const TransferResult = IDL.Vec(IDL.Opt(TransferResultItem));
  const ManageStorageRequest = IDL.Variant({
    add_storage_canisters: IDL.Vec(
      IDL.Tuple(IDL.Principal, IDL.Nat, IDL.Tuple(IDL.Nat, IDL.Nat, IDL.Nat))
    ),
    configure_storage: IDL.Variant({
      stableBtree: IDL.Opt(IDL.Nat),
      heap: IDL.Opt(IDL.Nat),
    }),
  });
  const ManageStorageResponse = IDL.Variant({
    add_storage_canisters: IDL.Tuple(IDL.Nat, IDL.Nat),
    configure_storage: IDL.Tuple(IDL.Nat, IDL.Nat),
  });
  const ManageStorageResult = IDL.Variant({
    ok: ManageStorageResponse,
    err: OrigynError,
  });
  const EscrowReceipt__1 = IDL.Record({
    token: TokenSpec,
    token_id: IDL.Text,
    seller: Account__2,
    buyer: Account__2,
    amount: IDL.Nat,
  });
  const SalesConfig = IDL.Record({
    broker_id: IDL.Opt(Account__2),
    pricing: PricingConfigShared,
    escrow_receipt: IDL.Opt(EscrowReceipt__1),
  });
  const MarketTransferRequest = IDL.Record({
    token_id: IDL.Text,
    sales_config: SalesConfig,
  });
  const MarketTransferRequestReponse = IDL.Record({
    token_id: IDL.Text,
    txn_type: IDL.Variant({
      escrow_deposit: IDL.Record({
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      fee_deposit: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      canister_network_updated: IDL.Record({
        network: IDL.Principal,
        extensible: CandyShared,
      }),
      escrow_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      canister_managers_updated: IDL.Record({
        managers: IDL.Vec(IDL.Principal),
        extensible: CandyShared,
      }),
      auction_bid: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Text,
      }),
      burn: IDL.Record({
        from: IDL.Opt(Account__2),
        extensible: CandyShared,
      }),
      data: IDL.Record({
        hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
        extensible: CandyShared,
        data_dapp: IDL.Opt(IDL.Text),
        data_path: IDL.Opt(IDL.Text),
      }),
      sale_ended: IDL.Record({
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Opt(IDL.Text),
      }),
      mint: IDL.Record({
        to: Account__2,
        from: Account__2,
        sale: IDL.Opt(IDL.Record({ token: TokenSpec, amount: IDL.Nat })),
        extensible: CandyShared,
      }),
      royalty_paid: IDL.Record({
        tag: IDL.Text,
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        receiver: Account__2,
        sale_id: IDL.Opt(IDL.Text),
      }),
      extensible: CandyShared,
      fee_deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      owner_transfer: IDL.Record({
        to: Account__2,
        from: Account__2,
        extensible: CandyShared,
      }),
      sale_opened: IDL.Record({
        pricing: PricingConfigShared,
        extensible: CandyShared,
        sale_id: IDL.Text,
      }),
      canister_owner_updated: IDL.Record({
        owner: IDL.Principal,
        extensible: CandyShared,
      }),
      sale_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
    }),
    timestamp: IDL.Int,
    index: IDL.Nat,
  });
  const MarketTransferResult = IDL.Variant({
    ok: MarketTransferRequestReponse,
    err: OrigynError,
  });
  const EXTMetadata = IDL.Variant({
    fungible: IDL.Record({
      decimals: IDL.Nat8,
      metadata: IDL.Opt(IDL.Vec(IDL.Nat8)),
      name: IDL.Text,
      symbol: IDL.Text,
    }),
    nonfungible: IDL.Record({ metadata: IDL.Opt(IDL.Vec(IDL.Nat8)) }),
  });
  const EXTMetadataResult = IDL.Variant({
    ok: EXTMetadata,
    err: EXTCommonError,
  });
  const NFTInfoStable = IDL.Record({
    metadata: CandyShared,
    current_sale: IDL.Opt(SaleStatusShared),
  });
  const NFTInfoResult = IDL.Variant({
    ok: NFTInfoStable,
    err: OrigynError,
  });
  const TokenSpec__2 = IDL.Variant({
    ic: ICTokenSpec,
    extensible: CandyShared,
  });
  const Account__3 = IDL.Variant({
    account_id: IDL.Text,
    principal: IDL.Principal,
    extensible: CandyShared,
    account: IDL.Record({
      owner: IDL.Principal,
      sub_account: IDL.Opt(IDL.Vec(IDL.Nat8)),
    }),
  });
  const EscrowRecord = IDL.Record({
    token: TokenSpec__2,
    token_id: IDL.Text,
    seller: Account__3,
    lock_to_date: IDL.Opt(IDL.Int),
    buyer: Account__3,
    amount: IDL.Nat,
    sale_id: IDL.Opt(IDL.Text),
    account_hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BidRequest = IDL.Record({
    config: BidConfigShared,
    escrow_record: EscrowRecord,
  });
  const TransactionID__1 = IDL.Variant({
    nat: IDL.Nat,
    text: IDL.Text,
    extensible: CandyShared,
  });
  const DepositDetail = IDL.Record({
    token: TokenSpec__1,
    trx_id: IDL.Opt(TransactionID__1),
    seller: Account__1,
    buyer: Account__1,
    amount: IDL.Nat,
    sale_id: IDL.Opt(IDL.Text),
  });
  const EscrowRequest = IDL.Record({
    token_id: IDL.Text,
    deposit: DepositDetail,
    lock_to_date: IDL.Opt(IDL.Int),
  });
  const FeeDepositRequest = IDL.Record({
    token: TokenSpec__1,
    account: Account__1,
  });
  const RejectDescription = IDL.Record({
    token: TokenSpec__1,
    token_id: IDL.Text,
    seller: Account__1,
    buyer: Account__1,
  });
  const FeeDepositWithdrawDescription = IDL.Record({
    status: IDL.Variant({
      locked: IDL.Record({ token_id: IDL.Text, sale_id: IDL.Text }),
      unlocked: IDL.Null,
    }),
    token: TokenSpec__1,
    withdraw_to: Account__1,
    account: Account__1,
    amount: IDL.Nat,
  });
  const WithdrawDescription = IDL.Record({
    token: TokenSpec__1,
    token_id: IDL.Text,
    seller: Account__1,
    withdraw_to: Account__1,
    buyer: Account__1,
    amount: IDL.Nat,
  });
  const DepositWithdrawDescription = IDL.Record({
    token: TokenSpec__1,
    withdraw_to: Account__1,
    buyer: Account__1,
    amount: IDL.Nat,
  });
  const WithdrawRequest = IDL.Variant({
    reject: RejectDescription,
    fee_deposit: FeeDepositWithdrawDescription,
    sale: WithdrawDescription,
    deposit: DepositWithdrawDescription,
    escrow: WithdrawDescription,
  });
  const TokenSpecFilter = IDL.Record({
    token: TokenSpec__1,
    filter_type: IDL.Variant({ allow: IDL.Null, block: IDL.Null }),
  });
  const TokenIDFilter = IDL.Record({
    filter_type: IDL.Variant({ allow: IDL.Null, block: IDL.Null }),
    token_id: IDL.Text,
    tokens: IDL.Vec(
      IDL.Record({
        token: TokenSpec__1,
        min_amount: IDL.Opt(IDL.Nat),
        max_amount: IDL.Opt(IDL.Nat),
      })
    ),
  });
  const AskSubscribeRequest = IDL.Variant({
    subscribe: IDL.Record({
      stake: IDL.Tuple(IDL.Principal, IDL.Nat),
      filter: IDL.Opt(
        IDL.Record({
          tokens: IDL.Opt(IDL.Vec(TokenSpecFilter)),
          token_ids: IDL.Opt(IDL.Vec(TokenIDFilter)),
        })
      ),
    }),
    unsubscribe: IDL.Tuple(IDL.Principal, IDL.Nat),
  });
  const DistributeSaleRequest = IDL.Record({ seller: IDL.Opt(Account__1) });
  const ManageSaleRequest = IDL.Variant({
    bid: BidRequest,
    escrow_deposit: EscrowRequest,
    fee_deposit: FeeDepositRequest,
    recognize_escrow: EscrowRequest,
    withdraw: WithdrawRequest,
    ask_subscribe: AskSubscribeRequest,
    end_sale: IDL.Text,
    refresh_offers: IDL.Opt(Account__1),
    distribute_sale: DistributeSaleRequest,
    open_sale: IDL.Text,
  });
  const BidResponse = IDL.Record({
    token_id: IDL.Text,
    txn_type: IDL.Variant({
      escrow_deposit: IDL.Record({
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      fee_deposit: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      canister_network_updated: IDL.Record({
        network: IDL.Principal,
        extensible: CandyShared,
      }),
      escrow_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      canister_managers_updated: IDL.Record({
        managers: IDL.Vec(IDL.Principal),
        extensible: CandyShared,
      }),
      auction_bid: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Text,
      }),
      burn: IDL.Record({
        from: IDL.Opt(Account__2),
        extensible: CandyShared,
      }),
      data: IDL.Record({
        hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
        extensible: CandyShared,
        data_dapp: IDL.Opt(IDL.Text),
        data_path: IDL.Opt(IDL.Text),
      }),
      sale_ended: IDL.Record({
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Opt(IDL.Text),
      }),
      mint: IDL.Record({
        to: Account__2,
        from: Account__2,
        sale: IDL.Opt(IDL.Record({ token: TokenSpec, amount: IDL.Nat })),
        extensible: CandyShared,
      }),
      royalty_paid: IDL.Record({
        tag: IDL.Text,
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        receiver: Account__2,
        sale_id: IDL.Opt(IDL.Text),
      }),
      extensible: CandyShared,
      fee_deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      owner_transfer: IDL.Record({
        to: Account__2,
        from: Account__2,
        extensible: CandyShared,
      }),
      sale_opened: IDL.Record({
        pricing: PricingConfigShared,
        extensible: CandyShared,
        sale_id: IDL.Text,
      }),
      canister_owner_updated: IDL.Record({
        owner: IDL.Principal,
        extensible: CandyShared,
      }),
      sale_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
    }),
    timestamp: IDL.Int,
    index: IDL.Nat,
  });
  const EscrowResponse = IDL.Record({
    balance: IDL.Nat,
    receipt: EscrowReceipt,
    transaction: TransactionRecord,
  });
  const FeeDepositResponse = IDL.Record({
    balance: IDL.Nat,
    transaction: TransactionRecord,
  });
  const RecognizeEscrowResponse = IDL.Record({
    balance: IDL.Nat,
    receipt: EscrowReceipt,
    transaction: IDL.Opt(TransactionRecord),
  });
  const WithdrawResponse = IDL.Record({
    token_id: IDL.Text,
    txn_type: IDL.Variant({
      escrow_deposit: IDL.Record({
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      fee_deposit: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      canister_network_updated: IDL.Record({
        network: IDL.Principal,
        extensible: CandyShared,
      }),
      escrow_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      canister_managers_updated: IDL.Record({
        managers: IDL.Vec(IDL.Principal),
        extensible: CandyShared,
      }),
      auction_bid: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Text,
      }),
      burn: IDL.Record({
        from: IDL.Opt(Account__2),
        extensible: CandyShared,
      }),
      data: IDL.Record({
        hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
        extensible: CandyShared,
        data_dapp: IDL.Opt(IDL.Text),
        data_path: IDL.Opt(IDL.Text),
      }),
      sale_ended: IDL.Record({
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Opt(IDL.Text),
      }),
      mint: IDL.Record({
        to: Account__2,
        from: Account__2,
        sale: IDL.Opt(IDL.Record({ token: TokenSpec, amount: IDL.Nat })),
        extensible: CandyShared,
      }),
      royalty_paid: IDL.Record({
        tag: IDL.Text,
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        receiver: Account__2,
        sale_id: IDL.Opt(IDL.Text),
      }),
      extensible: CandyShared,
      fee_deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      owner_transfer: IDL.Record({
        to: Account__2,
        from: Account__2,
        extensible: CandyShared,
      }),
      sale_opened: IDL.Record({
        pricing: PricingConfigShared,
        extensible: CandyShared,
        sale_id: IDL.Text,
      }),
      canister_owner_updated: IDL.Record({
        owner: IDL.Principal,
        extensible: CandyShared,
      }),
      sale_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
    }),
    timestamp: IDL.Int,
    index: IDL.Nat,
  });
  const AskSubscribeResponse = IDL.Bool;
  const EndSaleResponse = IDL.Record({
    token_id: IDL.Text,
    txn_type: IDL.Variant({
      escrow_deposit: IDL.Record({
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      fee_deposit: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      canister_network_updated: IDL.Record({
        network: IDL.Principal,
        extensible: CandyShared,
      }),
      escrow_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      canister_managers_updated: IDL.Record({
        managers: IDL.Vec(IDL.Principal),
        extensible: CandyShared,
      }),
      auction_bid: IDL.Record({
        token: TokenSpec,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Text,
      }),
      burn: IDL.Record({
        from: IDL.Opt(Account__2),
        extensible: CandyShared,
      }),
      data: IDL.Record({
        hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
        extensible: CandyShared,
        data_dapp: IDL.Opt(IDL.Text),
        data_path: IDL.Opt(IDL.Text),
      }),
      sale_ended: IDL.Record({
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        sale_id: IDL.Opt(IDL.Text),
      }),
      mint: IDL.Record({
        to: Account__2,
        from: Account__2,
        sale: IDL.Opt(IDL.Record({ token: TokenSpec, amount: IDL.Nat })),
        extensible: CandyShared,
      }),
      royalty_paid: IDL.Record({
        tag: IDL.Text,
        token: TokenSpec,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
        receiver: Account__2,
        sale_id: IDL.Opt(IDL.Text),
      }),
      extensible: CandyShared,
      fee_deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        account: Account__2,
        amount: IDL.Nat,
      }),
      owner_transfer: IDL.Record({
        to: Account__2,
        from: Account__2,
        extensible: CandyShared,
      }),
      sale_opened: IDL.Record({
        pricing: PricingConfigShared,
        extensible: CandyShared,
        sale_id: IDL.Text,
      }),
      canister_owner_updated: IDL.Record({
        owner: IDL.Principal,
        extensible: CandyShared,
      }),
      sale_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        token_id: IDL.Text,
        trx_id: TransactionID,
        seller: Account__2,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
      deposit_withdraw: IDL.Record({
        fee: IDL.Nat,
        token: TokenSpec,
        trx_id: TransactionID,
        extensible: CandyShared,
        buyer: Account__2,
        amount: IDL.Nat,
      }),
    }),
    timestamp: IDL.Int,
    index: IDL.Nat,
  });
  const Result = IDL.Variant({
    ok: ManageSaleResponse,
    err: OrigynError,
  });
  const DistributeSaleResponse = IDL.Vec(Result);
  ManageSaleResponse.fill(
    IDL.Variant({
      bid: BidResponse,
      escrow_deposit: EscrowResponse,
      fee_deposit: FeeDepositResponse,
      recognize_escrow: RecognizeEscrowResponse,
      withdraw: WithdrawResponse,
      ask_subscribe: AskSubscribeResponse,
      end_sale: EndSaleResponse,
      refresh_offers: IDL.Vec(EscrowRecord__1),
      distribute_sale: DistributeSaleResponse,
      open_sale: IDL.Bool,
    })
  );
  const ManageSaleResult = IDL.Variant({
    ok: ManageSaleResponse,
    err: OrigynError,
  });
  const SaleInfoRequest = IDL.Variant({
    status: IDL.Text,
    fee_deposit_info: IDL.Opt(Account__1),
    active: IDL.Opt(IDL.Tuple(IDL.Nat, IDL.Nat)),
    deposit_info: IDL.Opt(Account__1),
    history: IDL.Opt(IDL.Tuple(IDL.Nat, IDL.Nat)),
    escrow_info: EscrowReceipt,
  });
  const SubAccountInfo = IDL.Record({
    account_id: IDL.Vec(IDL.Nat8),
    principal: IDL.Principal,
    account_id_text: IDL.Text,
    account: IDL.Record({
      principal: IDL.Principal,
      sub_account: IDL.Vec(IDL.Nat8),
    }),
  });
  const SaleInfoResponse = IDL.Variant({
    status: IDL.Opt(SaleStatusShared),
    fee_deposit_info: SubAccountInfo,
    active: IDL.Record({
      eof: IDL.Bool,
      records: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Opt(SaleStatusShared))),
      count: IDL.Nat,
    }),
    deposit_info: SubAccountInfo,
    history: IDL.Record({
      eof: IDL.Bool,
      records: IDL.Vec(IDL.Opt(SaleStatusShared)),
      count: IDL.Nat,
    }),
    escrow_info: SubAccountInfo,
  });
  const SaleInfoResult = IDL.Variant({
    ok: SaleInfoResponse,
    err: OrigynError,
  });
  const ShareWalletRequest = IDL.Record({
    to: Account__1,
    token_id: IDL.Text,
    from: Account__1,
  });
  const OwnerTransferResponse = IDL.Record({
    transaction: TransactionRecord,
    assets: IDL.Vec(CandyShared),
  });
  const OwnerUpdateResult = IDL.Variant({
    ok: OwnerTransferResponse,
    err: OrigynError,
  });
  const StageChunkArg = IDL.Record({
    content: IDL.Vec(IDL.Nat8),
    token_id: IDL.Text,
    chunk: IDL.Nat,
    filedata: CandyShared,
    library_id: IDL.Text,
  });
  const StageLibraryResponse = IDL.Record({ canister: IDL.Principal });
  const StageLibraryResult = IDL.Variant({
    ok: StageLibraryResponse,
    err: OrigynError,
  });
  const StateSize = IDL.Record({
    sales_balances: IDL.Nat,
    offers: IDL.Nat,
    nft_ledgers: IDL.Nat,
    allocations: IDL.Nat,
    nft_sales: IDL.Nat,
    buckets: IDL.Nat,
    escrow_balances: IDL.Nat,
  });
  const StorageMetrics = IDL.Record({
    gateway: IDL.Principal,
    available_space: IDL.Nat,
    allocations: IDL.Vec(AllocationRecordStable),
    allocated_storage: IDL.Nat,
  });
  const StorageMetricsResult = IDL.Variant({
    ok: StorageMetrics,
    err: OrigynError,
  });
  const EXTTokensResponse = IDL.Tuple(
    IDL.Nat32,
    IDL.Opt(
      IDL.Record({
        locked: IDL.Opt(IDL.Int),
        seller: IDL.Principal,
        price: IDL.Nat64,
      })
    ),
    IDL.Opt(IDL.Vec(IDL.Nat8))
  );
  const EXTTokensResult = IDL.Variant({
    ok: IDL.Vec(EXTTokensResponse),
    err: EXTCommonError,
  });
  const EXTMemo = IDL.Vec(IDL.Nat8);
  const EXTSubAccount = IDL.Vec(IDL.Nat8);
  const EXTTransferRequest = IDL.Record({
    to: EXTUser,
    token: EXTTokenIdentifier,
    notify: IDL.Bool,
    from: EXTUser,
    memo: EXTMemo,
    subaccount: IDL.Opt(EXTSubAccount),
    amount: EXTBalance,
  });
  const EXTTransferResponse = IDL.Variant({
    ok: EXTBalance,
    err: IDL.Variant({
      CannotNotify: EXTAccountIdentifier,
      InsufficientBalance: IDL.Null,
      InvalidToken: EXTTokenIdentifier,
      Rejected: IDL.Null,
      Unauthorized: EXTAccountIdentifier,
      Other: IDL.Text,
    }),
  });
  const UpdateModeShared = IDL.Variant({
    Set: CandyShared,
    Lock: CandyShared,
    Next: IDL.Vec(UpdateShared),
  });
  UpdateShared.fill(IDL.Record({ mode: UpdateModeShared, name: IDL.Text }));
  const UpdateRequestShared = IDL.Record({
    id: IDL.Text,
    update: IDL.Vec(UpdateShared),
  });
  const NFTUpdateRequest = IDL.Variant({
    update: IDL.Record({
      token_id: IDL.Text,
      update: UpdateRequestShared,
      app_id: IDL.Text,
    }),
    replace: IDL.Record({ token_id: IDL.Text, data: CandyShared }),
  });
  const NFTUpdateResponse = IDL.Bool;
  const NFTUpdateResult = IDL.Variant({
    ok: NFTUpdateResponse,
    err: OrigynError,
  });
  const IndexType = IDL.Variant({
    Stable: IDL.Null,
    StableTyped: IDL.Null,
    Managed: IDL.Null,
  });
  const UpdateSetting = IDL.Variant({
    maxRecordsToArchive: IDL.Nat,
    archiveIndexType: IndexType,
    maxArchivePages: IDL.Nat,
    settleToRecords: IDL.Nat,
    archiveCycles: IDL.Nat,
    maxActiveRecords: IDL.Nat,
    maxRecordsInArchiveInstance: IDL.Nat,
    archiveControllers: IDL.Opt(IDL.Opt(IDL.Vec(IDL.Principal))),
  });
  const NFTUpdateMetadataNode = IDL.Record({
    token_id: IDL.Text,
    value: CandyShared,
    _system: IDL.Bool,
    field_id: IDL.Text,
  });
  const NFTUpdateMetadataNodeResponse = IDL.Record({
    property_new: PropertyShared,
    property_old: IDL.Opt(PropertyShared),
  });
  const NFTUpdateAppResult = IDL.Variant({
    ok: NFTUpdateMetadataNodeResponse,
    err: OrigynError,
  });
  const Nft_Canister = IDL.Service({
    __advance_time: IDL.Func([IDL.Int], [IDL.Int], []),
    __set_time_mode: IDL.Func(
      [IDL.Variant({ test: IDL.Null, standard: IDL.Null })],
      [IDL.Bool],
      []
    ),
    __supports: IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
      ["query"]
    ),
    __version: IDL.Func([], [IDL.Text], ["query"]),
    back_up: IDL.Func(
      [IDL.Nat],
      [IDL.Variant({ eof: NFTBackupChunk, data: NFTBackupChunk })],
      ["query"]
    ),
    balance: IDL.Func([EXTBalanceRequest], [EXTBalanceResult], ["query"]),
    balanceEXT: IDL.Func([EXTBalanceRequest], [EXTBalanceResult], ["query"]),
    balance_of_batch_nft_origyn: IDL.Func(
      [IDL.Vec(Account__1)],
      [IDL.Vec(BalanceResult)],
      ["query"]
    ),
    balance_of_nft_origyn: IDL.Func([Account__1], [BalanceResult], ["query"]),
    balance_of_secure_batch_nft_origyn: IDL.Func(
      [IDL.Vec(Account__1)],
      [IDL.Vec(BalanceResult)],
      []
    ),
    balance_of_secure_nft_origyn: IDL.Func([Account__1], [BalanceResult], []),
    bearer: IDL.Func([EXTTokenIdentifier], [EXTBearerResult], ["query"]),
    bearerEXT: IDL.Func([EXTTokenIdentifier], [EXTBearerResult], ["query"]),
    bearer_batch_nft_origyn: IDL.Func(
      [IDL.Vec(IDL.Text)],
      [IDL.Vec(BearerResult)],
      ["query"]
    ),
    bearer_batch_secure_nft_origyn: IDL.Func(
      [IDL.Vec(IDL.Text)],
      [IDL.Vec(BearerResult)],
      []
    ),
    bearer_nft_origyn: IDL.Func([IDL.Text], [BearerResult], ["query"]),
    bearer_secure_nft_origyn: IDL.Func([IDL.Text], [BearerResult], []),
    canister_status: IDL.Func(
      [IDL.Record({ canister_id: canister_id })],
      [canister_status],
      []
    ),
    chunk_nft_origyn: IDL.Func([ChunkRequest], [ChunkResult], ["query"]),
    chunk_secure_nft_origyn: IDL.Func([ChunkRequest], [ChunkResult], []),
    collectCanisterMetrics: IDL.Func([], [], ["query"]),
    collection_nft_origyn: IDL.Func(
      [
        IDL.Opt(
          IDL.Vec(IDL.Tuple(IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)))
        ),
      ],
      [CollectionResult],
      ["query"]
    ),
    collection_secure_nft_origyn: IDL.Func(
      [
        IDL.Opt(
          IDL.Vec(IDL.Tuple(IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)))
        ),
      ],
      [CollectionResult],
      []
    ),
    collection_update_batch_nft_origyn: IDL.Func(
      [IDL.Vec(ManageCollectionCommand)],
      [IDL.Vec(OrigynBoolResult)],
      []
    ),
    collection_update_nft_origyn: IDL.Func(
      [ManageCollectionCommand],
      [OrigynBoolResult],
      []
    ),
    count_unlisted_tokens_of: IDL.Func([Account], [IDL.Nat], []),
    cycles: IDL.Func([], [IDL.Nat], ["query"]),
    dip721_balance_of: IDL.Func([IDL.Principal], [IDL.Nat], ["query"]),
    dip721_custodians: IDL.Func([], [IDL.Vec(IDL.Principal)], ["query"]),
    dip721_is_approved_for_all: IDL.Func(
      [IDL.Principal, IDL.Principal],
      [DIP721BoolResult],
      ["query"]
    ),
    dip721_logo: IDL.Func([], [IDL.Opt(IDL.Text)], ["query"]),
    dip721_metadata: IDL.Func([], [DIP721Metadata], ["query"]),
    dip721_name: IDL.Func([], [IDL.Opt(IDL.Text)], ["query"]),
    dip721_operator_token_identifiers: IDL.Func(
      [IDL.Principal],
      [DIP721TokensListMetadata],
      ["query"]
    ),
    dip721_operator_token_metadata: IDL.Func(
      [IDL.Principal],
      [DIP721TokensMetadata],
      ["query"]
    ),
    dip721_owner_of: IDL.Func([IDL.Nat], [OwnerOfResponse], ["query"]),
    dip721_owner_token_identifiers: IDL.Func(
      [IDL.Principal],
      [DIP721TokensListMetadata],
      ["query"]
    ),
    dip721_owner_token_metadata: IDL.Func(
      [IDL.Principal],
      [DIP721TokensMetadata],
      ["query"]
    ),
    dip721_stats: IDL.Func([], [DIP721Stats], ["query"]),
    dip721_supported_interfaces: IDL.Func(
      [],
      [IDL.Vec(DIP721SupportedInterface)],
      ["query"]
    ),
    dip721_symbol: IDL.Func([], [IDL.Opt(IDL.Text)], ["query"]),
    dip721_token_metadata: IDL.Func(
      [IDL.Nat],
      [DIP721TokenMetadata],
      ["query"]
    ),
    dip721_total_supply: IDL.Func([], [IDL.Nat], ["query"]),
    dip721_total_transactions: IDL.Func([], [IDL.Nat], ["query"]),
    dip721_transfer: IDL.Func([IDL.Principal, IDL.Nat], [DIP721NatResult], []),
    dip721_transfer_from: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat],
      [DIP721NatResult],
      []
    ),
    getCanisterLog: IDL.Func(
      [IDL.Opt(CanisterLogRequest)],
      [IDL.Opt(CanisterLogResponse)],
      ["query"]
    ),
    getCanisterMetrics: IDL.Func(
      [GetMetricsParameters],
      [IDL.Opt(CanisterMetrics)],
      ["query"]
    ),
    getEXTTokenIdentifier: IDL.Func([IDL.Text], [IDL.Text], ["query"]),
    get_access_key: IDL.Func([], [OrigynTextResult], ["query"]),
    get_halt: IDL.Func([], [IDL.Bool], ["query"]),
    get_nat_as_token_id_origyn: IDL.Func([IDL.Nat], [IDL.Text], ["query"]),
    get_tip: IDL.Func([], [Tip], ["query"]),
    get_token_id_as_nat: IDL.Func([IDL.Text], [IDL.Nat], ["query"]),
    governance_batch_nft_origyn: IDL.Func(
      [IDL.Vec(GovernanceRequest)],
      [IDL.Vec(GovernanceResult)],
      []
    ),
    governance_nft_origyn: IDL.Func(
      [GovernanceRequest],
      [GovernanceResult],
      []
    ),
    history_batch_nft_origyn: IDL.Func(
      [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)))],
      [IDL.Vec(HistoryResult)],
      ["query"]
    ),
    history_batch_secure_nft_origyn: IDL.Func(
      [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)))],
      [IDL.Vec(HistoryResult)],
      []
    ),
    history_nft_origyn: IDL.Func(
      [IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)],
      [HistoryResult],
      ["query"]
    ),
    history_secure_nft_origyn: IDL.Func(
      [IDL.Text, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)],
      [HistoryResult],
      []
    ),
    http_access_key: IDL.Func([], [OrigynTextResult], []),
    http_request: IDL.Func([HttpRequest], [HTTPResponse], ["query"]),
    http_request_streaming_callback: IDL.Func(
      [StreamingCallbackToken],
      [StreamingCallbackResponse],
      ["query"]
    ),
    icrc3_get_archives: IDL.Func(
      [GetArchivesArgs],
      [GetArchivesResult],
      ["query"]
    ),
    icrc3_get_blocks: IDL.Func(
      [IDL.Vec(TransactionRange)],
      [GetTransactionsResult],
      ["query"]
    ),
    icrc3_get_tip_certificate: IDL.Func(
      [],
      [IDL.Opt(DataCertificate)],
      ["query"]
    ),
    icrc3_supported_block_types: IDL.Func([], [IDL.Vec(BlockType)], ["query"]),
    icrc7_approve: IDL.Func([ApprovalArgs], [ApprovalResult], []),
    icrc7_atomic_batch_transfers: IDL.Func([], [IDL.Opt(IDL.Bool)], ["query"]),
    icrc7_balance_of: IDL.Func(
      [IDL.Vec(Account)],
      [IDL.Vec(IDL.Nat)],
      ["query"]
    ),
    icrc7_collection_metadata: IDL.Func([], [CollectionMetadata], ["query"]),
    icrc7_default_take_value: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_description: IDL.Func([], [IDL.Opt(IDL.Text)], ["query"]),
    icrc7_logo: IDL.Func([], [IDL.Opt(IDL.Text)], ["query"]),
    icrc7_max_approvals_per_token_or_collection: IDL.Func(
      [],
      [IDL.Opt(IDL.Nat)],
      ["query"]
    ),
    icrc7_max_memo_size: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_max_query_batch_size: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_max_revoke_approvals: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_max_take_value: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_max_update_batch_size: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_name: IDL.Func([], [IDL.Text], ["query"]),
    icrc7_owner_of: IDL.Func(
      [IDL.Vec(IDL.Nat)],
      [IDL.Vec(IDL.Opt(Account))],
      ["query"]
    ),
    icrc7_permitted_drift: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_supply_cap: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_supported_standards: IDL.Func(
      [],
      [IDL.Vec(SupportedStandard)],
      ["query"]
    ),
    icrc7_symbol: IDL.Func([], [IDL.Text], ["query"]),
    icrc7_token_metadata: IDL.Func(
      [IDL.Vec(IDL.Nat)],
      [IDL.Vec(IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, Value))))],
      ["query"]
    ),
    icrc7_tokens: IDL.Func(
      [IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat32)],
      [IDL.Vec(IDL.Nat)],
      ["query"]
    ),
    icrc7_tokens_of: IDL.Func(
      [Account, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat32)],
      [IDL.Vec(IDL.Nat)],
      ["query"]
    ),
    icrc7_total_supply: IDL.Func([], [IDL.Nat], ["query"]),
    icrc7_transfer: IDL.Func([IDL.Vec(TransferArgs)], [TransferResult], []),
    icrc7_transfer_fee: IDL.Func([IDL.Nat], [IDL.Opt(IDL.Nat)], ["query"]),
    icrc7_tx_window: IDL.Func([], [IDL.Opt(IDL.Nat)], ["query"]),
    manage_storage_nft_origyn: IDL.Func(
      [ManageStorageRequest],
      [ManageStorageResult],
      []
    ),
    market_transfer_batch_nft_origyn: IDL.Func(
      [IDL.Vec(MarketTransferRequest)],
      [IDL.Vec(MarketTransferResult)],
      []
    ),
    market_transfer_nft_origyn: IDL.Func(
      [MarketTransferRequest],
      [MarketTransferResult],
      []
    ),
    metadata: IDL.Func([], [DIP721Metadata], ["query"]),
    metadataExt: IDL.Func([EXTTokenIdentifier], [EXTMetadataResult], ["query"]),
    mint_batch_nft_origyn: IDL.Func(
      [IDL.Vec(IDL.Tuple(IDL.Text, Account__1))],
      [IDL.Vec(OrigynTextResult)],
      []
    ),
    mint_nft_origyn: IDL.Func([IDL.Text, Account__1], [OrigynTextResult], []),
    nftStreamingCallback: IDL.Func(
      [StreamingCallbackToken],
      [StreamingCallbackResponse],
      ["query"]
    ),
    nft_batch_origyn: IDL.Func(
      [IDL.Vec(IDL.Text)],
      [IDL.Vec(NFTInfoResult)],
      ["query"]
    ),
    nft_batch_secure_origyn: IDL.Func(
      [IDL.Vec(IDL.Text)],
      [IDL.Vec(NFTInfoResult)],
      []
    ),
    nft_origyn: IDL.Func([IDL.Text], [NFTInfoResult], ["query"]),
    nft_secure_origyn: IDL.Func([IDL.Text], [NFTInfoResult], []),
    operaterTokenMetadata: IDL.Func(
      [IDL.Principal],
      [DIP721TokensMetadata],
      ["query"]
    ),
    ownerOf: IDL.Func([IDL.Nat], [OwnerOfResponse], ["query"]),
    ownerTokenMetadata: IDL.Func(
      [IDL.Principal],
      [DIP721TokensMetadata],
      ["query"]
    ),
    sale_batch_nft_origyn: IDL.Func(
      [IDL.Vec(ManageSaleRequest)],
      [IDL.Vec(ManageSaleResult)],
      []
    ),
    sale_info_batch_nft_origyn: IDL.Func(
      [IDL.Vec(SaleInfoRequest)],
      [IDL.Vec(SaleInfoResult)],
      ["query"]
    ),
    sale_info_batch_secure_nft_origyn: IDL.Func(
      [IDL.Vec(SaleInfoRequest)],
      [IDL.Vec(SaleInfoResult)],
      []
    ),
    sale_info_nft_origyn: IDL.Func(
      [SaleInfoRequest],
      [SaleInfoResult],
      ["query"]
    ),
    sale_info_secure_nft_origyn: IDL.Func(
      [SaleInfoRequest],
      [SaleInfoResult],
      []
    ),
    sale_nft_origyn: IDL.Func([ManageSaleRequest], [ManageSaleResult], []),
    set_data_harvester: IDL.Func([IDL.Nat], [], []),
    set_halt: IDL.Func([IDL.Bool], [], []),
    share_wallet_nft_origyn: IDL.Func(
      [ShareWalletRequest],
      [OwnerUpdateResult],
      []
    ),
    stage_batch_nft_origyn: IDL.Func(
      [IDL.Vec(IDL.Record({ metadata: CandyShared }))],
      [IDL.Vec(OrigynTextResult)],
      []
    ),
    stage_library_batch_nft_origyn: IDL.Func(
      [IDL.Vec(StageChunkArg)],
      [IDL.Vec(StageLibraryResult)],
      []
    ),
    stage_library_nft_origyn: IDL.Func(
      [StageChunkArg],
      [StageLibraryResult],
      []
    ),
    stage_nft_origyn: IDL.Func(
      [IDL.Record({ metadata: CandyShared })],
      [OrigynTextResult],
      []
    ),
    state_size: IDL.Func([], [StateSize], ["query"]),
    storage_info_nft_origyn: IDL.Func([], [StorageMetricsResult], ["query"]),
    storage_info_secure_nft_origyn: IDL.Func([], [StorageMetricsResult], []),
    tokens_ext: IDL.Func([IDL.Text], [EXTTokensResult], ["query"]),
    transfer: IDL.Func([EXTTransferRequest], [EXTTransferResponse], []),
    transferDip721: IDL.Func([IDL.Principal, IDL.Nat], [DIP721NatResult], []),
    transferEXT: IDL.Func([EXTTransferRequest], [EXTTransferResponse], []),
    transferFrom: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat],
      [DIP721NatResult],
      []
    ),
    transferFromDip721: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat],
      [DIP721NatResult],
      []
    ),
    unlisted_tokens_of: IDL.Func(
      [Account, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat32)],
      [IDL.Vec(IDL.Nat)],
      []
    ),
    update_app_nft_origyn: IDL.Func([NFTUpdateRequest], [NFTUpdateResult], []),
    update_icrc3: IDL.Func([IDL.Vec(UpdateSetting)], [IDL.Vec(IDL.Bool)], []),
    update_metadata_node: IDL.Func(
      [NFTUpdateMetadataNode],
      [NFTUpdateAppResult],
      []
    ),
    wallet_receive: IDL.Func([], [IDL.Nat], []),
    whoami: IDL.Func([], [IDL.Principal], ["query"]),
  });
  return Nft_Canister;
};

export const init = ({ IDL }) => {
  return [];
};
