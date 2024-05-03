// eslint-disable-next-line import/no-anonymous-default-export
export default ({ IDL }) => {
  const List = IDL.Rec();
  const List_1 = IDL.Rec();
  const Amount = IDL.Nat;
  const Time = IDL.Int;
  const IDOSetting = IDL.Record({
    IDOSupplies: IDL.Vec(IDL.Record({ supply: Amount, price: IDL.Nat })),
    IDOWhitelistEnabled: IDL.Bool,
    IDOOpeningTime: Time,
    IDOTotalSupply: IDL.Record({
      IDOSupply: Amount,
      percentageOfTotal: IDL.Float64,
    }),
    IDOEnabled: IDL.Bool,
    IDOLimitPerAccount: Amount,
    IDOClosingTime: Time,
  });
  const IDORequirement = IDL.Record({
    threshold: IDL.Float64,
    pairs: IDL.Vec(IDL.Record({ pair: IDL.Principal, token1ToUsdRatio: IDL.Float64 })),
  });
  const Address = IDL.Text;
  const Participant = IDL.Record({
    updatedTime: Time,
    used: Amount,
    limit: Amount,
    historyVol: IDL.Float64,
  });
  const Sa = IDL.Vec(IDL.Nat8);
  const Amount__1 = IDL.Nat;
  const KeepingBalance = IDL.Record({
    token0: IDL.Record({ locked: Amount__1, available: Amount__1 }),
    token1: IDL.Record({ locked: Amount__1, available: Amount__1 }),
  });
  const Nonce__2 = IDL.Nat;
  const AccountSetting__1 = IDL.Record({
    enPoolMode: IDL.Bool,
    modeSwitchHistory: IDL.Vec(IDL.Tuple(Nonce__2, Nonce__2)),
    start: IDL.Opt(Nonce__2),
    enKeepingBalance: IDL.Bool,
  });
  const BackupRequest = IDL.Variant({
    icdex_keepingBalances: IDL.Null,
    timeSortedTxids: IDL.Null,
    icdex_failedOrders: IDL.Null,
    icdex_makers: IDL.Null,
    icdex_stOrderTxids: IDL.Null,
    icdex_poolBalance: IDL.Null,
    drc205Data: IDL.Variant({ All: IDL.Null, Base: IDL.Null }),
    icdex_vols: IDL.Null,
    icdex_accountSettings: IDL.Null,
    icdex_pendingOrders: IDL.Null,
    icdex_userStopLossOrderList: IDL.Null,
    otherData: IDL.Null,
    icdex_dip20Balances: IDL.Null,
    clearingTxids: IDL.Null,
    icdex_nonces: IDL.Null,
    icdex_userProOrderList: IDL.Null,
    icdex_sto: IDL.Null,
    icdex_klines2: IDL.Null,
    sagaData: IDL.Variant({ All: IDL.Null, Base: IDL.Null }),
    icdex_orders: IDL.Null,
    icdex_orderBook: IDL.Null,
    traderReferrers: IDL.Null,
    icdex_RPCAccounts: IDL.Null,
    traderReferrerTemps: IDL.Null,
    ambassadors: IDL.Null,
    icdex_stOrderRecords: IDL.Null,
    ictc_admins: IDL.Null,
  });
  const AccountId__3 = IDL.Vec(IDL.Nat8);
  const KeepingBalance__1 = IDL.Record({
    token0: IDL.Record({ locked: Amount__1, available: Amount__1 }),
    token1: IDL.Record({ locked: Amount__1, available: Amount__1 }),
  });
  const Txid__4 = IDL.Vec(IDL.Nat8);
  const TradingStatus = IDL.Variant({
    Todo: IDL.Null,
    Closed: IDL.Null,
    Cancelled: IDL.Null,
    Pending: IDL.Null,
  });
  const Toid = IDL.Nat;
  const Txid = IDL.Vec(IDL.Nat8);
  const OrderType__1 = IDL.Variant({
    FAK: IDL.Null,
    FOK: IDL.Null,
    LMT: IDL.Null,
    MKT: IDL.Null,
  });
  const BalanceChange = IDL.Variant({
    DebitRecord: IDL.Nat,
    CreditRecord: IDL.Nat,
    NoChange: IDL.Null,
  });
  const OrderFilled = IDL.Record({
    time: Time,
    token0Value: BalanceChange,
    counterparty: Txid,
    token1Value: BalanceChange,
  });
  const AccountId__2 = IDL.Vec(IDL.Nat8);
  const Quantity = IDL.Nat;
  const Price = IDL.Nat;
  const OrderPrice__1 = IDL.Record({
    quantity: IDL.Variant({
      Buy: IDL.Tuple(Quantity, Amount__1),
      Sell: Quantity,
    }),
    price: Price,
  });
  const TradingOrder = IDL.Record({
    fee: IDL.Record({ fee0: IDL.Int, fee1: IDL.Int }),
    gas: IDL.Record({ gas0: IDL.Nat, gas1: IDL.Nat }),
    status: TradingStatus,
    toids: IDL.Vec(Toid),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    time: Time,
    txid: Txid,
    icrc1Account: IDL.Opt(
      IDL.Record({
        owner: IDL.Principal,
        subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
      })
    ),
    orderType: OrderType__1,
    filled: IDL.Vec(OrderFilled),
    expiration: Time,
    nonce: IDL.Nat,
    account: AccountId__2,
    remaining: OrderPrice__1,
    index: IDL.Nat,
    orderPrice: OrderPrice__1,
    refund: IDL.Tuple(IDL.Nat, IDL.Nat, IDL.Nat),
  });
  const Txid__2 = IDL.Vec(IDL.Nat8);
  const Soid__1 = IDL.Nat;
  const Amount__3 = IDL.Nat;
  const Setting__1 = IDL.Record({
    MAX_STORAGE_TRIES: IDL.Nat,
    EN_DEBUG: IDL.Bool,
    MAX_CACHE_NUMBER_PER: IDL.Nat,
    MAX_CACHE_TIME: IDL.Nat,
  });
  const Status = IDL.Variant({
    Failed: IDL.Null,
    Cancelled: IDL.Null,
    PartiallyCompletedAndCancelled: IDL.Null,
    Completed: IDL.Null,
    Pending: IDL.Null,
  });
  const Shares = IDL.Nat;
  const ShareChange = IDL.Variant({
    Burn: Shares,
    Mint: Shares,
    NoChange: IDL.Null,
  });
  const BalanceChange__1 = IDL.Variant({
    DebitRecord: IDL.Nat,
    CreditRecord: IDL.Nat,
    NoChange: IDL.Null,
  });
  const Data__1 = IDL.Vec(IDL.Nat8);
  const Txid__3 = IDL.Vec(IDL.Nat8);
  const TokenType__1 = IDL.Variant({
    Icp: IDL.Null,
    Token: IDL.Principal,
    Cycles: IDL.Null,
  });
  const Nonce__1 = IDL.Nat;
  const OperationType = IDL.Variant({
    AddLiquidity: IDL.Null,
    Swap: IDL.Null,
    Claim: IDL.Null,
    RemoveLiquidity: IDL.Null,
  });
  const AccountId__1 = IDL.Vec(IDL.Nat8);
  const CyclesWallet = IDL.Principal;
  const TxnRecord__1 = IDL.Record({
    fee: IDL.Record({ token0Fee: IDL.Int, token1Fee: IDL.Int }),
    status: Status,
    shares: ShareChange,
    msgCaller: IDL.Opt(IDL.Principal),
    order: IDL.Record({
      token0Value: IDL.Opt(BalanceChange__1),
      token1Value: IDL.Opt(BalanceChange__1),
    }),
    data: IDL.Opt(Data__1),
    time: Time,
    txid: Txid__3,
    orderMode: IDL.Variant({ AMM: IDL.Null, OrderBook: IDL.Null }),
    orderType: IDL.Opt(
      IDL.Variant({
        FAK: IDL.Null,
        FOK: IDL.Null,
        LMT: IDL.Null,
        MKT: IDL.Null,
      })
    ),
    filled: IDL.Record({
      token0Value: BalanceChange__1,
      token1Value: BalanceChange__1,
    }),
    token0: TokenType__1,
    token1: TokenType__1,
    nonce: Nonce__1,
    operation: OperationType,
    account: AccountId__1,
    details: IDL.Vec(
      IDL.Record({
        time: Time,
        token0Value: BalanceChange__1,
        counterparty: Txid__3,
        token1Value: BalanceChange__1,
      })
    ),
    caller: AccountId__1,
    index: IDL.Nat,
    cyclesWallet: IDL.Opt(CyclesWallet),
  });
  const DRC205Data = IDL.Record({
    accountLastTxns: IDL.Vec(
      IDL.Tuple(AccountId__3, IDL.Tuple(IDL.Vec(Txid__4), IDL.Vec(Txid__4)))
    ),
    setting: Setting__1,
    globalLastTxns: IDL.Tuple(IDL.Vec(Txid__4), IDL.Vec(Txid__4)),
    storagePool: IDL.Vec(IDL.Tuple(Txid__4, TxnRecord__1, IDL.Nat)),
    globalTxns: IDL.Tuple(IDL.Vec(IDL.Tuple(Txid__4, Time)), IDL.Vec(IDL.Tuple(Txid__4, Time))),
    txnRecords: IDL.Vec(IDL.Tuple(Txid__4, TxnRecord__1)),
  });
  const Vol__1 = IDL.Record({ value0: Amount__1, value1: Amount__1 });
  const AccountSetting = IDL.Record({
    enPoolMode: IDL.Bool,
    modeSwitchHistory: IDL.Vec(IDL.Tuple(Nonce__2, Nonce__2)),
    start: IDL.Opt(Nonce__2),
    enKeepingBalance: IDL.Bool,
  });
  const FeeBalance = IDL.Record({ value0: Amount__1, value1: Amount__1 });
  const Timestamp__2 = IDL.Nat;
  const PriceWeighted = IDL.Record({
    updateTime: Timestamp__2,
    token1TimeWeighted: IDL.Nat,
    token0TimeWeighted: IDL.Nat,
  });
  const Nonce__3 = IDL.Nat;
  const Price__2 = IDL.Nat;
  const KInterval = IDL.Nat;
  const Vol__2 = IDL.Record({ value0: IDL.Nat, value1: IDL.Nat });
  const Timestamp__1 = IDL.Nat;
  const KBar = IDL.Record({
    kid: IDL.Nat,
    low: IDL.Nat,
    vol: Vol__2,
    high: IDL.Nat,
    close: IDL.Nat,
    open: IDL.Nat,
    updatedTs: Timestamp__1,
  });
  const Toid__2 = IDL.Nat;
  const OrderStatus = IDL.Variant({
    Done: IDL.Null,
    Todo: IDL.Null,
    Doing: IDL.Null,
    Recovered: IDL.Null,
    Compensating: IDL.Null,
    Blocking: IDL.Null,
  });
  const Status__3 = IDL.Variant({
    Error: IDL.Null,
    Done: IDL.Null,
    Todo: IDL.Null,
    Doing: IDL.Null,
    Unknown: IDL.Null,
  });
  const Ttid = IDL.Nat;
  const Toid__1 = IDL.Nat;
  const Sa__2 = IDL.Vec(IDL.Nat8);
  const Subaccount = IDL.Vec(IDL.Nat8);
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(Subaccount),
  });
  const Timestamp__3 = IDL.Nat64;
  const TransferArgs = IDL.Record({
    to: Account,
    fee: IDL.Opt(IDL.Nat),
    memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
    from_subaccount: IDL.Opt(Subaccount),
    created_at_time: IDL.Opt(Timestamp__3),
    amount: IDL.Nat,
  });
  const STStatus__1 = IDL.Variant({
    Stopped: IDL.Null,
    Running: IDL.Null,
    Deleted: IDL.Null,
  });
  const Ppm__1 = IDL.Nat;
  const Sa__3 = IDL.Vec(IDL.Nat8);
  const To = IDL.Text;
  const Amount__4 = IDL.Nat;
  const Nonce__4 = IDL.Nat;
  const Sa__1 = IDL.Vec(IDL.Nat8);
  const Data__2 = IDL.Vec(IDL.Nat8);
  const From = IDL.Text;
  const Spender = IDL.Text;
  const Address__1 = IDL.Text;
  const BlobFill = IDL.Variant({
    AutoFill: IDL.Null,
    ManualFill: IDL.Vec(IDL.Nat8),
  });
  const Timeout = IDL.Nat32;
  const Decider = IDL.Text;
  const ExecuteType = IDL.Variant({
    fallback: IDL.Null,
    send: IDL.Nat,
    sendAll: IDL.Null,
  });
  const Subaccount__1 = IDL.Vec(IDL.Nat8);
  const Account__1 = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(Subaccount__1),
  });
  const Timestamp__4 = IDL.Nat64;
  const TransferArgs__1 = IDL.Record({
    to: Account__1,
    fee: IDL.Opt(IDL.Nat),
    memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
    from_subaccount: IDL.Opt(Subaccount__1),
    created_at_time: IDL.Opt(Timestamp__4),
    amount: IDL.Nat,
  });
  const CallType = IDL.Variant({
    __block: IDL.Null,
    ICDex: IDL.Variant({
      withdraw: IDL.Tuple(IDL.Opt(Amount__1), IDL.Opt(Amount__1), IDL.Opt(Sa__2)),
      depositFallback: IDL.Opt(Sa__2),
      deposit: IDL.Tuple(
        IDL.Variant({ token0: IDL.Null, token1: IDL.Null }),
        IDL.Nat,
        IDL.Opt(Sa__2)
      ),
    }),
    ICRC1: IDL.Variant({
      icrc1_balance_of: Account,
      icrc1_transfer: TransferArgs,
    }),
    This: IDL.Variant({
      updatePoolLocalBalance: IDL.Tuple(
        IDL.Opt(IDL.Variant({ add: IDL.Nat, set: IDL.Nat, sub: IDL.Nat })),
        IDL.Opt(IDL.Variant({ add: IDL.Nat, set: IDL.Nat, sub: IDL.Nat }))
      ),
      batchTransfer: IDL.Vec(
        IDL.Tuple(
          IDL.Variant({ add: IDL.Null, sub: IDL.Null }),
          IDL.Vec(IDL.Nat8),
          IDL.Variant({ token0: IDL.Null, token1: IDL.Null }),
          IDL.Variant({ locked: IDL.Nat, available: IDL.Nat })
        )
      ),
      dexDepositFallback: IDL.Tuple(IDL.Principal, IDL.Opt(IDL.Vec(IDL.Nat8))),
    }),
    StratOrder: IDL.Variant({
      sto_updateProOrder: IDL.Tuple(
        Soid__1,
        IDL.Variant({
          GridOrder: IDL.Record({
            status: IDL.Opt(STStatus__1),
            lowerLimit: IDL.Opt(Price__2),
            upperLimit: IDL.Opt(Price__2),
            spread: IDL.Opt(IDL.Variant({ Geom: Ppm__1, Arith: Price__2 })),
            amount: IDL.Opt(
              IDL.Variant({
                Percent: IDL.Opt(Ppm__1),
                Token0: IDL.Nat,
                Token1: IDL.Nat,
              })
            ),
          }),
        }),
        IDL.Opt(Sa__3)
      ),
      sto_createProOrder: IDL.Tuple(
        IDL.Variant({
          GridOrder: IDL.Record({
            lowerLimit: Price__2,
            upperLimit: Price__2,
            spread: IDL.Variant({ Geom: Ppm__1, Arith: Price__2 }),
            amount: IDL.Variant({
              Percent: IDL.Opt(Ppm__1),
              Token0: IDL.Nat,
              Token1: IDL.Nat,
            }),
          }),
        }),
        IDL.Opt(Sa__3)
      ),
      sto_cancelPendingOrders: IDL.Tuple(Soid__1, IDL.Opt(Sa__3)),
    }),
    DRC20: IDL.Variant({
      transferBatch: IDL.Tuple(
        IDL.Vec(To),
        IDL.Vec(Amount__4),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      transferFrom: IDL.Tuple(
        From,
        To,
        Amount__4,
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      approve: IDL.Tuple(Spender, Amount__4, IDL.Opt(Nonce__4), IDL.Opt(Sa__1), IDL.Opt(Data__2)),
      balanceOf: Address__1,
      txnRecord: BlobFill,
      lockTransfer: IDL.Tuple(
        To,
        Amount__4,
        Timeout,
        IDL.Opt(Decider),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      transfer: IDL.Tuple(To, Amount__4, IDL.Opt(Nonce__4), IDL.Opt(Sa__1), IDL.Opt(Data__2)),
      dropAccount: IDL.Opt(Sa__1),
      executeTransfer: IDL.Tuple(
        BlobFill,
        ExecuteType,
        IDL.Opt(To),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      lockTransferFrom: IDL.Tuple(
        From,
        To,
        Amount__4,
        Timeout,
        IDL.Opt(Decider),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
    }),
    ICRC1New: IDL.Variant({
      icrc1_balance_of: Account__1,
      icrc1_transfer: TransferArgs__1,
    }),
    __skip: IDL.Null,
  });
  const Attempts = IDL.Nat;
  const Callee__1 = IDL.Principal;
  const Compensation = IDL.Record({
    preTtid: IDL.Vec(Ttid),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    time: Time,
    toid: IDL.Opt(Toid__1),
    callType: CallType,
    cycles: IDL.Nat,
    recallInterval: IDL.Int,
    attemptsMax: Attempts,
    callee: Callee__1,
    forTtid: IDL.Opt(Ttid),
  });
  const Task__2 = IDL.Record({
    preTtid: IDL.Vec(Ttid),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    time: Time,
    toid: IDL.Opt(Toid__1),
    callType: CallType,
    cycles: IDL.Nat,
    recallInterval: IDL.Int,
    attemptsMax: Attempts,
    callee: Callee__1,
    forTtid: IDL.Opt(Ttid),
  });
  const Ttid__2 = IDL.Nat;
  const SagaTask = IDL.Record({
    status: Status__3,
    comp: IDL.Opt(Compensation),
    task: Task__2,
    ttid: Ttid__2,
  });
  List_1.fill(IDL.Opt(IDL.Tuple(SagaTask, List_1)));
  const Tcid = IDL.Nat;
  const CompTask = IDL.Record({
    status: Status__3,
    comp: Compensation,
    tcid: Tcid,
    forTtid: Ttid__2,
  });
  List.fill(IDL.Opt(IDL.Tuple(CompTask, List)));
  const CompStrategy = IDL.Variant({
    Backward: IDL.Null,
    Forward: IDL.Null,
  });
  const Order = IDL.Record({
    status: OrderStatus,
    tasks: List_1,
    callbackStatus: IDL.Opt(Status__3),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    name: IDL.Text,
    comps: List,
    time: Time,
    compStrategy: CompStrategy,
    allowPushing: IDL.Variant({ Opening: IDL.Null, Closed: IDL.Null }),
  });
  const Ttid__1 = IDL.Nat;
  const Task__1 = IDL.Record({
    preTtid: IDL.Vec(Ttid),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    time: Time,
    toid: IDL.Opt(Toid__1),
    callType: CallType,
    cycles: IDL.Nat,
    recallInterval: IDL.Int,
    attemptsMax: Attempts,
    callee: Callee__1,
    forTtid: IDL.Opt(Ttid),
  });
  const Callee = IDL.Principal;
  const CalleeStatus = IDL.Record({
    continuousFailure: IDL.Nat,
    successCount: IDL.Nat,
    failureCount: IDL.Nat,
  });
  const Status__1 = IDL.Variant({
    Error: IDL.Null,
    Done: IDL.Null,
    Todo: IDL.Null,
    Doing: IDL.Null,
    Unknown: IDL.Null,
  });
  const Duration = IDL.Nat64;
  const TransferError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    TemporarilyUnavailable: IDL.Null,
    BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }),
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    BadFee: IDL.Record({ expected_fee: IDL.Nat }),
    CreatedInFuture: IDL.Null,
    TooOld: IDL.Record({ allowed_window_nanos: Duration }),
    InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
  });
  const Txid__5 = IDL.Vec(IDL.Nat8);
  const TxnResult = IDL.Variant({
    ok: Txid__5,
    err: IDL.Record({
      code: IDL.Variant({
        NonceError: IDL.Null,
        InsufficientGas: IDL.Null,
        InsufficientAllowance: IDL.Null,
        UndefinedError: IDL.Null,
        InsufficientBalance: IDL.Null,
        NoLockedTransfer: IDL.Null,
        DuplicateExecutedTransfer: IDL.Null,
        LockedTransferExpired: IDL.Null,
      }),
      message: IDL.Text,
    }),
  });
  const Gas = IDL.Variant({
    token: IDL.Nat,
    cycles: IDL.Nat,
    noFee: IDL.Null,
  });
  const AccountId__4 = IDL.Vec(IDL.Nat8);
  const Time__1 = IDL.Int;
  const Operation = IDL.Variant({
    approve: IDL.Record({ allowance: IDL.Nat }),
    lockTransfer: IDL.Record({
      locked: IDL.Nat,
      expiration: Time__1,
      decider: AccountId__4,
    }),
    transfer: IDL.Record({
      action: IDL.Variant({
        burn: IDL.Null,
        mint: IDL.Null,
        send: IDL.Null,
      }),
    }),
    executeTransfer: IDL.Record({
      fallback: IDL.Nat,
      lockedTxid: Txid__5,
    }),
  });
  const Transaction = IDL.Record({
    to: AccountId__4,
    value: IDL.Nat,
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    from: AccountId__4,
    operation: Operation,
  });
  const TxnRecord__2 = IDL.Record({
    gas: Gas,
    msgCaller: IDL.Opt(IDL.Principal),
    transaction: Transaction,
    txid: Txid__5,
    nonce: IDL.Nat,
    timestamp: Time__1,
    caller: AccountId__4,
    index: IDL.Nat,
  });
  const TransferError__1 = IDL.Variant({
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
  const Receipt = IDL.Variant({
    __block: IDL.Null,
    ICDex: IDL.Variant({
      withdraw: IDL.Tuple(IDL.Nat, IDL.Nat),
      depositFallback: IDL.Tuple(IDL.Nat, IDL.Nat),
      deposit: IDL.Null,
    }),
    ICRC1: IDL.Variant({
      icrc1_balance_of: IDL.Nat,
      icrc1_transfer: IDL.Variant({ Ok: IDL.Nat, Err: TransferError }),
    }),
    This: IDL.Variant({
      updatePoolLocalBalance: IDL.Record({
        ts: IDL.Nat,
        balance0: IDL.Nat,
        balance1: IDL.Nat,
      }),
      batchTransfer: IDL.Vec(
        IDL.Record({
          token0: IDL.Record({ locked: IDL.Nat, available: IDL.Nat }),
          token1: IDL.Record({ locked: IDL.Nat, available: IDL.Nat }),
        })
      ),
      dexDepositFallback: IDL.Tuple(IDL.Nat, IDL.Nat),
    }),
    StratOrder: IDL.Variant({
      sto_updateProOrder: Soid__1,
      sto_createProOrder: Soid__1,
      sto_cancelPendingOrders: IDL.Null,
    }),
    DRC20: IDL.Variant({
      transferBatch: IDL.Vec(TxnResult),
      transferFrom: TxnResult,
      approve: TxnResult,
      balanceOf: Amount__4,
      txnRecord: IDL.Opt(TxnRecord__2),
      lockTransfer: TxnResult,
      transfer: TxnResult,
      dropAccount: IDL.Null,
      executeTransfer: TxnResult,
      lockTransferFrom: TxnResult,
    }),
    ICRC1New: IDL.Variant({
      icrc1_balance_of: IDL.Nat,
      icrc1_transfer: IDL.Variant({
        Ok: IDL.Nat,
        Err: TransferError__1,
      }),
    }),
    __skip: IDL.Null,
  });
  const ErrorCode = IDL.Variant({
    canister_error: IDL.Null,
    call_error: IDL.Record({ err_code: IDL.Nat32 }),
    system_transient: IDL.Null,
    future: IDL.Nat32,
    canister_reject: IDL.Null,
    destination_invalid: IDL.Null,
    system_fatal: IDL.Null,
  });
  const Err = IDL.Record({ code: ErrorCode, message: IDL.Text });
  const TaskResult = IDL.Tuple(Status__1, IDL.Opt(Receipt), IDL.Opt(Err));
  const ErrorLog = IDL.Record({
    result: IDL.Opt(TaskResult),
    time: Time,
    ttid: Ttid,
    callee: IDL.Opt(Callee__1),
  });
  const Status__2 = IDL.Variant({
    Error: IDL.Null,
    Done: IDL.Null,
    Todo: IDL.Null,
    Doing: IDL.Null,
    Unknown: IDL.Null,
  });
  const Task = IDL.Record({
    preTtid: IDL.Vec(Ttid),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    time: Time,
    toid: IDL.Opt(Toid__1),
    callType: CallType,
    cycles: IDL.Nat,
    recallInterval: IDL.Int,
    attemptsMax: Attempts,
    callee: Callee__1,
    forTtid: IDL.Opt(Ttid),
  });
  const TaskEvent = IDL.Record({
    result: TaskResult,
    callbackStatus: IDL.Opt(Status__2),
    task: Task,
    time: Time,
    toid: IDL.Opt(Toid__1),
    ttid: Ttid,
    attempts: Attempts,
    txHash: IDL.Vec(IDL.Nat8),
  });
  const SagaData = IDL.Record({
    orders: IDL.Vec(IDL.Tuple(Toid__2, Order)),
    autoClearTimeout: IDL.Int,
    aliveOrders: IDL.Vec(IDL.Tuple(Toid__2, Time)),
    index: IDL.Nat,
    actuator: IDL.Record({
      tasks: IDL.Tuple(IDL.Vec(IDL.Tuple(Ttid__1, Task__1)), IDL.Vec(IDL.Tuple(Ttid__1, Task__1))),
      callees: IDL.Vec(IDL.Tuple(Callee, CalleeStatus)),
      errorLogs: IDL.Vec(IDL.Tuple(IDL.Nat, ErrorLog)),
      taskLogs: IDL.Vec(IDL.Tuple(Ttid__1, TaskEvent)),
      index: IDL.Nat,
      firstErrIndex: IDL.Nat,
      errIndex: IDL.Nat,
      firstIndex: IDL.Nat,
    }),
    taskEvents: IDL.Vec(IDL.Tuple(Toid__2, IDL.Vec(Ttid__1))),
    firstIndex: IDL.Nat,
  });
  const AmbassadorData = IDL.Tuple(IDL.Text, IDL.Nat, Vol__1);
  const Timestamp = IDL.Nat;
  const OrderSide = IDL.Variant({ Buy: IDL.Null, Sell: IDL.Null });
  const TWAPSetting__1 = IDL.Record({
    endTime: Timestamp,
    startingTime: Timestamp,
    order: IDL.Record({
      priceLimit: Price__2,
      priceSpread: Price__2,
      side: OrderSide,
    }),
    totalLimit: IDL.Variant({ Token0: IDL.Nat, Token1: IDL.Nat }),
    amountPerTrigger: IDL.Variant({
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
    triggerInterval: IDL.Nat,
  });
  const TWAP = IDL.Record({
    setting: TWAPSetting__1,
    lastTime: IDL.Opt(Timestamp),
  });
  const VWAPSetting__1 = IDL.Record({
    endTime: Timestamp,
    startingTime: Timestamp,
    order: IDL.Record({
      priceLimit: Price__2,
      priceSpread: Price__2,
      side: OrderSide,
    }),
    totalLimit: IDL.Variant({ Token0: IDL.Nat, Token1: IDL.Nat }),
    amountPerTrigger: IDL.Variant({
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
    triggerVol: IDL.Variant({ Geom: Ppm__1, Arith: IDL.Nat }),
  });
  const VWAP = IDL.Record({
    setting: VWAPSetting__1,
    lastVol: IDL.Opt(IDL.Nat),
  });
  const IcebergOrderSetting__1 = IDL.Record({
    endTime: Timestamp,
    startingTime: Timestamp,
    order: IDL.Record({ side: OrderSide, price: Price__2 }),
    totalLimit: IDL.Variant({ Token0: IDL.Nat, Token1: IDL.Nat }),
    amountPerTrigger: IDL.Variant({
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
  });
  const IcebergOrder = IDL.Record({
    setting: IcebergOrderSetting__1,
    lastTxid: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const OrderSide__1 = IDL.Variant({ Buy: IDL.Null, Sell: IDL.Null });
  const TriggeredOrder = IDL.Record({
    order: IDL.Record({
      side: OrderSide__1,
      quantity: IDL.Nat,
      price: Price__2,
    }),
    triggerPrice: Price__2,
  });
  const Condition = IDL.Record({
    order: IDL.Record({
      side: OrderSide__1,
      quantity: IDL.Nat,
      price: Price__2,
    }),
    triggerPrice: Price__2,
  });
  const StopLossOrder = IDL.Record({
    triggeredOrder: IDL.Opt(TriggeredOrder),
    condition: Condition,
  });
  const GridSetting = IDL.Record({
    ppmFactor: IDL.Opt(IDL.Nat),
    lowerLimit: Price__2,
    upperLimit: Price__2,
    gridCountPerSide: IDL.Nat,
    spread: IDL.Variant({ Geom: Ppm__1, Arith: Price__2 }),
    amount: IDL.Variant({
      Percent: IDL.Opt(Ppm__1),
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
    initPrice: Price__2,
  });
  const Amount__2 = IDL.Nat;
  const GridPrices = IDL.Record({
    buy: IDL.Vec(Price__2),
    sell: IDL.Vec(Price__2),
    midPrice: IDL.Opt(Price__2),
  });
  const GridOrder = IDL.Record({
    setting: GridSetting,
    level1Filled: IDL.Opt(IDL.Record({ buy1: Amount__2, sell1: Amount__2 })),
    filter: IDL.Opt(
      IDL.Record({
        gridTop: Price__2,
        gridBottom: Price__2,
        buyingBlankLocked: IDL.Vec(IDL.Tuple(Price__2, Price__2)),
        sellingBlankLocked: IDL.Vec(IDL.Tuple(Price__2, Price__2)),
      })
    ),
    gridPrices: GridPrices,
  });
  const STStrategy = IDL.Variant({
    TWAP: TWAP,
    VWAP: VWAP,
    IcebergOrder: IcebergOrder,
    StopLossOrder: StopLossOrder,
    GridOrder: GridOrder,
  });
  const ICRC1Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const STStats = IDL.Record({
    totalOutAmount: IDL.Record({
      token0: Amount__2,
      token1: Amount__2,
    }),
    orderCount: IDL.Nat,
    totalInAmount: IDL.Record({
      token0: Amount__2,
      token1: Amount__2,
    }),
    errorCount: IDL.Nat,
  });
  const STType = IDL.Variant({
    TWAP: IDL.Null,
    VWAP: IDL.Null,
    IcebergOrder: IDL.Null,
    StopLossOrder: IDL.Null,
    GridOrder: IDL.Null,
  });
  const STOrder__1 = IDL.Record({
    status: STStatus__1,
    pendingOrders: IDL.Record({
      buy: IDL.Vec(IDL.Tuple(IDL.Opt(Txid__2), Price__2, IDL.Nat)),
      sell: IDL.Vec(IDL.Tuple(IDL.Opt(Txid__2), Price__2, IDL.Nat)),
    }),
    strategy: STStrategy,
    soid: Soid__1,
    initTime: Timestamp,
    icrc1Account: ICRC1Account,
    stats: STStats,
    triggerTime: Timestamp,
    stType: STType,
  });
  const BackupResponse = IDL.Variant({
    icdex_keepingBalances: IDL.Vec(IDL.Tuple(AccountId__3, KeepingBalance__1)),
    timeSortedTxids: IDL.Tuple(
      IDL.Vec(IDL.Tuple(Txid__4, Time)),
      IDL.Vec(IDL.Tuple(Txid__4, Time))
    ),
    icdex_failedOrders: IDL.Vec(IDL.Tuple(Txid__4, TradingOrder)),
    icdex_makers: IDL.Vec(IDL.Tuple(AccountId__3, IDL.Tuple(IDL.Nat, IDL.Principal))),
    icdex_stOrderTxids: IDL.Vec(IDL.Tuple(Txid__2, Soid__1)),
    icdex_poolBalance: IDL.Record({
      token0: Amount__3,
      token1: Amount__3,
    }),
    drc205Data: DRC205Data,
    icdex_vols: IDL.Vec(IDL.Tuple(AccountId__3, Vol__1)),
    icdex_accountSettings: IDL.Vec(IDL.Tuple(AccountId__3, AccountSetting)),
    icdex_pendingOrders: IDL.Vec(IDL.Tuple(AccountId__3, IDL.Vec(Txid__4))),
    icdex_userStopLossOrderList: IDL.Vec(IDL.Tuple(AccountId__3, IDL.Vec(Soid__1))),
    otherData: IDL.Record({
      icdex_totalFee: FeeBalance,
      icdex_totalVol: Vol__1,
      icdex_index: IDL.Nat,
      icdex_lastPrice: OrderPrice__1,
      icdex_priceWeighted: PriceWeighted,
      taDescription: IDL.Text,
    }),
    icdex_dip20Balances: IDL.Vec(IDL.Tuple(AccountId__3, IDL.Tuple(IDL.Principal, IDL.Nat))),
    clearingTxids: IDL.Vec(Txid__4),
    icdex_nonces: IDL.Vec(IDL.Tuple(AccountId__3, Nonce__3)),
    icdex_userProOrderList: IDL.Vec(IDL.Tuple(AccountId__3, IDL.Vec(Soid__1))),
    icdex_sto: IDL.Record({
      icdex_soid: IDL.Nat,
      icdex_activeStopLossOrderList: IDL.Record({
        buy: IDL.Vec(IDL.Tuple(Soid__1, Price__2)),
        sell: IDL.Vec(IDL.Tuple(Soid__1, Price__2)),
      }),
      icdex_activeProOrderList: IDL.Vec(Soid__1),
    }),
    icdex_klines2: IDL.Vec(IDL.Tuple(KInterval, IDL.Tuple(IDL.Vec(KBar), IDL.Vec(KBar)))),
    sagaData: SagaData,
    icdex_orders: IDL.Vec(IDL.Tuple(Txid__4, TradingOrder)),
    icdex_orderBook: IDL.Record({
      ask: IDL.Vec(IDL.Tuple(Txid__4, OrderPrice__1)),
      bid: IDL.Vec(IDL.Tuple(Txid__4, OrderPrice__1)),
    }),
    traderReferrers: IDL.Vec(IDL.Tuple(AccountId__3, AccountId__3)),
    icdex_RPCAccounts: IDL.Vec(
      IDL.Tuple(
        IDL.Text,
        IDL.Vec(
          IDL.Record({
            owner: IDL.Principal,
            subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
          })
        )
      )
    ),
    traderReferrerTemps: IDL.Vec(IDL.Tuple(AccountId__3, IDL.Tuple(AccountId__3, IDL.Text, Time))),
    ambassadors: IDL.Vec(IDL.Tuple(AccountId__3, AmbassadorData)),
    icdex_stOrderRecords: IDL.Vec(IDL.Tuple(Soid__1, STOrder__1)),
    ictc_admins: IDL.Vec(IDL.Principal),
  });
  const ListPage = IDL.Nat;
  const ListSize = IDL.Nat;
  const AccountId = IDL.Vec(IDL.Nat8);
  const Vol = IDL.Record({ value0: Amount__1, value1: Amount__1 });
  const TrieList_5 = IDL.Record({
    total: IDL.Nat,
    data: IDL.Vec(
      IDL.Tuple(
        AccountId,
        IDL.Record({
          vol: Vol,
          count: IDL.Nat,
          rate: IDL.Float64,
          commission: Vol,
        })
      )
    ),
    totalPage: IDL.Nat,
  });
  const Nonce = IDL.Nat;
  const Txid__1 = IDL.Vec(IDL.Nat8);
  const IcpE8s = IDL.Nat;
  const DexConfig = IDL.Record({
    MAX_TPS: IDL.Opt(IDL.Nat),
    ICTC_RUN_INTERVAL: IDL.Opt(IDL.Nat),
    MAKER_BONUS_RATE: IDL.Opt(IDL.Nat),
    MAX_PENDINGS: IDL.Opt(IDL.Nat),
    ORDER_EXPIRATION_DURATION: IDL.Opt(IDL.Int),
    TRADING_FEE: IDL.Opt(IDL.Nat),
    UNIT_SIZE: IDL.Opt(IDL.Nat),
    STORAGE_INTERVAL: IDL.Opt(IDL.Nat),
    ICP_FEE: IDL.Opt(IcpE8s),
  });
  const Soid = IDL.Nat;
  const ICRC1Account__1 = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const OrderPrice = IDL.Record({
    quantity: IDL.Variant({
      Buy: IDL.Tuple(Quantity, Amount__1),
      Sell: Quantity,
    }),
    price: Price,
  });
  const Config = IDL.Record({
    MAX_STORAGE_TRIES: IDL.Opt(IDL.Nat),
    EN_DEBUG: IDL.Opt(IDL.Bool),
    MAX_CACHE_NUMBER_PER: IDL.Opt(IDL.Nat),
    MAX_CACHE_TIME: IDL.Opt(IDL.Nat),
  });
  const TokenType = IDL.Variant({
    Icp: IDL.Null,
    Token: IDL.Principal,
    Cycles: IDL.Null,
  });
  const TokenStd__1 = IDL.Variant({
    dft: IDL.Null,
    ext: IDL.Null,
    icp: IDL.Null,
    other: IDL.Text,
    cycles: IDL.Null,
    ledger: IDL.Null,
    icrc1: IDL.Null,
    dip20: IDL.Null,
    drc20: IDL.Null,
  });
  const DexInfo = IDL.Record({
    feeRate: IDL.Float64,
    mmType: IDL.Variant({ AMM: IDL.Null, OrderBook: IDL.Null }),
    token0: IDL.Tuple(TokenType, TokenStd__1),
    token1: IDL.Tuple(TokenType, TokenStd__1),
    pairName: IDL.Text,
    dexName: IDL.Text,
    canisterId: IDL.Principal,
  });
  const Address__2 = IDL.Text;
  const TxnRecord__3 = IDL.Record({
    fee: IDL.Record({ token0Fee: IDL.Int, token1Fee: IDL.Int }),
    status: Status,
    shares: ShareChange,
    msgCaller: IDL.Opt(IDL.Principal),
    order: IDL.Record({
      token0Value: IDL.Opt(BalanceChange__1),
      token1Value: IDL.Opt(BalanceChange__1),
    }),
    data: IDL.Opt(Data__1),
    time: Time,
    txid: Txid__3,
    orderMode: IDL.Variant({ AMM: IDL.Null, OrderBook: IDL.Null }),
    orderType: IDL.Opt(
      IDL.Variant({
        FAK: IDL.Null,
        FOK: IDL.Null,
        LMT: IDL.Null,
        MKT: IDL.Null,
      })
    ),
    filled: IDL.Record({
      token0Value: BalanceChange__1,
      token1Value: BalanceChange__1,
    }),
    token0: TokenType__1,
    token1: TokenType__1,
    nonce: Nonce__1,
    operation: OperationType,
    account: AccountId__1,
    details: IDL.Vec(
      IDL.Record({
        time: Time,
        token0Value: BalanceChange__1,
        counterparty: Txid__3,
        token1Value: BalanceChange__1,
      })
    ),
    caller: AccountId__1,
    index: IDL.Nat,
    cyclesWallet: IDL.Opt(CyclesWallet),
  });
  const Txid__6 = IDL.Vec(IDL.Nat8);
  const DRC207Support = IDL.Record({
    timer: IDL.Record({
      interval_seconds: IDL.Opt(IDL.Nat),
      enable: IDL.Bool,
    }),
    monitorable_by_self: IDL.Bool,
    monitorable_by_blackhole: IDL.Record({
      canister_id: IDL.Opt(IDL.Principal),
      allowed: IDL.Bool,
    }),
    cycles_receivable: IDL.Bool,
  });
  const FeeStatus = IDL.Record({
    feeRate: IDL.Float64,
    feeBalance: FeeBalance,
    totalFee: FeeBalance,
  });
  const DexSetting = IDL.Record({
    MAX_TPS: IDL.Nat,
    ICTC_RUN_INTERVAL: IDL.Nat,
    MAKER_BONUS_RATE: IDL.Nat,
    MAX_PENDINGS: IDL.Nat,
    TRADING_FEE: IDL.Nat,
    UNIT_SIZE: IDL.Nat,
    STORAGE_INTERVAL: IDL.Nat,
    ICP_FEE: IcpE8s,
  });
  const TxAccount = IDL.Text;
  const Toid__4 = IDL.Nat;
  const CallType__1 = IDL.Variant({
    __block: IDL.Null,
    ICDex: IDL.Variant({
      withdraw: IDL.Tuple(IDL.Opt(Amount__1), IDL.Opt(Amount__1), IDL.Opt(Sa__2)),
      depositFallback: IDL.Opt(Sa__2),
      deposit: IDL.Tuple(
        IDL.Variant({ token0: IDL.Null, token1: IDL.Null }),
        IDL.Nat,
        IDL.Opt(Sa__2)
      ),
    }),
    ICRC1: IDL.Variant({
      icrc1_balance_of: Account,
      icrc1_transfer: TransferArgs,
    }),
    This: IDL.Variant({
      updatePoolLocalBalance: IDL.Tuple(
        IDL.Opt(IDL.Variant({ add: IDL.Nat, set: IDL.Nat, sub: IDL.Nat })),
        IDL.Opt(IDL.Variant({ add: IDL.Nat, set: IDL.Nat, sub: IDL.Nat }))
      ),
      batchTransfer: IDL.Vec(
        IDL.Tuple(
          IDL.Variant({ add: IDL.Null, sub: IDL.Null }),
          IDL.Vec(IDL.Nat8),
          IDL.Variant({ token0: IDL.Null, token1: IDL.Null }),
          IDL.Variant({ locked: IDL.Nat, available: IDL.Nat })
        )
      ),
      dexDepositFallback: IDL.Tuple(IDL.Principal, IDL.Opt(IDL.Vec(IDL.Nat8))),
    }),
    StratOrder: IDL.Variant({
      sto_updateProOrder: IDL.Tuple(
        Soid__1,
        IDL.Variant({
          GridOrder: IDL.Record({
            status: IDL.Opt(STStatus__1),
            lowerLimit: IDL.Opt(Price__2),
            upperLimit: IDL.Opt(Price__2),
            spread: IDL.Opt(IDL.Variant({ Geom: Ppm__1, Arith: Price__2 })),
            amount: IDL.Opt(
              IDL.Variant({
                Percent: IDL.Opt(Ppm__1),
                Token0: IDL.Nat,
                Token1: IDL.Nat,
              })
            ),
          }),
        }),
        IDL.Opt(Sa__3)
      ),
      sto_createProOrder: IDL.Tuple(
        IDL.Variant({
          GridOrder: IDL.Record({
            lowerLimit: Price__2,
            upperLimit: Price__2,
            spread: IDL.Variant({ Geom: Ppm__1, Arith: Price__2 }),
            amount: IDL.Variant({
              Percent: IDL.Opt(Ppm__1),
              Token0: IDL.Nat,
              Token1: IDL.Nat,
            }),
          }),
        }),
        IDL.Opt(Sa__3)
      ),
      sto_cancelPendingOrders: IDL.Tuple(Soid__1, IDL.Opt(Sa__3)),
    }),
    DRC20: IDL.Variant({
      transferBatch: IDL.Tuple(
        IDL.Vec(To),
        IDL.Vec(Amount__4),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      transferFrom: IDL.Tuple(
        From,
        To,
        Amount__4,
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      approve: IDL.Tuple(Spender, Amount__4, IDL.Opt(Nonce__4), IDL.Opt(Sa__1), IDL.Opt(Data__2)),
      balanceOf: Address__1,
      txnRecord: BlobFill,
      lockTransfer: IDL.Tuple(
        To,
        Amount__4,
        Timeout,
        IDL.Opt(Decider),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      transfer: IDL.Tuple(To, Amount__4, IDL.Opt(Nonce__4), IDL.Opt(Sa__1), IDL.Opt(Data__2)),
      dropAccount: IDL.Opt(Sa__1),
      executeTransfer: IDL.Tuple(
        BlobFill,
        ExecuteType,
        IDL.Opt(To),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
      lockTransferFrom: IDL.Tuple(
        From,
        To,
        Amount__4,
        Timeout,
        IDL.Opt(Decider),
        IDL.Opt(Nonce__4),
        IDL.Opt(Sa__1),
        IDL.Opt(Data__2)
      ),
    }),
    ICRC1New: IDL.Variant({
      icrc1_balance_of: Account__1,
      icrc1_transfer: TransferArgs__1,
    }),
    __skip: IDL.Null,
  });
  const Order__1 = IDL.Record({
    status: OrderStatus,
    tasks: List_1,
    callbackStatus: IDL.Opt(Status__3),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    name: IDL.Text,
    comps: List,
    time: Time,
    compStrategy: CompStrategy,
    allowPushing: IDL.Variant({ Opening: IDL.Null, Closed: IDL.Null }),
  });
  const TokenSymbol = IDL.Text;
  const TokenStd = IDL.Variant({
    dft: IDL.Null,
    ext: IDL.Null,
    icp: IDL.Null,
    other: IDL.Text,
    cycles: IDL.Null,
    ledger: IDL.Null,
    icrc1: IDL.Null,
    dip20: IDL.Null,
    drc20: IDL.Null,
  });
  const TokenInfo = IDL.Tuple(IDL.Principal, TokenSymbol, TokenStd);
  const Toid__3 = IDL.Nat;
  const Timestamp__5 = IDL.Nat;
  const OrderFilled__1 = IDL.Record({
    time: Time,
    token0Value: BalanceChange,
    counterparty: Txid,
    token1Value: BalanceChange,
  });
  const OrderSide__2 = IDL.Variant({ Buy: IDL.Null, Sell: IDL.Null });
  const PriceResponse = IDL.Record({ quantity: IDL.Nat, price: IDL.Nat });
  const Liquidity = IDL.Record({
    vol: Vol__1,
    shares: Amount__1,
    shareWeighted: IDL.Record({
      updateTime: Timestamp__2,
      shareTimeWeighted: IDL.Nat,
    }),
    unitValue: IDL.Tuple(Amount__1, Amount__1),
    value0: Amount__1,
    value1: Amount__1,
    priceWeighted: PriceWeighted,
    swapCount: IDL.Nat64,
  });
  const Liquidity2 = IDL.Record({
    vol: Vol__1,
    shares: Amount__1,
    shareWeighted: IDL.Record({
      updateTime: Timestamp__2,
      shareTimeWeighted: IDL.Nat,
    }),
    unitValue: IDL.Tuple(Amount__1, Amount__1),
    orderCount: IDL.Nat64,
    token0: Amount__1,
    token1: Amount__1,
    priceWeighted: PriceWeighted,
    price: IDL.Nat,
    userCount: IDL.Nat64,
    unitSize: IDL.Nat,
  });
  const TrieList_4 = IDL.Record({
    total: IDL.Nat,
    data: IDL.Vec(
      IDL.Tuple(
        AccountId,
        IDL.Record({
          vol: Vol,
          orders: IDL.Nat,
          commission: Vol,
          filledCount: IDL.Nat,
        })
      )
    ),
    totalPage: IDL.Nat,
  });
  const TradingOrder__1 = IDL.Record({
    fee: IDL.Record({ fee0: IDL.Int, fee1: IDL.Int }),
    gas: IDL.Record({ gas0: IDL.Nat, gas1: IDL.Nat }),
    status: TradingStatus,
    toids: IDL.Vec(Toid),
    data: IDL.Opt(IDL.Vec(IDL.Nat8)),
    time: Time,
    txid: Txid,
    icrc1Account: IDL.Opt(
      IDL.Record({
        owner: IDL.Principal,
        subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
      })
    ),
    orderType: OrderType__1,
    filled: IDL.Vec(OrderFilled),
    expiration: Time,
    nonce: IDL.Nat,
    account: AccountId__2,
    remaining: OrderPrice__1,
    index: IDL.Nat,
    orderPrice: OrderPrice__1,
    refund: IDL.Tuple(IDL.Nat, IDL.Nat, IDL.Nat),
  });
  const TrieList_3 = IDL.Record({
    total: IDL.Nat,
    data: IDL.Vec(IDL.Tuple(Txid__1, TradingOrder__1)),
    totalPage: IDL.Nat,
  });
  const Price__1 = IDL.Nat;
  const TxnRecord = IDL.Record({
    fee: IDL.Record({ token0Fee: IDL.Int, token1Fee: IDL.Int }),
    status: Status,
    shares: ShareChange,
    msgCaller: IDL.Opt(IDL.Principal),
    order: IDL.Record({
      token0Value: IDL.Opt(BalanceChange__1),
      token1Value: IDL.Opt(BalanceChange__1),
    }),
    data: IDL.Opt(Data__1),
    time: Time,
    txid: Txid__3,
    orderMode: IDL.Variant({ AMM: IDL.Null, OrderBook: IDL.Null }),
    orderType: IDL.Opt(
      IDL.Variant({
        FAK: IDL.Null,
        FOK: IDL.Null,
        LMT: IDL.Null,
        MKT: IDL.Null,
      })
    ),
    filled: IDL.Record({
      token0Value: BalanceChange__1,
      token1Value: BalanceChange__1,
    }),
    token0: TokenType__1,
    token1: TokenType__1,
    nonce: Nonce__1,
    operation: OperationType,
    account: AccountId__1,
    details: IDL.Vec(
      IDL.Record({
        time: Time,
        token0Value: BalanceChange__1,
        counterparty: Txid__3,
        token1Value: BalanceChange__1,
      })
    ),
    caller: AccountId__1,
    index: IDL.Nat,
    cyclesWallet: IDL.Opt(CyclesWallet),
  });
  const OrderStatusResponse = IDL.Variant({
    Failed: TradingOrder,
    None: IDL.Null,
    Completed: TxnRecord,
    Pending: TradingOrder,
  });
  const TWAPSetting = IDL.Record({
    endTime: Timestamp,
    startingTime: Timestamp,
    order: IDL.Record({
      priceLimit: Price__2,
      priceSpread: Price__2,
      side: OrderSide,
    }),
    totalLimit: IDL.Variant({ Token0: IDL.Nat, Token1: IDL.Nat }),
    amountPerTrigger: IDL.Variant({
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
    triggerInterval: IDL.Nat,
  });
  const VWAPSetting = IDL.Record({
    endTime: Timestamp,
    startingTime: Timestamp,
    order: IDL.Record({
      priceLimit: Price__2,
      priceSpread: Price__2,
      side: OrderSide,
    }),
    totalLimit: IDL.Variant({ Token0: IDL.Nat, Token1: IDL.Nat }),
    amountPerTrigger: IDL.Variant({
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
    triggerVol: IDL.Variant({ Geom: Ppm__1, Arith: IDL.Nat }),
  });
  const IcebergOrderSetting = IDL.Record({
    endTime: Timestamp,
    startingTime: Timestamp,
    order: IDL.Record({ side: OrderSide, price: Price__2 }),
    totalLimit: IDL.Variant({ Token0: IDL.Nat, Token1: IDL.Nat }),
    amountPerTrigger: IDL.Variant({
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
  });
  const GridOrderSetting = IDL.Record({
    lowerLimit: Price__2,
    upperLimit: Price__2,
    spread: IDL.Variant({ Geom: Ppm__1, Arith: Price__2 }),
    amount: IDL.Variant({
      Percent: IDL.Opt(Ppm__1),
      Token0: IDL.Nat,
      Token1: IDL.Nat,
    }),
  });
  const STOrder = IDL.Record({
    status: STStatus__1,
    pendingOrders: IDL.Record({
      buy: IDL.Vec(IDL.Tuple(IDL.Opt(Txid__2), Price__2, IDL.Nat)),
      sell: IDL.Vec(IDL.Tuple(IDL.Opt(Txid__2), Price__2, IDL.Nat)),
    }),
    strategy: STStrategy,
    soid: Soid__1,
    initTime: Timestamp,
    icrc1Account: ICRC1Account,
    stats: STStats,
    triggerTime: Timestamp,
    stType: STType,
  });
  const TrieList_2 = IDL.Record({
    total: IDL.Nat,
    data: IDL.Vec(IDL.Tuple(Soid, STOrder)),
    totalPage: IDL.Nat,
  });
  const Setting = IDL.Record({
    gridMaxPerSide: IDL.Nat,
    stopLossCountMax: IDL.Nat,
    poFee1: IDL.Nat,
    poFee2: IDL.Float64,
    proCountMax: IDL.Nat,
    sloFee1: IDL.Nat,
    sloFee2: IDL.Float64,
  });
  const TrieList_1 = IDL.Record({
    total: IDL.Nat,
    data: IDL.Vec(IDL.Tuple(Txid__1, Soid)),
    totalPage: IDL.Nat,
  });
  const STStatus = IDL.Variant({
    Stopped: IDL.Null,
    Running: IDL.Null,
    Deleted: IDL.Null,
  });
  const Ppm = IDL.Nat;
  const SysMode = IDL.Variant({
    DisabledTrading: IDL.Null,
    ReadOnly: IDL.Null,
    ClosingOnly: IDL.Null,
    GeneralTrading: IDL.Null,
  });
  const OrderType = IDL.Variant({
    FAK: IDL.Null,
    FOK: IDL.Null,
    LMT: IDL.Null,
    MKT: IDL.Null,
  });
  const PeriodNs = IDL.Int;
  const Data = IDL.Vec(IDL.Nat8);
  const TradingResult = IDL.Variant({
    ok: IDL.Record({
      status: TradingStatus,
      txid: Txid,
      filled: IDL.Vec(OrderFilled),
    }),
    err: IDL.Record({
      code: IDL.Variant({
        NonceError: IDL.Null,
        InvalidAmount: IDL.Null,
        UndefinedError: IDL.Null,
        UnacceptableVolatility: IDL.Null,
        TransactionBlocking: IDL.Null,
        InsufficientBalance: IDL.Null,
        TransferException: IDL.Null,
      }),
      message: IDL.Text,
    }),
  });
  const DebitToken = IDL.Principal;
  const TrieList = IDL.Record({
    total: IDL.Nat,
    data: IDL.Vec(IDL.Tuple(AccountId, Vol)),
    totalPage: IDL.Nat,
  });
  return IDL.Service({
    IDO_config: IDL.Func([IDOSetting], [], []),
    IDO_getConfig: IDL.Func(
      [],
      [IDL.Opt(IDL.Principal), IDOSetting, IDL.Opt(IDORequirement)],
      ["query"]
    ),
    IDO_qualification: IDL.Func(
      [IDL.Opt(Address)],
      [IDL.Vec(IDL.Tuple(Address, Participant))],
      ["query"]
    ),
    IDO_removeWhitelist: IDL.Func([IDL.Vec(Address)], [], []),
    IDO_setFunder: IDL.Func([IDL.Opt(IDL.Principal), IDL.Opt(IDORequirement)], [], []),
    IDO_setWhitelist: IDL.Func([IDL.Vec(IDL.Tuple(Address, Amount))], [], []),
    IDO_updateQualification: IDL.Func([IDL.Opt(Sa)], [IDL.Opt(Participant)], []),
    accountBalance: IDL.Func([Address], [KeepingBalance], ["query"]),
    accountConfig: IDL.Func(
      [IDL.Variant({ PoolMode: IDL.Null, TunnelMode: IDL.Null }), IDL.Bool, IDL.Opt(Sa)],
      [],
      []
    ),
    accountSetting: IDL.Func([Address], [AccountSetting__1], ["query"]),
    backup: IDL.Func([BackupRequest], [BackupResponse], []),
    brokerList: IDL.Func([IDL.Opt(ListPage), IDL.Opt(ListSize)], [TrieList_5], ["query"]),
    cancel: IDL.Func([Nonce, IDL.Opt(Sa)], [], []),
    cancelAll: IDL.Func(
      [
        IDL.Variant({
          self_sa: IDL.Opt(Sa),
          management: IDL.Opt(AccountId),
        }),
        IDL.Opt(OrderSide),
      ],
      [],
      []
    ),
    cancelByTxid: IDL.Func([Txid__1, IDL.Opt(Sa)], [], []),
    checkPoolBalance: IDL.Func(
      [],
      [
        IDL.Opt(IDL.Bool),
        IDL.Record({
          result: IDL.Bool,
          total: IDL.Record({ token0: Amount, token1: Amount }),
          pool: IDL.Record({ token0: Amount, token1: Amount }),
        }),
      ],
      []
    ),
    clearAccountSetting: IDL.Func([], [], []),
    clearNonCoreData: IDL.Func([], [], []),
    config: IDL.Func([DexConfig], [IDL.Bool], []),
    count: IDL.Func([IDL.Opt(Address)], [IDL.Nat], ["query"]),
    debug_gridOrders: IDL.Func([], [IDL.Vec(IDL.Tuple(Soid, ICRC1Account__1, OrderPrice))], []),
    deposit: IDL.Func(
      [IDL.Variant({ token0: IDL.Null, token1: IDL.Null }), Amount, IDL.Opt(Sa)],
      [],
      []
    ),
    depositFallback: IDL.Func([IDL.Opt(Sa)], [Amount, Amount], []),
    drc205_canisterId: IDL.Func([], [IDL.Principal], ["query"]),
    drc205_config: IDL.Func([Config], [IDL.Bool], []),
    drc205_dexInfo: IDL.Func([], [DexInfo], ["query"]),
    drc205_events: IDL.Func([IDL.Opt(Address__2)], [IDL.Vec(TxnRecord__3)], ["query"]),
    drc205_events_filter: IDL.Func(
      [IDL.Opt(Address__2), IDL.Opt(Time), IDL.Opt(Time)],
      [IDL.Vec(TxnRecord__3), IDL.Bool],
      ["query"]
    ),
    drc205_getConfig: IDL.Func([], [Setting__1], ["query"]),
    drc205_pool: IDL.Func([], [IDL.Vec(IDL.Tuple(Txid__1, TxnRecord__3, IDL.Nat))], ["query"]),
    drc205_txn: IDL.Func([Txid__6], [IDL.Opt(TxnRecord__3)], ["query"]),
    drc205_txn2: IDL.Func([Txid__6], [IDL.Opt(TxnRecord__3)], []),
    drc207: IDL.Func([], [DRC207Support], ["query"]),
    fallback: IDL.Func([Nonce, IDL.Opt(Sa)], [IDL.Bool], []),
    fallbackByTxid: IDL.Func([Txid__1, IDL.Opt(Sa)], [IDL.Bool], []),
    fee: IDL.Func(
      [],
      [
        IDL.Record({
          maker: IDL.Record({ buy: IDL.Float64, sell: IDL.Float64 }),
          taker: IDL.Record({ buy: IDL.Float64, sell: IDL.Float64 }),
        }),
      ],
      ["query"]
    ),
    feeStatus: IDL.Func([], [FeeStatus], ["query"]),
    getAuctionMode: IDL.Func([], [IDL.Bool, AccountId], ["query"]),
    getConfig: IDL.Func([], [DexSetting], ["query"]),
    getDepositAccount: IDL.Func([Address], [Account__1, Address], ["query"]),
    getPairAddress: IDL.Func(
      [],
      [
        IDL.Record({
          fees: IDL.Tuple(Account__1, Address),
          pool: IDL.Tuple(Account__1, Address),
        }),
      ],
      ["query"]
    ),
    getQuotes: IDL.Func([KInterval], [IDL.Vec(KBar)], ["query"]),
    getRole: IDL.Func(
      [Address],
      [
        IDL.Record({
          broker: IDL.Bool,
          proTrader: IDL.Bool,
          vipMaker: IDL.Bool,
        }),
      ],
      ["query"]
    ),
    getTxAccount: IDL.Func([Address], [Account__1, TxAccount, Nonce, Txid__1], ["query"]),
    ictc_TM: IDL.Func([], [IDL.Text], ["query"]),
    ictc_addAdmin: IDL.Func([IDL.Principal], [], []),
    ictc_appendTT: IDL.Func(
      [
        IDL.Opt(IDL.Vec(IDL.Nat8)),
        Toid__4,
        IDL.Opt(Ttid__2),
        IDL.Principal,
        CallType__1,
        IDL.Vec(Ttid__2),
      ],
      [Ttid__2],
      []
    ),
    ictc_blockTO: IDL.Func([Toid__4], [IDL.Opt(Toid__4)], []),
    ictc_clearLog: IDL.Func([IDL.Opt(IDL.Int), IDL.Bool], [], []),
    ictc_clearTTPool: IDL.Func([], [], []),
    ictc_completeTO: IDL.Func([Toid__4, OrderStatus], [IDL.Bool], []),
    ictc_doneTO: IDL.Func([Toid__4, OrderStatus, IDL.Bool], [IDL.Bool], []),
    ictc_doneTT: IDL.Func([Toid__4, Ttid__2, IDL.Bool], [IDL.Opt(Ttid__2)], []),
    ictc_getAdmins: IDL.Func([], [IDL.Vec(IDL.Principal)], ["query"]),
    ictc_getCalleeStatus: IDL.Func([IDL.Principal], [IDL.Opt(CalleeStatus)], ["query"]),
    ictc_getPool: IDL.Func(
      [],
      [
        IDL.Record({
          ttPool: IDL.Record({
            total: IDL.Nat,
            items: IDL.Vec(IDL.Tuple(Ttid__2, Task__2)),
          }),
          toPool: IDL.Record({
            total: IDL.Nat,
            items: IDL.Vec(IDL.Tuple(Toid__4, IDL.Opt(Order__1))),
          }),
        }),
      ],
      ["query"]
    ),
    ictc_getTO: IDL.Func([Toid__4], [IDL.Opt(Order__1)], ["query"]),
    ictc_getTOCount: IDL.Func([], [IDL.Nat], ["query"]),
    ictc_getTOPool: IDL.Func([], [IDL.Vec(IDL.Tuple(Toid__4, IDL.Opt(Order__1)))], ["query"]),
    ictc_getTOs: IDL.Func(
      [IDL.Nat, IDL.Nat],
      [
        IDL.Record({
          total: IDL.Nat,
          data: IDL.Vec(IDL.Tuple(Toid__4, Order__1)),
          totalPage: IDL.Nat,
        }),
      ],
      ["query"]
    ),
    ictc_getTT: IDL.Func([Ttid__2], [IDL.Opt(TaskEvent)], ["query"]),
    ictc_getTTByTO: IDL.Func([Toid__4], [IDL.Vec(TaskEvent)], ["query"]),
    ictc_getTTErrors: IDL.Func(
      [IDL.Nat, IDL.Nat],
      [
        IDL.Record({
          total: IDL.Nat,
          data: IDL.Vec(IDL.Tuple(IDL.Nat, ErrorLog)),
          totalPage: IDL.Nat,
        }),
      ],
      ["query"]
    ),
    ictc_getTTPool: IDL.Func([], [IDL.Vec(IDL.Tuple(Ttid__2, Task__2))], ["query"]),
    ictc_getTTs: IDL.Func(
      [IDL.Nat, IDL.Nat],
      [
        IDL.Record({
          total: IDL.Nat,
          data: IDL.Vec(IDL.Tuple(Ttid__2, TaskEvent)),
          totalPage: IDL.Nat,
        }),
      ],
      ["query"]
    ),
    ictc_redoTT: IDL.Func([Toid__4, Ttid__2], [IDL.Opt(Ttid__2)], []),
    ictc_removeAdmin: IDL.Func([IDL.Principal], [], []),
    ictc_runTO: IDL.Func([Toid__4], [IDL.Opt(OrderStatus)], []),
    ictc_runTT: IDL.Func([], [IDL.Bool], []),
    info: IDL.Func(
      [],
      [
        IDL.Record({
          decimals: IDL.Nat8,
          setting: DexSetting,
          owner: IDL.Principal,
          name: IDL.Text,
          version: IDL.Text,
          token0: TokenInfo,
          token1: TokenInfo,
          paused: IDL.Bool,
        }),
      ],
      ["query"]
    ),
    init: IDL.Func([], [], []),
    isAccountIctcDone: IDL.Func([AccountId], [IDL.Bool, IDL.Vec(Toid__3)], ["query"]),
    latestFilled: IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(Timestamp__5, Txid__1, OrderFilled__1, OrderSide__2))],
      ["query"]
    ),
    level10: IDL.Func(
      [],
      [
        IDL.Nat,
        IDL.Record({
          ask: IDL.Vec(PriceResponse),
          bid: IDL.Vec(PriceResponse),
        }),
      ],
      ["query"]
    ),
    level100: IDL.Func(
      [],
      [
        IDL.Nat,
        IDL.Record({
          ask: IDL.Vec(PriceResponse),
          bid: IDL.Vec(PriceResponse),
        }),
      ],
      ["query"]
    ),
    liquidity: IDL.Func([IDL.Opt(Address)], [Liquidity], ["query"]),
    liquidity2: IDL.Func([IDL.Opt(Address)], [Liquidity2], ["query"]),
    makerList: IDL.Func([IDL.Opt(ListPage), IDL.Opt(ListSize)], [TrieList_4], ["query"]),
    makerRebate: IDL.Func([Address], [IDL.Float64, IDL.Float64], ["query"]),
    mergePair: IDL.Func([IDL.Principal], [IDL.Bool], []),
    name: IDL.Func([], [IDL.Text], ["query"]),
    orderExpirationDuration: IDL.Func([], [IDL.Int], ["query"]),
    pending: IDL.Func(
      [IDL.Opt(Address), IDL.Opt(ListPage), IDL.Opt(ListSize)],
      [TrieList_3],
      ["query"]
    ),
    pendingAll: IDL.Func([IDL.Opt(ListPage), IDL.Opt(ListSize)], [TrieList_3], ["query"]),
    pendingCount: IDL.Func([], [IDL.Nat], ["query"]),
    poolBalance: IDL.Func([], [IDL.Record({ token0: Amount, token1: Amount })], ["query"]),
    prepare: IDL.Func([Address], [TxAccount, Nonce], ["query"]),
    recovery: IDL.Func([BackupResponse], [IDL.Bool], []),
    removeVipMaker: IDL.Func([Address], [], []),
    safeAccountBalance: IDL.Func(
      [Address],
      [
        IDL.Record({
          balance: KeepingBalance,
          pendingOrders: IDL.Tuple(Amount, Amount),
          price: Price__1,
          unitSize: IDL.Nat,
        }),
      ],
      ["query"]
    ),
    setAuctionMode: IDL.Func([IDL.Bool, IDL.Opt(AccountId)], [IDL.Bool, AccountId], []),
    setOrderFail: IDL.Func([IDL.Text, Amount, Amount], [IDL.Bool], []),
    setPause: IDL.Func([IDL.Bool, IDL.Opt(Time)], [IDL.Bool], []),
    setUpgradeMode: IDL.Func([IDL.Variant({ All: IDL.Null, Base: IDL.Null })], [], []),
    setVipMaker: IDL.Func([Address, IDL.Nat], [], []),
    stats: IDL.Func(
      [],
      [
        IDL.Record({
          change24h: IDL.Float64,
          vol24h: Vol,
          totalVol: Vol,
          price: IDL.Float64,
        }),
      ],
      ["query"]
    ),
    status: IDL.Func([Address, Nonce], [OrderStatusResponse], ["query"]),
    statusByTxid: IDL.Func([Txid__1], [OrderStatusResponse], ["query"]),
    sto_cancelPendingOrders: IDL.Func([Soid, IDL.Opt(Sa)], [], []),
    sto_clearTxidLog: IDL.Func([], [], []),
    sto_config: IDL.Func(
      [
        IDL.Record({
          gridMaxPerSide: IDL.Opt(IDL.Nat),
          stopLossCountMax: IDL.Opt(IDL.Nat),
          poFee1: IDL.Opt(IDL.Nat),
          poFee2: IDL.Opt(IDL.Float64),
          proCountMax: IDL.Opt(IDL.Nat),
          sloFee1: IDL.Opt(IDL.Nat),
          sloFee2: IDL.Opt(IDL.Float64),
        }),
      ],
      [],
      []
    ),
    sto_createProOrder: IDL.Func(
      [
        IDL.Variant({
          TWAP: TWAPSetting,
          VWAP: VWAPSetting,
          IcebergOrder: IcebergOrderSetting,
          GridOrder: GridOrderSetting,
        }),
        IDL.Opt(Sa),
      ],
      [Soid],
      []
    ),
    sto_createStopLossOrder: IDL.Func(
      [
        IDL.Record({
          order: IDL.Record({
            side: OrderSide,
            quantity: IDL.Nat,
            price: Price__1,
          }),
          triggerPrice: Price__1,
        }),
        IDL.Opt(Sa),
      ],
      [Soid],
      []
    ),
    sto_enableStratOrder: IDL.Func([IDL.Variant({ Enable: IDL.Null, Disable: IDL.Null })], [], []),
    sto_getAccountProOrders: IDL.Func([Address], [IDL.Vec(STOrder)], ["query"]),
    sto_getAccountStopLossOrders: IDL.Func([Address], [IDL.Vec(STOrder)], ["query"]),
    sto_getActiveProOrders: IDL.Func([IDL.Opt(ListPage), IDL.Opt(ListSize)], [TrieList_2], []),
    sto_getActiveStopLossOrders: IDL.Func(
      [IDL.Variant({ Buy: IDL.Null, Sell: IDL.Null }), IDL.Opt(ListPage), IDL.Opt(ListSize)],
      [TrieList_2],
      []
    ),
    sto_getConfig: IDL.Func([], [Setting], ["query"]),
    sto_getStratOrder: IDL.Func([Soid], [IDL.Opt(STOrder)], ["query"]),
    sto_getStratOrderByTxid: IDL.Func([Txid__1], [IDL.Opt(STOrder)], ["query"]),
    sto_getStratTxids: IDL.Func([IDL.Opt(ListPage), IDL.Opt(ListSize)], [TrieList_1], []),
    sto_updateProOrder: IDL.Func(
      [
        Soid,
        IDL.Variant({
          TWAP: IDL.Record({
            status: IDL.Opt(STStatus),
            setting: IDL.Opt(TWAPSetting),
          }),
          VWAP: IDL.Record({
            status: IDL.Opt(STStatus),
            setting: IDL.Opt(VWAPSetting),
          }),
          IcebergOrder: IDL.Record({
            status: IDL.Opt(STStatus),
            setting: IDL.Opt(IcebergOrderSetting),
          }),
          GridOrder: IDL.Record({
            status: IDL.Opt(STStatus),
            lowerLimit: IDL.Opt(Price__1),
            upperLimit: IDL.Opt(Price__1),
            spread: IDL.Opt(IDL.Variant({ Geom: Ppm, Arith: Price__1 })),
            amount: IDL.Opt(
              IDL.Variant({
                Percent: IDL.Opt(Ppm),
                Token0: IDL.Nat,
                Token1: IDL.Nat,
              })
            ),
          }),
        }),
        IDL.Opt(Sa),
      ],
      [Soid],
      []
    ),
    sto_updateStopLossOrder: IDL.Func(
      [
        Soid,
        IDL.Record({
          status: IDL.Opt(STStatus),
          order: IDL.Opt(
            IDL.Record({
              side: OrderSide,
              quantity: IDL.Nat,
              price: Price__1,
            })
          ),
          triggerPrice: IDL.Opt(Price__1),
        }),
        IDL.Opt(Sa),
      ],
      [Soid],
      []
    ),
    sync: IDL.Func([], [], []),
    sysMode: IDL.Func([], [IDL.Record({ mode: SysMode, openingTime: Time })], ["query"]),
    ta_ambassador: IDL.Func([Address], [IDL.Bool, IDL.Text, IDL.Nat, Vol], ["query"]),
    ta_description: IDL.Func([], [IDL.Text], ["query"]),
    ta_getReferrer: IDL.Func([Address], [IDL.Opt(IDL.Tuple(Address, IDL.Bool))], ["query"]),
    ta_setDescription: IDL.Func([IDL.Text], [], []),
    ta_setReferrer: IDL.Func([Address, IDL.Opt(IDL.Text), IDL.Opt(Sa)], [IDL.Bool], []),
    ta_stats: IDL.Func([IDL.Opt(IDL.Text)], [IDL.Nat, IDL.Nat, Vol], ["query"]),
    timerStart: IDL.Func([IDL.Nat], [], []),
    timerStop: IDL.Func([], [], []),
    token0: IDL.Func([], [TokenType, IDL.Opt(TokenStd)], ["query"]),
    token1: IDL.Func([], [TokenType, IDL.Opt(TokenStd)], ["query"]),
    tpsStats: IDL.Func([], [IDL.Nat, IDL.Float64, IDL.Nat, IDL.Nat, IDL.Int, IDL.Int], ["query"]),
    trade: IDL.Func(
      [OrderPrice, OrderType, IDL.Opt(PeriodNs), IDL.Opt(Nonce), IDL.Opt(Sa), IDL.Opt(Data)],
      [TradingResult],
      []
    ),
    tradeCore: IDL.Func(
      [
        OrderPrice,
        OrderType,
        IDL.Opt(PeriodNs),
        IDL.Opt(Nonce),
        IDL.Opt(Sa),
        IDL.Opt(Data),
        IDL.Opt(IDL.Record({ broker: IDL.Principal, rate: IDL.Float64 })),
        IDL.Opt(IDL.Bool),
      ],
      [TradingResult],
      []
    ),
    tradeMKT: IDL.Func(
      [DebitToken, Amount, IDL.Opt(Nonce), IDL.Opt(Sa), IDL.Opt(Data)],
      [TradingResult],
      []
    ),
    tradeMKT_b: IDL.Func(
      [
        DebitToken,
        Amount,
        IDL.Opt(IDL.Nat),
        IDL.Opt(Nonce),
        IDL.Opt(Sa),
        IDL.Opt(Data),
        IDL.Opt(IDL.Record({ broker: IDL.Principal, rate: IDL.Float64 })),
      ],
      [TradingResult],
      []
    ),
    trade_b: IDL.Func(
      [
        OrderPrice,
        OrderType,
        IDL.Opt(PeriodNs),
        IDL.Opt(Nonce),
        IDL.Opt(Sa),
        IDL.Opt(Data),
        IDL.Opt(IDL.Record({ broker: IDL.Principal, rate: IDL.Float64 })),
      ],
      [TradingResult],
      []
    ),
    userCount: IDL.Func([], [IDL.Nat], ["query"]),
    version: IDL.Func([], [IDL.Text], ["query"]),
    volsAll: IDL.Func([IDL.Opt(ListPage), IDL.Opt(ListSize)], [TrieList], ["query"]),
    wallet_receive: IDL.Func([], [], []),
    withdraw: IDL.Func([IDL.Opt(Amount), IDL.Opt(Amount), IDL.Opt(Sa)], [Amount, Amount], []),
    withdraw2: IDL.Func(
      [IDL.Opt(Amount), IDL.Opt(Amount), IDL.Opt(Sa)],
      [Amount, Amount, IDL.Variant({ Completed: IDL.Null, Pending: IDL.Null })],
      []
    ),
    withdraw_cycles: IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => {
  return [];
};
