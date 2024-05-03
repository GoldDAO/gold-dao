export default ({ IDL }) => {
  const Args = IDL.Record({ test_mode: IDL.Bool });
  const NeuronId = IDL.Record({ id: IDL.Vec(IDL.Nat8) });
  const UserClaimErrorResponse = IDL.Variant({
    NeuronHotKeyAbsent: IDL.Null,
    TokenSymbolInvalid: IDL.Text,
    NeuronNotClaimed: IDL.Null,
    NeuronOwnerInvalid: IDL.Opt(IDL.Principal),
    NeuronHotKeyInvalid: IDL.Null,
    TransferFailed: IDL.Text,
    NeuronDoesNotExist: IDL.Null,
    InternalError: IDL.Text,
  });
  const Result = IDL.Variant({
    Ok: NeuronId,
    Err: UserClaimErrorResponse,
  });
  const Result_1 = IDL.Variant({
    Ok: IDL.Bool,
    Err: UserClaimErrorResponse,
  });
  const SetReserveTransferAmountRequest = IDL.Record({
    transfer_amounts: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat)),
  });
  const SetReserveTransferAmountResponse = IDL.Variant({
    Success: IDL.Null,
    InternalError: IDL.Text,
  });
  const Result_2 = IDL.Variant({
    Ok: SetReserveTransferAmountResponse,
    Err: SetReserveTransferAmountResponse,
  });
  const TokenInfo = IDL.Record({
    fee: IDL.Nat64,
    decimals: IDL.Nat64,
    ledger_id: IDL.Principal,
  });
  const SetRewardTokenTypesRequest = IDL.Record({
    token_list: IDL.Vec(IDL.Tuple(IDL.Text, TokenInfo)),
  });
  const SetRewardTokenTypesResponse = IDL.Variant({
    Success: IDL.Null,
    InternalError: IDL.Text,
  });
  return IDL.Service({
    add_neuron_ownership: IDL.Func([NeuronId], [Result], []),
    claim_reward: IDL.Func([NeuronId, IDL.Text], [Result_1], []),
    get_neurons_by_owner: IDL.Func(
      [],
      [IDL.Opt(IDL.Vec(NeuronId))],
      ['query'],
    ),
    remove_neuron_ownership: IDL.Func([NeuronId], [Result], []),
    set_reserve_transfer_amounts: IDL.Func(
      [SetReserveTransferAmountRequest],
      [Result_2],
      [],
    ),
    set_reward_token_types: IDL.Func(
      [SetRewardTokenTypesRequest],
      [SetRewardTokenTypesResponse],
      [],
    ),
  });
};
export const init = ({ IDL }) => {
  const Args = IDL.Record({ test_mode: IDL.Bool });
  return [Args];
};
