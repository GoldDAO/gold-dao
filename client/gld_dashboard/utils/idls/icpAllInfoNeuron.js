/* eslint-disable camelcase */
/* eslint-disable no-unused-vars */

export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({ test_mode: IDL.Bool });
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const DepositAccount = IDL.Record({
    legacy_account_id: IDL.Text,
    icrc_account: Account,
  });
  const NeuronWithMetric = IDL.Record({
    id: IDL.Nat64,
    dissolve_delay: IDL.Nat64,
    maturity: IDL.Nat64,
    staked_amount: IDL.Nat64,
    deposit_account: IDL.Opt(DepositAccount),
    dissolving: IDL.Bool,
  });
  const NeuronList = IDL.Record({
    active: IDL.Vec(NeuronWithMetric),
    disbursed: IDL.Vec(IDL.Nat64),
    spawning: IDL.Vec(IDL.Nat64),
  });
  const ListNeuronsResponse = IDL.Record({ neurons: NeuronList });
  const Spawn = IDL.Record({
    percentage_to_spawn: IDL.Opt(IDL.Nat32),
    new_controller: IDL.Opt(IDL.Principal),
    nonce: IDL.Opt(IDL.Nat64),
  });
  const Split = IDL.Record({ amount_e8s: IDL.Nat64 });
  const NeuronId = IDL.Record({ id: IDL.Nat64 });
  const Follow = IDL.Record({
    topic: IDL.Int32,
    followees: IDL.Vec(NeuronId),
  });
  const MemoAndController = IDL.Record({
    controller: IDL.Opt(IDL.Principal),
    memo: IDL.Nat64,
  });
  const By = IDL.Variant({
    NeuronIdOrSubaccount: IDL.Record({}),
    MemoAndController,
    Memo: IDL.Nat64,
  });
  const ClaimOrRefresh = IDL.Record({ by: IDL.Opt(By) });
  const RemoveHotKey = IDL.Record({
    hot_key_to_remove: IDL.Opt(IDL.Principal),
  });
  const AddHotKey = IDL.Record({ new_hot_key: IDL.Opt(IDL.Principal) });
  const ChangeAutoStakeMaturity = IDL.Record({
    requested_setting_for_auto_stake_maturity: IDL.Bool,
  });
  const IncreaseDissolveDelay = IDL.Record({
    additional_dissolve_delay_seconds: IDL.Nat32,
  });
  const SetDissolveTimestamp = IDL.Record({
    dissolve_timestamp_seconds: IDL.Nat64,
  });
  const Operation = IDL.Variant({
    RemoveHotKey,
    AddHotKey,
    ChangeAutoStakeMaturity,
    StopDissolving: IDL.Record({}),
    StartDissolving: IDL.Record({}),
    IncreaseDissolveDelay,
    JoinCommunityFund: IDL.Record({}),
    LeaveCommunityFund: IDL.Record({}),
    SetDissolveTimestamp,
  });
  const Configure = IDL.Record({ operation: IDL.Opt(Operation) });
  const RegisterVote = IDL.Record({
    vote: IDL.Int32,
    proposal: IDL.Opt(NeuronId),
  });
  const Merge = IDL.Record({ source_neuron_id: IDL.Opt(NeuronId) });
  const DisburseToNeuron = IDL.Record({
    dissolve_delay_seconds: IDL.Nat64,
    kyc_verified: IDL.Bool,
    amount_e8s: IDL.Nat64,
    new_controller: IDL.Opt(IDL.Principal),
    nonce: IDL.Nat64,
  });
  const StakeMaturity = IDL.Record({
    percentage_to_stake: IDL.Opt(IDL.Nat32),
  });
  const MergeMaturity = IDL.Record({ percentage_to_merge: IDL.Nat32 });
  const AccountIdentifier = IDL.Record({ hash: IDL.Vec(IDL.Nat8) });
  const Amount = IDL.Record({ e8s: IDL.Nat64 });
  const Disburse = IDL.Record({
    to_account: IDL.Opt(AccountIdentifier),
    amount: IDL.Opt(Amount),
  });
  const Command = IDL.Variant({
    Spawn,
    Split,
    Follow,
    ClaimOrRefresh,
    Configure,
    RegisterVote,
    Merge,
    DisburseToNeuron,
    StakeMaturity,
    MergeMaturity,
    Disburse,
  });
  const ManageNnsNeuronRequest = IDL.Record({
    command: Command,
    neuron_id: IDL.Nat64,
  });
  const ManageNnsNeuronResponse = IDL.Variant({
    Success: IDL.Text,
    InternalError: IDL.Text,
  });
  const RewardsRecipient = IDL.Record({
    tag: IDL.Text,
    reward_weight: IDL.Nat16,
    account: Account,
  });
  const ManageRewardRecipientsRequest = IDL.Record({
    list: IDL.Vec(RewardsRecipient),
  });
  const ManageRewardRecipientsResponse = IDL.Variant({
    Success: IDL.Null,
    InternalError: IDL.Text,
  });
  const StakeNnsNeuronResponse = IDL.Variant({
    Success: IDL.Nat64,
    InternalError: IDL.Text,
  });
  return IDL.Service({
    list_neurons: IDL.Func([], [ListNeuronsResponse], ['query']),
    manage_nns_neuron: IDL.Func([ManageNnsNeuronRequest], [ManageNnsNeuronResponse], []),
    manage_reward_recipients: IDL.Func(
      [ManageRewardRecipientsRequest],
      [ManageRewardRecipientsResponse],
      [],
    ),
    stake_nns_neuron: IDL.Func([], [StakeNnsNeuronResponse], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({ test_mode: IDL.Bool });
  return [IDL.Opt(InitArgs)];
};
