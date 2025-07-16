export const idlFactory = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const InitArgs = IDL.Record({
    sns_gov_canister_id: IDL.Principal,
    test_mode: IDL.Bool,
    version: BuildVersion,
    ogy_ledger_canister_id: IDL.Principal,
    icp_ledger_canister_id: IDL.Principal,
    sns_ledger_canister_id: IDL.Principal,
    commit_hash: IDL.Text,
  });
  const Args_4 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  const NeuronId = IDL.Record({ id: IDL.Vec(IDL.Nat8) });
  const Args = IDL.Record({ token: IDL.Text, neuron_id: NeuronId });
  const Response = IDL.Variant({
    Ok: IDL.Bool,
    TokenSymbolInvalid: IDL.Text,
    NeuronNotClaimed: IDL.Null,
    NeuronOwnerInvalid: IDL.Opt(IDL.Principal),
    NeuronHotKeyInvalid: IDL.Null,
    TransferFailed: IDL.Text,
    NeuronDoesNotExist: IDL.Null,
    InternalError: IDL.Text,
  });
  const TokenSymbol = IDL.Variant({
    ICP: IDL.Null,
    OGY: IDL.Null,
    WTN: IDL.Null,
    GOLDAO: IDL.Null,
    GLDT: IDL.Null,
  });
  const ClaimRewardArgs = IDL.Record({
    token: TokenSymbol,
    neuron_id: NeuronId,
  });
  const Args_1 = IDL.Record({ claim_reward_args: IDL.Vec(ClaimRewardArgs) });
  const ClaimRewardErrorType = IDL.Variant({
    TokenSymbolInvalid: TokenSymbol,
    NeuronNotClaimed: IDL.Null,
    NeuronOwnerInvalid: IDL.Opt(IDL.Principal),
    NeuronHotKeyInvalid: IDL.Null,
    TransferFailed: IDL.Text,
    NeuronDoesNotExist: IDL.Null,
    InternalError: IDL.Text,
  });
  const ClaimRewardError = IDL.Record({
    token: IDL.Opt(TokenSymbol),
    error: ClaimRewardErrorType,
    neuron_id: NeuronId,
  });
  const Result = IDL.Variant({
    Ok: IDL.Null,
    Err: IDL.Vec(ClaimRewardError),
  });
  const Response_1 = IDL.Variant({
    Success: IDL.Null,
    InternalError: IDL.Text,
  });
  const Args_2 = IDL.Record({
    transfer_amounts: IDL.Vec(IDL.Tuple(TokenSymbol, IDL.Nat)),
  });
  const TokenInfo = IDL.Record({
    fee: IDL.Nat64,
    decimals: IDL.Nat64,
    ledger_id: IDL.Principal,
  });
  const Args_3 = IDL.Record({
    token_list: IDL.Vec(IDL.Tuple(TokenSymbol, TokenInfo)),
  });
  const Response_2 = IDL.Variant({
    Success: IDL.Null,
    InternalError: IDL.Text,
  });
  return IDL.Service({
    claim_reward: IDL.Func([Args], [Response], []),
    claim_rewards_batch: IDL.Func([Args_1], [Result], []),
    set_daily_goldao_burn_rate: IDL.Func([IDL.Nat], [Response_1], []),
    set_reserve_transfer_amounts: IDL.Func([Args_2], [Response_1], []),
    set_reward_token_types: IDL.Func([Args_3], [Response_2], []),
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
  const InitArgs = IDL.Record({
    sns_gov_canister_id: IDL.Principal,
    test_mode: IDL.Bool,
    version: BuildVersion,
    ogy_ledger_canister_id: IDL.Principal,
    icp_ledger_canister_id: IDL.Principal,
    sns_ledger_canister_id: IDL.Principal,
    commit_hash: IDL.Text,
  });
  const Args_4 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  return [Args_4];
};
