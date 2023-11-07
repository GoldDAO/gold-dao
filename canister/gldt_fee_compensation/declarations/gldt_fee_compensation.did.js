export const idlFactory = ({ IDL }) => {
  const GldNftConf = IDL.Record({
    'weight' : IDL.Nat16,
    'gld_nft_canister_id' : IDL.Principal,
    'last_query_index' : IDL.Nat,
  });
  const Conf = IDL.Record({
    'gld_nft_canister_conf' : IDL.Vec(GldNftConf),
    'gldt_canister_id' : IDL.Principal,
    'fallback_timer_interval_secs' : IDL.Nat64,
    'enabled' : IDL.Bool,
    'gldt_ledger_canister_id' : IDL.Principal,
    'execution_delay_secs' : IDL.Nat64,
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
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Null });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Null });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Null });
  const ErrorType = IDL.Variant({
    'TransferError' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'CompensationDisabled' : IDL.Null,
    'Other' : IDL.Null,
  });
  const CustomError = IDL.Record({
    'error_message' : IDL.Opt(IDL.Text),
    'error_type' : ErrorType,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : CustomError });
  const CollectMetricsRequestType = IDL.Variant({
    'force' : IDL.Null,
    'normal' : IDL.Null,
  });
  const UpdateInformationRequest = IDL.Record({
    'metrics' : IDL.Opt(CollectMetricsRequestType),
  });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'getCanistergeekInformation' : IDL.Func(
        [GetInformationRequest],
        [GetInformationResponse],
        ['query'],
      ),
    'get_balance' : IDL.Func([], [Result], []),
    'get_compensation_enabled' : IDL.Func([], [Result_1], ['query']),
    'get_execution_delay_secs' : IDL.Func([], [Result_2], ['query']),
    'get_fallback_timer_interval_secs' : IDL.Func([], [Result_2], ['query']),
    'get_gld_nft_conf' : IDL.Func([], [IDL.Vec(GldNftConf)], ['query']),
    'notify_compensation_job' : IDL.Func([], [Result_3], []),
    'set_compensation_enabled' : IDL.Func([IDL.Bool], [Result_3], []),
    'set_execution_delay_secs' : IDL.Func([IDL.Nat64], [Result_3], []),
    'set_fallback_timer_interval_secs' : IDL.Func([IDL.Nat64], [Result_3], []),
    'set_gld_nft_conf' : IDL.Func([IDL.Vec(GldNftConf)], [Result_3], []),
    'updateCanistergeekInformation' : IDL.Func(
        [UpdateInformationRequest],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const GldNftConf = IDL.Record({
    'weight' : IDL.Nat16,
    'gld_nft_canister_id' : IDL.Principal,
    'last_query_index' : IDL.Nat,
  });
  const Conf = IDL.Record({
    'gld_nft_canister_conf' : IDL.Vec(GldNftConf),
    'gldt_canister_id' : IDL.Principal,
    'fallback_timer_interval_secs' : IDL.Nat64,
    'enabled' : IDL.Bool,
    'gldt_ledger_canister_id' : IDL.Principal,
    'execution_delay_secs' : IDL.Nat64,
  });
  return [IDL.Opt(Conf)];
};
