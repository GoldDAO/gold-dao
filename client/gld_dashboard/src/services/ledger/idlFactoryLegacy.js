export const idlFactory = ({ IDL }) => {
  const Tokens = IDL.Record({ e8s: IDL.Nat64 });
  const AccountIdentifierDFX = IDL.Text;
  const Duration = IDL.Record({ secs: IDL.Nat64, nanos: IDL.Nat32 });
  const ArchiveOptions = IDL.Record({
    num_blocks_to_archive: IDL.Nat64,
    trigger_threshold: IDL.Nat64,
    max_message_size_bytes: IDL.Opt(IDL.Nat64),
    cycles_for_archive_creation: IDL.Opt(IDL.Nat64),
    node_max_memory_size_bytes: IDL.Opt(IDL.Nat64),
    controller_id: IDL.Principal,
  });
  const LedgerCanisterInitPayload = IDL.Record({
    send_whitelist: IDL.Vec(IDL.Principal),
    admin: IDL.Principal,
    token_symbol: IDL.Opt(IDL.Text),
    transfer_fee: IDL.Opt(Tokens),
    minting_account: AccountIdentifierDFX,
    transaction_window: IDL.Opt(Duration),
    max_message_size_bytes: IDL.Opt(IDL.Nat64),
    archive_options: IDL.Opt(ArchiveOptions),
    standard_whitelist: IDL.Vec(IDL.Principal),
    initial_values: IDL.Vec(IDL.Tuple(AccountIdentifierDFX, Tokens)),
    token_name: IDL.Opt(IDL.Text),
  });
  const AccountIdentifier = IDL.Vec(IDL.Nat8);
  const AccountBalanceArgs = IDL.Record({ account: AccountIdentifier });
  const AccountBalanceArgsDFX = IDL.Record({
    account: AccountIdentifierDFX,
  });
  const Archive = IDL.Record({ canister_id: IDL.Principal });
  const Archives = IDL.Record({ archives: IDL.Vec(Archive) });
  const BlockHeight = IDL.Nat64;
  const BlockArg = BlockHeight;
  const Memo = IDL.Nat64;
  const OperationBlock = IDL.Variant({
    Burn: IDL.Record({ from: AccountIdentifierDFX, amount: Tokens }),
    Mint: IDL.Record({ to: AccountIdentifierDFX, amount: Tokens }),
    Transfer: IDL.Record({
      to: AccountIdentifierDFX,
      fee: Tokens,
      from: AccountIdentifierDFX,
      amount: Tokens,
    }),
  });
  const TimeStamp = IDL.Record({ timestamp_nanos: IDL.Nat64 });
  const TransactionDFX = IDL.Record({
    memo: Memo,
    operation: IDL.Opt(OperationBlock),
    created_at_time: TimeStamp,
  });
  const BlockDFX = IDL.Record({
    transaction: TransactionDFX,
    timestamp: TimeStamp,
    parent_hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CanisterId = IDL.Principal;
  const BlockResDFX = IDL.Opt(
    IDL.Variant({
      Ok: IDL.Opt(IDL.Variant({ Ok: BlockDFX, Err: CanisterId })),
      Err: IDL.Text,
    })
  );
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    url: IDL.Text,
    method: IDL.Text,
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(HeaderField),
  });
  const HttpResponse = IDL.Record({
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(HeaderField),
    status_code: IDL.Nat16,
  });
  const SubAccount = IDL.Vec(IDL.Nat8);
  const NotifyCanisterArgs = IDL.Record({
    to_subaccount: IDL.Opt(SubAccount),
    from_subaccount: IDL.Opt(SubAccount),
    to_canister: IDL.Principal,
    max_fee: Tokens,
    block_height: BlockHeight,
  });
  const BlockIndex = IDL.Nat64;
  const GetBlocksArgs = IDL.Record({
    start: BlockIndex,
    length: IDL.Nat64,
  });
  const Operation = IDL.Variant({
    Burn: IDL.Record({ from: AccountIdentifier, amount: Tokens }),
    Mint: IDL.Record({ to: AccountIdentifier, amount: Tokens }),
    Transfer: IDL.Record({
      to: AccountIdentifier,
      fee: Tokens,
      from: AccountIdentifier,
      amount: Tokens,
    }),
  });
  const Transaction = IDL.Record({
    memo: Memo,
    operation: IDL.Opt(Operation),
    created_at_time: TimeStamp,
  });
  const Block = IDL.Record({
    transaction: Transaction,
    timestamp: TimeStamp,
    parent_hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BlockRange = IDL.Record({ blocks: IDL.Vec(Block) });
  const QueryArchiveError = IDL.Variant({
    BadFirstBlockIndex: IDL.Record({
      requested_index: BlockIndex,
      first_valid_index: BlockIndex,
    }),
    Other: IDL.Record({
      error_message: IDL.Text,
      error_code: IDL.Nat64,
    }),
  });
  const QueryArchiveResult = IDL.Variant({
    Ok: BlockRange,
    Err: QueryArchiveError,
  });
  const QueryArchiveFn = IDL.Func(
    [GetBlocksArgs],
    [QueryArchiveResult],
    ["query"]
  );
  const QueryBlocksResponse = IDL.Record({
    certificate: IDL.Opt(IDL.Vec(IDL.Nat8)),
    blocks: IDL.Vec(Block),
    chain_length: IDL.Nat64,
    first_block_index: BlockIndex,
    archived_blocks: IDL.Vec(
      IDL.Record({
        callback: QueryArchiveFn,
        start: BlockIndex,
        length: IDL.Nat64,
      })
    ),
  });
  const SendArgs = IDL.Record({
    to: AccountIdentifierDFX,
    fee: Tokens,
    memo: Memo,
    from_subaccount: IDL.Opt(SubAccount),
    created_at_time: IDL.Opt(TimeStamp),
    amount: Tokens,
  });
  const TipOfChainRes = IDL.Record({
    certification: IDL.Opt(IDL.Vec(IDL.Nat8)),
    tip_index: BlockHeight,
  });
  const TransferArgs = IDL.Record({
    to: AccountIdentifier,
    fee: Tokens,
    memo: Memo,
    from_subaccount: IDL.Opt(SubAccount),
    created_at_time: IDL.Opt(TimeStamp),
    amount: Tokens,
  });
  const TransferError = IDL.Variant({
    TxTooOld: IDL.Record({ allowed_window_nanos: IDL.Nat64 }),
    BadFee: IDL.Record({ expected_fee: Tokens }),
    TxDuplicate: IDL.Record({ duplicate_of: BlockIndex }),
    TxCreatedInFuture: IDL.Null,
    InsufficientFunds: IDL.Record({ balance: Tokens }),
  });
  const TransferResult = IDL.Variant({
    Ok: BlockIndex,
    Err: TransferError,
  });
  const TransferFeeArg = IDL.Record({});
  const TransferFee = IDL.Record({ transfer_fee: Tokens });
  const TransferStandardArgs = IDL.Record({
    to: AccountIdentifier,
    fee: Tokens,
    memo: Memo,
    from_subaccount: IDL.Opt(SubAccount),
    from_principal: IDL.Principal,
    created_at_time: IDL.Opt(TimeStamp),
    amount: Tokens,
  });
  return IDL.Service({
    account_balance: IDL.Func([AccountBalanceArgs], [Tokens], ["query"]),
    account_balance_dfx: IDL.Func([AccountBalanceArgsDFX], [Tokens], ["query"]),
    archives: IDL.Func([], [Archives], ["query"]),
    block_dfx: IDL.Func([BlockArg], [BlockResDFX], ["query"]),
    decimals: IDL.Func([], [IDL.Record({ decimals: IDL.Nat32 })], ["query"]),
    get_admin_dfx: IDL.Func([IDL.Record({})], [IDL.Principal], ["query"]),
    get_minting_account_id_dfx: IDL.Func(
      [IDL.Record({})],
      [IDL.Opt(AccountIdentifierDFX)],
      ["query"]
    ),
    get_nodes: IDL.Func([], [IDL.Vec(CanisterId)], ["query"]),
    get_send_whitelist_dfx: IDL.Func(
      [IDL.Record({})],
      [IDL.Vec(IDL.Principal)],
      ["query"]
    ),
    get_standard_whitelist_dfx: IDL.Func(
      [IDL.Record({})],
      [IDL.Vec(IDL.Principal)],
      ["query"]
    ),
    http_request: IDL.Func([HttpRequest], [HttpResponse], ["query"]),
    name: IDL.Func([], [IDL.Record({ name: IDL.Text })], ["query"]),
    notify_dfx: IDL.Func([NotifyCanisterArgs], [], []),
    query_blocks: IDL.Func([GetBlocksArgs], [QueryBlocksResponse], ["query"]),
    send_dfx: IDL.Func([SendArgs], [BlockHeight], []),
    set_admin_dfx: IDL.Func([IDL.Principal], [], []),
    set_minting_account_id_dfx: IDL.Func([AccountIdentifierDFX], [], []),
    set_send_whitelist_dfx: IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    set_standard_whitelist_dfx: IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    symbol: IDL.Func([], [IDL.Record({ symbol: IDL.Text })], ["query"]),
    tip_of_chain_dfx: IDL.Func([IDL.Record({})], [TipOfChainRes], ["query"]),
    total_supply_dfx: IDL.Func([IDL.Record({})], [Tokens], ["query"]),
    transfer: IDL.Func([TransferArgs], [TransferResult], []),
    transfer_fee: IDL.Func([TransferFeeArg], [TransferFee], ["query"]),
    transfer_standard_stdldg: IDL.Func(
      [TransferStandardArgs],
      [TransferResult],
      []
    ),
    update_archive_option: IDL.Func([ArchiveOptions], [], []),
  });
};
export const init = ({ IDL }) => {
  const Tokens = IDL.Record({ e8s: IDL.Nat64 });
  const AccountIdentifierDFX = IDL.Text;
  const Duration = IDL.Record({ secs: IDL.Nat64, nanos: IDL.Nat32 });
  const ArchiveOptions = IDL.Record({
    num_blocks_to_archive: IDL.Nat64,
    trigger_threshold: IDL.Nat64,
    max_message_size_bytes: IDL.Opt(IDL.Nat64),
    cycles_for_archive_creation: IDL.Opt(IDL.Nat64),
    node_max_memory_size_bytes: IDL.Opt(IDL.Nat64),
    controller_id: IDL.Principal,
  });
  const LedgerCanisterInitPayload = IDL.Record({
    send_whitelist: IDL.Vec(IDL.Principal),
    admin: IDL.Principal,
    token_symbol: IDL.Opt(IDL.Text),
    transfer_fee: IDL.Opt(Tokens),
    minting_account: AccountIdentifierDFX,
    transaction_window: IDL.Opt(Duration),
    max_message_size_bytes: IDL.Opt(IDL.Nat64),
    archive_options: IDL.Opt(ArchiveOptions),
    standard_whitelist: IDL.Vec(IDL.Principal),
    initial_values: IDL.Vec(IDL.Tuple(AccountIdentifierDFX, Tokens)),
    token_name: IDL.Opt(IDL.Text),
  });
  return [LedgerCanisterInitPayload];
};
