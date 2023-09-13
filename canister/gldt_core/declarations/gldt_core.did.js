export const idlFactory = ({ IDL }) => {
  const Box = IDL.Rec();
  const GldNft = IDL.Rec();
  const NftCanisterConf = IDL.Record({ 'grams' : IDL.Nat16 });
  const Conf = IDL.Record({
    'gld_nft_canister_ids' : IDL.Vec(IDL.Tuple(IDL.Principal, NftCanisterConf)),
    'gldt_ledger_canister_id' : IDL.Principal,
  });
  const StatusRequest = IDL.Record({
    'memory_size' : IDL.Bool,
    'cycles' : IDL.Bool,
    'heap_memory_size' : IDL.Bool,
  });
  const MetricsGranularity = IDL.Variant({
    'hourly' : IDL.Null,
    'daily' : IDL.Null,
  });
  const GetMetricsParameters = IDL.Record({
    'dateToMillis' : IDL.Nat,
    'granularity' : MetricsGranularity,
    'dateFromMillis' : IDL.Nat,
  });
  const MetricsRequest = IDL.Record({ 'parameters' : GetMetricsParameters });
  const GetLogMessagesFilter = IDL.Record({
    'analyzeCount' : IDL.Nat32,
    'messageRegex' : IDL.Opt(IDL.Text),
    'messageContains' : IDL.Opt(IDL.Text),
  });
  const GetLogMessagesParameters = IDL.Record({
    'count' : IDL.Nat32,
    'filter' : IDL.Opt(GetLogMessagesFilter),
    'fromTimeNanos' : IDL.Opt(IDL.Nat64),
  });
  const GetLatestLogMessagesParameters = IDL.Record({
    'upToTimeNanos' : IDL.Opt(IDL.Nat64),
    'count' : IDL.Nat32,
    'filter' : IDL.Opt(GetLogMessagesFilter),
  });
  const CanisterLogRequest = IDL.Variant({
    'getMessagesInfo' : IDL.Null,
    'getMessages' : GetLogMessagesParameters,
    'getLatestMessages' : GetLatestLogMessagesParameters,
  });
  const GetInformationRequest = IDL.Record({
    'status' : IDL.Opt(StatusRequest),
    'metrics' : IDL.Opt(MetricsRequest),
    'logs' : IDL.Opt(CanisterLogRequest),
    'version' : IDL.Bool,
  });
  const StatusResponse = IDL.Record({
    'memory_size' : IDL.Opt(IDL.Nat64),
    'cycles' : IDL.Opt(IDL.Nat64),
    'heap_memory_size' : IDL.Opt(IDL.Nat64),
  });
  const HourlyMetricsData = IDL.Record({
    'updateCalls' : IDL.Vec(IDL.Nat64),
    'canisterHeapMemorySize' : IDL.Vec(IDL.Nat64),
    'canisterCycles' : IDL.Vec(IDL.Nat64),
    'canisterMemorySize' : IDL.Vec(IDL.Nat64),
    'timeMillis' : IDL.Int,
  });
  const NumericEntity = IDL.Record({
    'avg' : IDL.Nat64,
    'max' : IDL.Nat64,
    'min' : IDL.Nat64,
    'first' : IDL.Nat64,
    'last' : IDL.Nat64,
  });
  const DailyMetricsData = IDL.Record({
    'updateCalls' : IDL.Nat64,
    'canisterHeapMemorySize' : NumericEntity,
    'canisterCycles' : NumericEntity,
    'canisterMemorySize' : NumericEntity,
    'timeMillis' : IDL.Int,
  });
  const CanisterMetricsData = IDL.Variant({
    'hourly' : IDL.Vec(HourlyMetricsData),
    'daily' : IDL.Vec(DailyMetricsData),
  });
  const CanisterMetrics = IDL.Record({ 'data' : CanisterMetricsData });
  const MetricsResponse = IDL.Record({ 'metrics' : IDL.Opt(CanisterMetrics) });
  const CanisterLogFeature = IDL.Variant({
    'filterMessageByContains' : IDL.Null,
    'filterMessageByRegex' : IDL.Null,
  });
  const CanisterLogMessagesInfo = IDL.Record({
    'features' : IDL.Vec(IDL.Opt(CanisterLogFeature)),
    'lastTimeNanos' : IDL.Opt(IDL.Nat64),
    'count' : IDL.Nat32,
    'firstTimeNanos' : IDL.Opt(IDL.Nat64),
  });
  const LogMessageData = IDL.Record({
    'timeNanos' : IDL.Nat64,
    'message' : IDL.Text,
  });
  const CanisterLogMessages = IDL.Record({
    'data' : IDL.Vec(LogMessageData),
    'lastAnalyzedMessageTimeNanos' : IDL.Opt(IDL.Nat64),
  });
  const CanisterLogResponse = IDL.Variant({
    'messagesInfo' : CanisterLogMessagesInfo,
    'messages' : CanisterLogMessages,
  });
  const GetInformationResponse = IDL.Record({
    'status' : IDL.Opt(StatusResponse),
    'metrics' : IDL.Opt(MetricsResponse),
    'logs' : IDL.Opt(CanisterLogResponse),
    'version' : IDL.Opt(IDL.Nat),
  });
  const GetRecordsRequest = IDL.Record({
    'page' : IDL.Opt(IDL.Nat32),
    'limit' : IDL.Opt(IDL.Nat32),
  });
  const RecordType = IDL.Variant({ 'Burn' : IDL.Null, 'Mint' : IDL.Null });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const GldtRecord = IDL.Record({
    'nft_id' : IDL.Text,
    'record_type' : RecordType,
    'memo' : IDL.Vec(IDL.Nat8),
    'num_tokens' : IDL.Nat,
    'escrow_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'counterparty' : Account,
    'grams' : IDL.Nat16,
    'timestamp' : IDL.Nat64,
    'nft_sale_id' : IDL.Text,
    'gld_nft_canister_id' : IDL.Principal,
    'block_height' : IDL.Nat,
  });
  const GetRecordsResponse = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Opt(IDL.Vec(GldtRecord)),
  });
  const GetStatusRequest = IDL.Record({
    'nft_id' : IDL.Text,
    'gld_nft_canister_id' : IDL.Principal,
    'sale_id' : IDL.Text,
  });
  const SwappingStates = IDL.Variant({
    'Burned' : IDL.Null,
    'Initialised' : IDL.Null,
    'Swapped' : IDL.Null,
    'Finalised' : IDL.Null,
    'Minted' : IDL.Null,
  });
  const GetStatusResponse = IDL.Record({ 'status' : IDL.Opt(SwappingStates) });
  const Result = IDL.Variant({ 'Ok' : GetStatusResponse, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : GetRecordsResponse, 'Err' : IDL.Text });
  const InfoRequest = IDL.Record({
    'nft_id' : IDL.Text,
    'source_canister' : IDL.Principal,
  });
  const GldtBurned = IDL.Record({ 'burn_block_height' : IDL.Nat64 });
  const GldtMinted = IDL.Record({
    'num_tokens' : IDL.Nat,
    'mint_block_height' : IDL.Nat,
    'last_audited_timestamp_seconds' : IDL.Nat64,
    'burned' : IDL.Opt(GldtBurned),
  });
  const GldtSwapped = IDL.Record({ 'index' : IDL.Nat, 'sale_id' : IDL.Text });
  GldNft.fill(
    IDL.Record({
      'requested_memo' : IDL.Vec(IDL.Nat8),
      'older_record' : IDL.Opt(GldNft),
      'to_subaccount' : IDL.Vec(IDL.Nat8),
      'minted' : IDL.Opt(GldtMinted),
      'swapped' : IDL.Opt(GldtSwapped),
      'receiving_account' : Account,
      'grams' : IDL.Nat16,
      'gldt_minting_timestamp_seconds' : IDL.Nat64,
      'nft_sale_id' : IDL.Text,
      'gld_nft_canister_id' : IDL.Principal,
    })
  );
  const NftInfo = IDL.Record({ 'info' : IDL.Opt(GldNft) });
  const AuctionStateShared_status = IDL.Variant({
    'closed' : IDL.Null,
    'open' : IDL.Null,
    'not_started' : IDL.Null,
  });
  Box.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(Box, Box)),
      'Nat' : IDL.Nat,
      'Set' : IDL.Vec(Box),
      'Nat16' : IDL.Nat16,
      'Nat32' : IDL.Nat32,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Bool' : IDL.Bool,
      'Int8' : IDL.Int8,
      'Ints' : IDL.Vec(IDL.Int),
      'Nat8' : IDL.Nat8,
      'Nats' : IDL.Vec(IDL.Nat),
      'Text' : IDL.Text,
      'Bytes' : IDL.Vec(IDL.Nat8),
      'Int16' : IDL.Int16,
      'Int32' : IDL.Int32,
      'Int64' : IDL.Int64,
      'Option' : IDL.Opt(Box),
      'Floats' : IDL.Vec(IDL.Float64),
      'Float' : IDL.Float64,
      'Principal' : IDL.Principal,
      'Array' : IDL.Vec(Box),
      'Class' : IDL.Vec(
        IDL.Record({ 'value' : Box, 'name' : IDL.Text, 'immutable' : IDL.Bool })
      ),
    })
  );
  const PropertyShared = IDL.Record({
    'value' : Box,
    'name' : IDL.Text,
    'immutable' : IDL.Bool,
  });
  const CandyShared = IDL.Variant({
    'Int' : IDL.Int,
    'Map' : IDL.Vec(IDL.Tuple(Box, Box)),
    'Nat' : IDL.Nat,
    'Set' : IDL.Vec(Box),
    'Nat16' : IDL.Nat16,
    'Nat32' : IDL.Nat32,
    'Nat64' : IDL.Nat64,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Bool' : IDL.Bool,
    'Int8' : IDL.Int8,
    'Ints' : IDL.Vec(IDL.Int),
    'Nat8' : IDL.Nat8,
    'Nats' : IDL.Vec(IDL.Nat),
    'Text' : IDL.Text,
    'Bytes' : IDL.Vec(IDL.Nat8),
    'Int16' : IDL.Int16,
    'Int32' : IDL.Int32,
    'Int64' : IDL.Int64,
    'Option' : IDL.Opt(Box),
    'Floats' : IDL.Vec(IDL.Float64),
    'Float' : IDL.Float64,
    'Principal' : IDL.Principal,
    'Array' : IDL.Vec(Box),
    'Class' : IDL.Vec(PropertyShared),
  });
  const ICTokenSpec_standard = IDL.Variant({
    'ICRC1' : IDL.Null,
    'EXTFungible' : IDL.Null,
    'DIP20' : IDL.Null,
    'Other' : CandyShared,
    'Ledger' : IDL.Null,
  });
  const ICTokenSpec = IDL.Record({
    'id' : IDL.Opt(IDL.Nat),
    'fee' : IDL.Opt(IDL.Nat),
    'decimals' : IDL.Nat,
    'canister' : IDL.Principal,
    'standard' : ICTokenSpec_standard,
    'symbol' : IDL.Text,
  });
  const TokenSpec = IDL.Variant({
    'ic' : ICTokenSpec,
    'extensible' : CandyShared,
  });
  const Account_1 = IDL.Variant({
    'account_id' : IDL.Text,
    'principal' : IDL.Principal,
    'extensible' : CandyShared,
    'account' : IDL.Record({
      'owner' : IDL.Principal,
      'sub_account' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    }),
  });
  const EscrowReceipt = IDL.Record({
    'token' : TokenSpec,
    'token_id' : IDL.Text,
    'seller' : Account_1,
    'buyer' : Account_1,
    'amount' : IDL.Nat,
  });
  const AskFeature_min_increase = IDL.Variant({
    'amount' : IDL.Nat,
    'percentage' : IDL.Float64,
  });
  const DutchParams_time_unit = IDL.Variant({
    'day' : IDL.Nat,
    'hour' : IDL.Nat,
    'minute' : IDL.Nat,
  });
  const DutchParams_decay_type = IDL.Variant({
    'flat' : IDL.Nat,
    'percent' : IDL.Float64,
  });
  const DutchParams = IDL.Record({
    'time_unit' : DutchParams_time_unit,
    'decay_type' : DutchParams_decay_type,
  });
  const AskFeature_ending = IDL.Variant({
    'date' : IDL.Int,
    'timeout' : IDL.Nat,
  });
  const AskFeature = IDL.Variant({
    'kyc' : IDL.Principal,
    'start_price' : IDL.Nat,
    'token' : TokenSpec,
    'notify' : IDL.Vec(IDL.Principal),
    'wait_for_quiet' : IDL.Record({
      'max' : IDL.Nat,
      'fade' : IDL.Float64,
      'extension' : IDL.Nat64,
    }),
    'reserve' : IDL.Nat,
    'start_date' : IDL.Int,
    'min_increase' : AskFeature_min_increase,
    'allow_list' : IDL.Vec(IDL.Principal),
    'buy_now' : IDL.Nat,
    'nifty_settlement' : IDL.Record({
      'fixed' : IDL.Bool,
      'interestRatePerSecond' : IDL.Float64,
      'duration' : IDL.Opt(IDL.Int),
      'expiration' : IDL.Opt(IDL.Int),
      'lenderOffer' : IDL.Bool,
    }),
    'atomic' : IDL.Null,
    'dutch' : DutchParams,
    'ending' : AskFeature_ending,
  });
  const AuctionConfig__1_ending = IDL.Variant({
    'date' : IDL.Int,
    'wait_for_quiet' : IDL.Record({
      'max' : IDL.Nat,
      'date' : IDL.Int,
      'fade' : IDL.Float64,
      'extension' : IDL.Nat64,
    }),
  });
  const AuctionConfig__1 = IDL.Record({
    'start_price' : IDL.Nat,
    'token' : TokenSpec,
    'reserve' : IDL.Opt(IDL.Nat),
    'start_date' : IDL.Int,
    'min_increase' : AskFeature_min_increase,
    'allow_list' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'buy_now' : IDL.Opt(IDL.Nat),
    'ending' : AuctionConfig__1_ending,
  });
  const PricingConfigShared__1 = IDL.Variant({
    'ask' : IDL.Opt(IDL.Vec(AskFeature)),
    'extensible' : CandyShared,
    'instant' : IDL.Null,
    'auction' : AuctionConfig__1,
  });
  const AuctionStateShared = IDL.Record({
    'status' : AuctionStateShared_status,
    'participants' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Int)),
    'token' : TokenSpec,
    'current_bid_amount' : IDL.Nat,
    'winner' : IDL.Opt(Account_1),
    'end_date' : IDL.Int,
    'start_date' : IDL.Int,
    'wait_for_quiet_count' : IDL.Opt(IDL.Nat),
    'current_escrow' : IDL.Opt(EscrowReceipt),
    'allow_list' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Bool))),
    'current_broker_id' : IDL.Opt(IDL.Principal),
    'min_next_bid' : IDL.Nat,
    'config' : PricingConfigShared__1,
  });
  const SaleStatusShared_sale_type = IDL.Variant({
    'auction' : AuctionStateShared,
  });
  const SaleStatusShared = IDL.Record({
    'token_id' : IDL.Text,
    'sale_type' : SaleStatusShared_sale_type,
    'broker_id' : IDL.Opt(IDL.Principal),
    'original_broker_id' : IDL.Opt(IDL.Principal),
    'sale_id' : IDL.Text,
  });
  const SubAccountInfo_account = IDL.Record({
    'principal' : IDL.Principal,
    'sub_account' : IDL.Vec(IDL.Nat8),
  });
  const SubAccountInfo = IDL.Record({
    'account_id' : IDL.Vec(IDL.Nat8),
    'principal' : IDL.Principal,
    'account_id_text' : IDL.Text,
    'account' : SubAccountInfo_account,
  });
  const SubscriberNotification = IDL.Record({
    'collection' : IDL.Principal,
    'sale' : SaleStatusShared,
    'seller' : Account_1,
    'escrow_info' : SubAccountInfo,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const CollectMetricsRequestType = IDL.Variant({
    'force' : IDL.Null,
    'normal' : IDL.Null,
  });
  const UpdateInformationRequest = IDL.Record({
    'metrics' : IDL.Opt(CollectMetricsRequestType),
  });
  return IDL.Service({
    'getCanistergeekInformation' : IDL.Func(
        [GetInformationRequest],
        [GetInformationResponse],
        ['query'],
      ),
    'get_conf' : IDL.Func([], [Conf], []),
    'get_records' : IDL.Func(
        [GetRecordsRequest],
        [GetRecordsResponse],
        ['query'],
      ),
    'get_status_of_swap' : IDL.Func([GetStatusRequest], [Result], ['query']),
    'get_swaps_by_user' : IDL.Func(
        [IDL.Opt(Account), IDL.Opt(IDL.Nat32), IDL.Opt(IDL.Nat32)],
        [Result_1],
        ['query'],
      ),
    'nft_info' : IDL.Func([InfoRequest], [NftInfo], []),
    'notify_sale_nft_origyn' : IDL.Func(
        [SubscriberNotification],
        [Result_2],
        [],
      ),
    'updateCanistergeekInformation' : IDL.Func(
        [UpdateInformationRequest],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const NftCanisterConf = IDL.Record({ 'grams' : IDL.Nat16 });
  const Conf = IDL.Record({
    'gld_nft_canister_ids' : IDL.Vec(IDL.Tuple(IDL.Principal, NftCanisterConf)),
    'gldt_ledger_canister_id' : IDL.Principal,
  });
  return [IDL.Opt(Conf)];
};
