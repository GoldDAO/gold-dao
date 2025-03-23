export const idlFactory = ({ IDL }) => {
  const TransactionType = IDL.Variant({
    decreaseLiquidity: IDL.Null,
    claim: IDL.Null,
    swap: IDL.Null,
    addLiquidity: IDL.Null,
    increaseLiquidity: IDL.Null,
  });
  const Transaction = IDL.Record({
    to: IDL.Text,
    action: TransactionType,
    token0Id: IDL.Text,
    token1Id: IDL.Text,
    liquidityTotal: IDL.Nat,
    from: IDL.Text,
    hash: IDL.Text,
    tick: IDL.Int,
    token1Price: IDL.Float64,
    recipient: IDL.Text,
    token0ChangeAmount: IDL.Float64,
    sender: IDL.Text,
    liquidityChange: IDL.Nat,
    token1Standard: IDL.Text,
    token0Fee: IDL.Float64,
    token1Fee: IDL.Float64,
    timestamp: IDL.Int,
    token1ChangeAmount: IDL.Float64,
    token1Decimals: IDL.Float64,
    token0Standard: IDL.Text,
    amountUSD: IDL.Float64,
    amountToken0: IDL.Float64,
    amountToken1: IDL.Float64,
    poolFee: IDL.Nat,
    token0Symbol: IDL.Text,
    token0Decimals: IDL.Float64,
    token0Price: IDL.Float64,
    token1Symbol: IDL.Text,
    poolId: IDL.Text,
  });
  const PoolTvlData = IDL.Record({
    token0Id: IDL.Text,
    token1Id: IDL.Text,
    pool: IDL.Text,
    tvlUSD: IDL.Float64,
    token0Symbol: IDL.Text,
    token1Symbol: IDL.Text,
  });
  const NatResult = IDL.Variant({ ok: IDL.Nat, err: IDL.Text });
  const PublicTokenOverview = IDL.Record({
    id: IDL.Nat,
    volumeUSD1d: IDL.Float64,
    volumeUSD7d: IDL.Float64,
    totalVolumeUSD: IDL.Float64,
    name: IDL.Text,
    volumeUSD: IDL.Float64,
    feesUSD: IDL.Float64,
    priceUSDChange: IDL.Float64,
    address: IDL.Text,
    txCount: IDL.Int,
    priceUSD: IDL.Float64,
    standard: IDL.Text,
    symbol: IDL.Text,
  });
  const PoolInfo = IDL.Record({
    fee: IDL.Int,
    token0Id: IDL.Text,
    token1Id: IDL.Text,
    pool: IDL.Text,
    token1Price: IDL.Float64,
    token1Standard: IDL.Text,
    token1Decimals: IDL.Float64,
    token0Standard: IDL.Text,
    token0Symbol: IDL.Text,
    token0Decimals: IDL.Float64,
    token0Price: IDL.Float64,
    token1Symbol: IDL.Text,
  });
  const PublicTokenChartDayData = IDL.Record({
    id: IDL.Int,
    volumeUSD: IDL.Float64,
    timestamp: IDL.Int,
    txCount: IDL.Int,
  });
  const PublicTokenPricesData = IDL.Record({
    id: IDL.Int,
    low: IDL.Float64,
    high: IDL.Float64,
    close: IDL.Float64,
    open: IDL.Float64,
    timestamp: IDL.Int,
  });
  const OldPublicTokenOverview = IDL.Record({
    id: IDL.Nat,
    totalVolumeUSD: IDL.Float64,
    name: IDL.Text,
    priceUSDChangeWeek: IDL.Float64,
    volumeUSD: IDL.Float64,
    feesUSD: IDL.Float64,
    priceUSDChange: IDL.Float64,
    tvlUSD: IDL.Float64,
    address: IDL.Text,
    volumeUSDWeek: IDL.Float64,
    txCount: IDL.Int,
    priceUSD: IDL.Float64,
    volumeUSDChange: IDL.Float64,
    tvlUSDChange: IDL.Float64,
    standard: IDL.Text,
    tvlToken: IDL.Float64,
    symbol: IDL.Text,
  });
  return IDL.Service({
    addOwners: IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    batchInsert: IDL.Func([IDL.Text, IDL.Vec(Transaction)], [], []),
    batchUpdatePoolTvl: IDL.Func([IDL.Vec(PoolTvlData)], [], []),
    clean: IDL.Func([], [], []),
    cycleAvailable: IDL.Func([], [NatResult], []),
    cycleBalance: IDL.Func([], [NatResult], ["query"]),
    getAllTokens: IDL.Func([], [IDL.Vec(PublicTokenOverview)], ["query"]),
    getOwners: IDL.Func([], [IDL.Vec(IDL.Principal)], []),
    getPoolTvl: IDL.Func([], [IDL.Vec(PoolTvlData)], ["query"]),
    getPoolsForToken: IDL.Func([IDL.Text], [IDL.Vec(PoolInfo)], ["query"]),
    getToken: IDL.Func([IDL.Text], [PublicTokenOverview], ["query"]),
    getTokenChartData: IDL.Func(
      [IDL.Text, IDL.Nat, IDL.Nat],
      [IDL.Vec(PublicTokenChartDayData)],
      ["query"]
    ),
    getTokenPricesData: IDL.Func(
      [IDL.Text, IDL.Int, IDL.Int, IDL.Nat],
      [IDL.Vec(PublicTokenPricesData)],
      ["query"]
    ),
    getTokenTransactions: IDL.Func(
      [IDL.Text, IDL.Nat, IDL.Nat],
      [IDL.Vec(Transaction)],
      ["query"]
    ),
    insert: IDL.Func([IDL.Text, Transaction], [], []),
    updateDayData: IDL.Func(
      [IDL.Text, IDL.Nat, IDL.Float64, IDL.Float64, IDL.Float64, IDL.Float64],
      [],
      []
    ),
    updateOverview: IDL.Func([IDL.Vec(OldPublicTokenOverview)], [], []),
  });
};
export const init = ({ IDL }) => [];
