export const idlFactory = ({ IDL }) => {
  const ActiveUsers = IDL.Record({
    active_principals_count: IDL.Nat64,
    active_accounts_count: IDL.Nat64,
  });
  const Overview = IDL.Record({
    balance: IDL.Nat,
    sent: IDL.Tuple(IDL.Nat32, IDL.Nat),
    last_active: IDL.Nat64,
    first_active: IDL.Nat64,
    received: IDL.Tuple(IDL.Nat32, IDL.Nat),
    max_balance: IDL.Nat,
  });
  const GovernanceStats = IDL.Record({
    total_rewards: IDL.Nat,
    total_staked: IDL.Nat,
    total_locked: IDL.Nat,
    total_unlocked: IDL.Nat,
  });
  const WalletOverview = IDL.Record({
    total: IDL.Nat64,
    ledger: Overview,
    governance: GovernanceStats,
  });
  const GetHoldersArgs = IDL.Record({
    offset: IDL.Nat64,
    limit: IDL.Nat64,
    merge_accounts_to_principals: IDL.Bool,
  });
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const GetHoldersResponse = IDL.Record({
    current_offset: IDL.Nat64,
    data: IDL.Vec(IDL.Tuple(Account, WalletOverview)),
    limit: IDL.Nat64,
    total_count: IDL.Nat64,
  });
  const LockedNeuronsAmount = IDL.Record({
    one_year: IDL.Nat64,
    two_years: IDL.Nat64,
    three_years: IDL.Nat64,
    four_years: IDL.Nat64,
    five_years: IDL.Nat64,
  });
  const ProposalsMetrics = IDL.Record({
    daily_voting_rewards: IDL.Nat64,
    reward_base_current_year: IDL.Nat64,
    average_voting_participation: IDL.Nat64,
    average_voting_power: IDL.Nat64,
    total_voting_power: IDL.Nat64,
    total_proposals: IDL.Nat64,
  });
  const HistoryData = IDL.Record({ balance: IDL.Nat });
  const TokenSupplyData = IDL.Record({
    circulating_supply: IDL.Nat,
    total_supply: IDL.Nat,
  });
  const GetVotingParticipationHistoryArgs = IDL.Record({ days: IDL.Nat64 });
  const GetVotingPowerRatioHistory = IDL.Record({ days: IDL.Nat64 });
  return IDL.Service({
    get_active_users_count: IDL.Func([], [ActiveUsers], ['query']),
    get_all_neuron_owners: IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    get_foundation_assets: IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(IDL.Text, WalletOverview))],
      ['query'],
    ),
    get_holders: IDL.Func([GetHoldersArgs], [GetHoldersResponse], ['query']),
    get_locked_neurons_period: IDL.Func(
      [],
      [LockedNeuronsAmount],
      ['query'],
    ),
    get_neurons_stats: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [GovernanceStats],
      ['query'],
    ),
    get_proposals_metrics: IDL.Func([], [ProposalsMetrics], ['query']),
    get_stake_history: IDL.Func(
      [IDL.Nat64],
      [IDL.Vec(IDL.Tuple(IDL.Nat64, HistoryData))],
      ['query'],
    ),
    get_supply_data: IDL.Func([], [TokenSupplyData], ['query']),
    get_voting_participation_history: IDL.Func(
      [GetVotingParticipationHistoryArgs],
      [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))],
      ['query'],
    ),
    get_voting_power_ratio_history: IDL.Func(
      [GetVotingPowerRatioHistory],
      [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))],
      ['query'],
    ),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    foundation_accounts: IDL.Vec(IDL.Text),
    treasury_account: IDL.Text,
    sns_rewards_canister_id: IDL.Principal,
    ogy_new_ledger_canister_id: IDL.Principal,
    super_stats_canister_id: IDL.Principal,
    sns_governance_canister_id: IDL.Principal,
  });
  return [InitArgs];
};
