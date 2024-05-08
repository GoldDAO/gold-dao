export default({ IDL }) => {
  const InitArgs = IDL.Record({
    'sns_gov_canister_id' : IDL.Principal,
    'test_mode' : IDL.Bool,
    'ogy_ledger_canister_id' : IDL.Principal,
    'icp_ledger_canister_id' : IDL.Principal,
    'sns_ledger_canister_id' : IDL.Principal,
  });
  const NeuronId = IDL.Record({ 'id' : IDL.Vec(IDL.Nat8) });
  const Response = IDL.Variant({
    'Ok' : NeuronId,
    'NeuronHotKeyAbsent' : IDL.Null,
    'NeuronOwnerInvalid' : IDL.Opt(IDL.Principal),
    'NeuronHotKeyInvalid' : IDL.Null,
    'NeuronDoesNotExist' : IDL.Null,
    'InternalError' : IDL.Text,
  });
  const Args = IDL.Record({ 'token' : IDL.Text, 'neuron_id' : NeuronId });
  const Response_1 = IDL.Variant({
    'Ok' : IDL.Bool,
    'NeuronHotKeyAbsent' : IDL.Null,
    'TokenSymbolInvalid' : IDL.Text,
    'NeuronNotClaimed' : IDL.Null,
    'NeuronOwnerInvalid' : IDL.Opt(IDL.Principal),
    'NeuronHotKeyInvalid' : IDL.Null,
    'TransferFailed' : IDL.Text,
    'NeuronDoesNotExist' : IDL.Null,
    'InternalError' : IDL.Text,
  });
  const Response_2 = IDL.Variant({
    'Ok' : NeuronId,
    'NeuronHotKeyAbsent' : IDL.Null,
    'NeuronNotClaimed' : IDL.Null,
    'NeuronOwnerInvalid' : IDL.Opt(IDL.Principal),
    'NeuronHotKeyInvalid' : IDL.Null,
    'NeuronDoesNotExist' : IDL.Null,
    'InternalError' : IDL.Text,
  });
  const Response_3 = IDL.Variant({
    'Success' : IDL.Null,
    'InternalError' : IDL.Text,
  });
  const Args_1 = IDL.Record({
    'transfer_amounts' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat)),
  });
  const TokenInfo = IDL.Record({
    'fee' : IDL.Nat64,
    'decimals' : IDL.Nat64,
    'ledger_id' : IDL.Principal,
  });
  const Args_2 = IDL.Record({
    'token_list' : IDL.Vec(IDL.Tuple(IDL.Text, TokenInfo)),
  });
  const Response_4 = IDL.Variant({
    'Success' : IDL.Null,
    'InternalError' : IDL.Text,
  });
  return IDL.Service({
    'add_neuron_ownership' : IDL.Func([NeuronId], [Response], []),
    'claim_reward' : IDL.Func([Args], [Response_1], []),
    'get_neurons_by_owner' : IDL.Func(
        [],
        [IDL.Opt(IDL.Vec(NeuronId))],
        ['query'],
      ),
    'remove_neuron_ownership' : IDL.Func([NeuronId], [Response_2], []),
    'set_daily_gldgov_burn_rate' : IDL.Func([IDL.Nat], [Response_3], []),
    'set_reserve_transfer_amounts' : IDL.Func([Args_1], [Response_3], []),
    'set_reward_token_types' : IDL.Func([Args_2], [Response_4], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'sns_gov_canister_id' : IDL.Principal,
    'test_mode' : IDL.Bool,
    'ogy_ledger_canister_id' : IDL.Principal,
    'icp_ledger_canister_id' : IDL.Principal,
    'sns_ledger_canister_id' : IDL.Principal,
  });
  return [InitArgs];
};