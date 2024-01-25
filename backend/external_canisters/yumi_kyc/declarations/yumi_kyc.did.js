export const idlFactory = ({ IDL }) => {
  const CandyShared = IDL.Rec();
  const PropertyShared = IDL.Record({
    'value' : CandyShared,
    'name' : IDL.Text,
    'immutable' : IDL.Bool,
  });
  CandyShared.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(CandyShared, CandyShared)),
      'Nat' : IDL.Nat,
      'Set' : IDL.Vec(CandyShared),
      'Nat16' : IDL.Nat16,
      'Nat32' : IDL.Nat32,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Bool' : IDL.Bool,
      'Int8' : IDL.Int8,
      'Nat8' : IDL.Nat8,
      'Nats' : IDL.Vec(IDL.Nat),
      'Text' : IDL.Text,
      'Bytes' : IDL.Vec(IDL.Nat8),
      'Int16' : IDL.Int16,
      'Int32' : IDL.Int32,
      'Int64' : IDL.Int64,
      'Option' : IDL.Opt(CandyShared),
      'Floats' : IDL.Vec(IDL.Float64),
      'Float' : IDL.Float64,
      'Principal' : IDL.Principal,
      'Array' : IDL.Vec(CandyShared),
      'Class' : IDL.Vec(PropertyShared),
    })
  );
  const KYCAccount = IDL.Variant({
    'ICRC1' : IDL.Record({
      'owner' : IDL.Principal,
      'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    }),
    'Account' : IDL.Vec(IDL.Nat8),
    'Extensible' : CandyShared,
  });
  const KYCLevel = IDL.Variant({
    'NA' : IDL.Null,
    'Tier1' : IDL.Null,
    'Tier2' : IDL.Null,
    'Tier3' : IDL.Null,
  });
  const UpdateKycStatusReq = IDL.Record({
    'kycLevel' : KYCLevel,
    'account' : KYCAccount,
  });
  const Rate = IDL.Record({ 'percison' : IDL.Nat, 'rate' : IDL.Nat });
  const Channel = IDL.Variant({
    'Gold' : IDL.Null,
    'Land' : IDL.Null,
    'Co_owned' : IDL.Null,
    'Yumi' : IDL.Null,
  });
  const Access = IDL.Variant({
    'Limit' : IDL.Nat,
    'Fail' : IDL.Null,
    'Pass' : IDL.Null,
  });
  const Workflow = IDL.Record({
    'workflow_id' : IDL.Text,
    'applicant_id' : IDL.Text,
    'workflow_run_id' : IDL.Text,
  });
  const RiskAssessment = IDL.Variant({
    'Low' : IDL.Null,
    'High' : IDL.Null,
    'Medium' : IDL.Null,
  });
  const UpgradStatus = IDL.Variant({
    'UnderReview' : IDL.Null,
    'None' : IDL.Null,
    'AwaitingInput' : IDL.Null,
  });
  const UserInfo = IDL.Record({ 'userName' : IDL.Text, 'email' : IDL.Text });
  const GetLogMessagesFilter = IDL.Record({
    'analyzeCount' : IDL.Nat32,
    'messageRegex' : IDL.Opt(IDL.Text),
    'messageContains' : IDL.Opt(IDL.Text),
  });
  const Nanos = IDL.Nat64;
  const GetLogMessagesParameters = IDL.Record({
    'count' : IDL.Nat32,
    'filter' : IDL.Opt(GetLogMessagesFilter),
    'fromTimeNanos' : IDL.Opt(Nanos),
  });
  const GetLatestLogMessagesParameters = IDL.Record({
    'upToTimeNanos' : IDL.Opt(Nanos),
    'count' : IDL.Nat32,
    'filter' : IDL.Opt(GetLogMessagesFilter),
  });
  const CanisterLogRequest = IDL.Variant({
    'getMessagesInfo' : IDL.Null,
    'getMessages' : GetLogMessagesParameters,
    'getLatestMessages' : GetLatestLogMessagesParameters,
  });
  const CanisterLogFeature = IDL.Variant({
    'filterMessageByContains' : IDL.Null,
    'filterMessageByRegex' : IDL.Null,
  });
  const CanisterLogMessagesInfo = IDL.Record({
    'features' : IDL.Vec(IDL.Opt(CanisterLogFeature)),
    'lastTimeNanos' : IDL.Opt(Nanos),
    'count' : IDL.Nat32,
    'firstTimeNanos' : IDL.Opt(Nanos),
  });
  const LogMessagesData = IDL.Record({
    'timeNanos' : Nanos,
    'message' : IDL.Text,
  });
  const CanisterLogMessages = IDL.Record({
    'data' : IDL.Vec(LogMessagesData),
    'lastAnalyzedMessageTimeNanos' : IDL.Opt(Nanos),
  });
  const CanisterLogResponse = IDL.Variant({
    'messagesInfo' : CanisterLogMessagesInfo,
    'messages' : CanisterLogMessages,
  });
  const ICTokenSpec = IDL.Record({
    'id' : IDL.Opt(IDL.Nat),
    'fee' : IDL.Opt(IDL.Nat),
    'decimals' : IDL.Nat,
    'canister' : IDL.Principal,
    'standard' : IDL.Variant({
      'ICRC1' : IDL.Null,
      'EXTFungible' : IDL.Null,
      'DIP20' : IDL.Null,
      'Other' : CandyShared,
      'Ledger' : IDL.Null,
    }),
    'symbol' : IDL.Text,
  });
  const TokenSpec__1 = IDL.Variant({
    'IC' : ICTokenSpec,
    'Extensible' : CandyShared,
  });
  const TokenSpec = IDL.Variant({
    'IC' : ICTokenSpec,
    'Extensible' : CandyShared,
  });
  const KYCAccount__1 = IDL.Variant({
    'ICRC1' : IDL.Record({
      'owner' : IDL.Principal,
      'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    }),
    'Account' : IDL.Vec(IDL.Nat8),
    'Extensible' : CandyShared,
  });
  const KYCNotification = IDL.Record({
    'token' : IDL.Opt(TokenSpec),
    'metadata' : IDL.Opt(CandyShared),
    'counterparty' : KYCAccount__1,
    'amount' : IDL.Opt(IDL.Nat),
  });
  const KYCCanisterRequest = IDL.Record({
    'token' : IDL.Opt(TokenSpec),
    'counterparty' : KYCAccount__1,
    'extensible' : IDL.Opt(CandyShared),
    'amount' : IDL.Opt(IDL.Nat),
  });
  const VerificationResult = IDL.Variant({
    'NA' : IDL.Null,
    'Fail' : IDL.Null,
    'Pass' : IDL.Null,
  });
  const KYCResult = IDL.Record({
    'aml' : VerificationResult,
    'kyc' : VerificationResult,
    'token' : IDL.Opt(TokenSpec),
    'message' : IDL.Opt(IDL.Text),
    'amount' : IDL.Opt(IDL.Nat),
  });
  const ExRate = IDL.Record({ 'pair' : IDL.Text, 'rate' : Rate });
  const Kyc = IDL.Service({
    'addSubmitKyc' : IDL.Func([KYCAccount], [], []),
    'addwhitelist' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'batch_update_kyc_status' : IDL.Func([IDL.Vec(UpdateKycStatusReq)], [], []),
    'delWhitelist' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'del_primary_link' : IDL.Func([IDL.Principal], [], ['oneway']),
    'deleUserRiskAssessment' : IDL.Func([IDL.Principal], [], []),
    'deleUserUpgradStatus' : IDL.Func([IDL.Principal], [], []),
    'deleteBatchKycPendingUserInfo' : IDL.Func([IDL.Vec(IDL.Text)], [], []),
    'deletePrincipalWorkflow' : IDL.Func([IDL.Principal, IDL.Text], [], []),
    'flushUsersRiskAssessment' : IDL.Func([], [], []),
    'flushUsersUpgradStatus' : IDL.Func([], [], []),
    'getAllExRates' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, Rate))],
        ['query'],
      ),
    'getAllKycAccess' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(Channel, IDL.Vec(IDL.Tuple(KYCLevel, Access))))],
        ['query'],
      ),
    'getAllKycStatus' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(KYCAccount, KYCLevel))],
        ['query'],
      ),
    'getAllKycStatusUpdateRecord' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        ['query'],
      ),
    'getAllKycTier3Limit' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(KYCAccount, IDL.Nat))],
        ['query'],
      ),
    'getAllKycWorkflows' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Vec(Workflow)))],
        ['query'],
      ),
    'getAllRiskAssessment' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, RiskAssessment))],
        ['query'],
      ),
    'getAllRouters' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Channel))],
        ['query'],
      ),
    'getAllTradeAmount' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))],
        ['query'],
      ),
    'getAllUserUpgradStatus' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, UpgradStatus))],
        ['query'],
      ),
    'getBatchKycPendingUserInfo' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, UserInfo))],
        ['query'],
      ),
    'getBatchPricipalWorkflow' : IDL.Func(
        [IDL.Vec(IDL.Principal)],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Vec(Workflow)))],
        ['query'],
      ),
    'getCanisterLog' : IDL.Func(
        [IDL.Opt(CanisterLogRequest)],
        [IDL.Opt(CanisterLogResponse)],
        ['query'],
      ),
    'getCustomerKyc' : IDL.Func([], [IDL.Vec(KYCAccount)], ['query']),
    'getDecimal' : IDL.Func([], [IDL.Nat], ['query']),
    'getEntrieKycPendingUserInfo' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, UserInfo))],
        ['query'],
      ),
    'getExRate' : IDL.Func([IDL.Text], [Rate], ['query']),
    'getExRateByToken' : IDL.Func([TokenSpec__1], [Rate, IDL.Nat], ['query']),
    'getKycStatus' : IDL.Func([KYCAccount], [IDL.Opt(KYCLevel)], ['query']),
    'getKycStatusUpdateRecord' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        ['query'],
      ),
    'getOwner' : IDL.Func([], [IDL.Principal], ['query']),
    'getPricipalWorkflow' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Vec(Workflow))],
        ['query'],
      ),
    'getUserRiskAssessment' : IDL.Func(
        [IDL.Principal],
        [RiskAssessment],
        ['query'],
      ),
    'getUserTradeAmount' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Nat)],
        ['query'],
      ),
    'getUserUpgradStatus' : IDL.Func(
        [IDL.Principal],
        [UpgradStatus],
        ['query'],
      ),
    'getWhitelist' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'icrc17_kyc_notification' : IDL.Func([KYCNotification], [], ['oneway']),
    'icrc17_kyc_notification_for_yumi' : IDL.Func(
        [Channel, KYCNotification],
        [],
        ['oneway'],
      ),
    'icrc17_kyc_request' : IDL.Func([KYCCanisterRequest], [KYCResult], []),
    'icrc17_kyc_request_for_yumi' : IDL.Func(
        [Channel, KYCCanisterRequest],
        [KYCResult],
        [],
      ),
    'inputPricipalWorkflow' : IDL.Func([IDL.Principal, Workflow], [], []),
    'inputkycPendingUserInfo' : IDL.Func([IDL.Text, UserInfo], [], []),
    'resetTradeAmount' : IDL.Func([], [], []),
    'setDecimal' : IDL.Func([IDL.Nat], [], []),
    'setExRate' : IDL.Func([IDL.Vec(ExRate)], [], []),
    'setKycAccess' : IDL.Func([Channel, KYCLevel, Access], [], []),
    'setKycTier3Limit' : IDL.Func([IDL.Principal, IDL.Nat], [], []),
    'setOwner' : IDL.Func([IDL.Principal], [], []),
    'setRouter' : IDL.Func([IDL.Principal, Channel], [], []),
    'set_primary_link' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [],
        ['oneway'],
      ),
    'updateKycStatusRecord' : IDL.Func([IDL.Vec(IDL.Text), IDL.Text], [], []),
    'updateUsersRiskAssessment' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Principal, RiskAssessment))],
        [],
        [],
      ),
    'updateUsersUpgradStatus' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Principal, UpgradStatus))],
        [],
        [],
      ),
    'user_kyc_request' : IDL.Func([KYCAccount], [KYCLevel], ['query']),
  });
  return Kyc;
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
