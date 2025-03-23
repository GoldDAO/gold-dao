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
    test_mode: IDL.Bool,
    ogy_sns_ledger_canister_id: IDL.Principal,
    ogy_sns_rewards_canister_id: IDL.Principal,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    sns_rewards_canister_id: IDL.Principal,
    commit_hash: IDL.Text,
    ogy_sns_governance_canister_id: IDL.Principal,
  });
  const Args_2 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  const NeuronId = IDL.Record({ id: IDL.Vec(IDL.Nat8) });
  const NeuronPermission = IDL.Record({
    principal: IDL.Opt(IDL.Principal),
    permission_type: IDL.Vec(IDL.Int32),
  });
  const DissolveState = IDL.Variant({
    DissolveDelaySeconds: IDL.Nat64,
    WhenDissolvedTimestampSeconds: IDL.Nat64,
  });
  const Subaccount = IDL.Record({ subaccount: IDL.Vec(IDL.Nat8) });
  const Account = IDL.Record({
    owner: IDL.Opt(IDL.Principal),
    subaccount: IDL.Opt(Subaccount),
  });
  const DisburseMaturityInProgress = IDL.Record({
    timestamp_of_disbursement_seconds: IDL.Nat64,
    amount_e8s: IDL.Nat64,
    account_to_disburse_to: IDL.Opt(Account),
  });
  const Followees = IDL.Record({ followees: IDL.Vec(NeuronId) });
  const Neuron = IDL.Record({
    id: IDL.Opt(NeuronId),
    staked_maturity_e8s_equivalent: IDL.Opt(IDL.Nat64),
    permissions: IDL.Vec(NeuronPermission),
    maturity_e8s_equivalent: IDL.Nat64,
    cached_neuron_stake_e8s: IDL.Nat64,
    created_timestamp_seconds: IDL.Nat64,
    source_nns_neuron_id: IDL.Opt(IDL.Nat64),
    auto_stake_maturity: IDL.Opt(IDL.Bool),
    aging_since_timestamp_seconds: IDL.Nat64,
    dissolve_state: IDL.Opt(DissolveState),
    voting_power_percentage_multiplier: IDL.Nat64,
    vesting_period_seconds: IDL.Opt(IDL.Nat64),
    disburse_maturity_in_progress: IDL.Vec(DisburseMaturityInProgress),
    followees: IDL.Vec(IDL.Tuple(IDL.Nat64, Followees)),
    neuron_fees_e8s: IDL.Nat64,
  });
  const NeuronList = IDL.Record({
    ogy_neurons: IDL.Vec(Neuron),
    wtn_neurons: IDL.Vec(Neuron),
  });
  const ListNeuronsResponse = IDL.Record({ neurons: NeuronList });
  const NeuronType = IDL.Variant({ Ogy: IDL.Null, Wtn: IDL.Null });
  const Split = IDL.Record({ memo: IDL.Nat64, amount_e8s: IDL.Nat64 });
  const Follow = IDL.Record({
    function_id: IDL.Nat64,
    followees: IDL.Vec(NeuronId),
  });
  const DisburseMaturity = IDL.Record({
    to_account: IDL.Opt(Account),
    percentage_to_disburse: IDL.Nat32,
  });
  const MemoAndController = IDL.Record({
    controller: IDL.Opt(IDL.Principal),
    memo: IDL.Nat64,
  });
  const By = IDL.Variant({
    MemoAndController: MemoAndController,
    NeuronId: IDL.Record({}),
  });
  const ClaimOrRefresh = IDL.Record({ by: IDL.Opt(By) });
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
    ChangeAutoStakeMaturity: ChangeAutoStakeMaturity,
    StopDissolving: IDL.Record({}),
    StartDissolving: IDL.Record({}),
    IncreaseDissolveDelay: IncreaseDissolveDelay,
    SetDissolveTimestamp: SetDissolveTimestamp,
  });
  const Configure = IDL.Record({ operation: IDL.Opt(Operation) });
  const ProposalId = IDL.Record({ id: IDL.Nat64 });
  const RegisterVote = IDL.Record({
    vote: IDL.Int32,
    proposal: IDL.Opt(ProposalId),
  });
  const DefaultFollowees = IDL.Record({
    followees: IDL.Vec(IDL.Tuple(IDL.Nat64, Followees)),
  });
  const NeuronPermissionList = IDL.Record({
    permissions: IDL.Vec(IDL.Int32),
  });
  const VotingRewardsParameters = IDL.Record({
    final_reward_rate_basis_points: IDL.Opt(IDL.Nat64),
    initial_reward_rate_basis_points: IDL.Opt(IDL.Nat64),
    reward_rate_transition_duration_seconds: IDL.Opt(IDL.Nat64),
    round_duration_seconds: IDL.Opt(IDL.Nat64),
  });
  const NervousSystemParameters = IDL.Record({
    default_followees: IDL.Opt(DefaultFollowees),
    max_dissolve_delay_seconds: IDL.Opt(IDL.Nat64),
    max_dissolve_delay_bonus_percentage: IDL.Opt(IDL.Nat64),
    max_followees_per_function: IDL.Opt(IDL.Nat64),
    neuron_claimer_permissions: IDL.Opt(NeuronPermissionList),
    neuron_minimum_stake_e8s: IDL.Opt(IDL.Nat64),
    max_neuron_age_for_age_bonus: IDL.Opt(IDL.Nat64),
    initial_voting_period_seconds: IDL.Opt(IDL.Nat64),
    neuron_minimum_dissolve_delay_to_vote_seconds: IDL.Opt(IDL.Nat64),
    reject_cost_e8s: IDL.Opt(IDL.Nat64),
    max_proposals_to_keep_per_action: IDL.Opt(IDL.Nat32),
    wait_for_quiet_deadline_increase_seconds: IDL.Opt(IDL.Nat64),
    max_number_of_neurons: IDL.Opt(IDL.Nat64),
    transaction_fee_e8s: IDL.Opt(IDL.Nat64),
    max_number_of_proposals_with_ballots: IDL.Opt(IDL.Nat64),
    max_age_bonus_percentage: IDL.Opt(IDL.Nat64),
    neuron_grantable_permissions: IDL.Opt(NeuronPermissionList),
    voting_rewards_parameters: IDL.Opt(VotingRewardsParameters),
    maturity_modulation_disabled: IDL.Opt(IDL.Bool),
    max_number_of_principals_per_neuron: IDL.Opt(IDL.Nat64),
  });
  const GenericNervousSystemFunction = IDL.Record({
    validator_canister_id: IDL.Opt(IDL.Principal),
    target_canister_id: IDL.Opt(IDL.Principal),
    validator_method_name: IDL.Opt(IDL.Text),
    target_method_name: IDL.Opt(IDL.Text),
  });
  const FunctionType = IDL.Variant({
    NativeNervousSystemFunction: IDL.Record({}),
    GenericNervousSystemFunction: GenericNervousSystemFunction,
  });
  const NervousSystemFunction = IDL.Record({
    id: IDL.Nat64,
    name: IDL.Text,
    description: IDL.Opt(IDL.Text),
    function_type: IDL.Opt(FunctionType),
  });
  const RegisterDappCanisters = IDL.Record({
    canister_ids: IDL.Vec(IDL.Principal),
  });
  const TransferSnsTreasuryFunds = IDL.Record({
    from_treasury: IDL.Int32,
    to_principal: IDL.Opt(IDL.Principal),
    to_subaccount: IDL.Opt(Subaccount),
    memo: IDL.Opt(IDL.Nat64),
    amount_e8s: IDL.Nat64,
  });
  const UpgradeSnsControlledCanister = IDL.Record({
    new_canister_wasm: IDL.Vec(IDL.Nat8),
    mode: IDL.Opt(IDL.Int32),
    canister_id: IDL.Opt(IDL.Principal),
    canister_upgrade_arg: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const DeregisterDappCanisters = IDL.Record({
    canister_ids: IDL.Vec(IDL.Principal),
    new_controllers: IDL.Vec(IDL.Principal),
  });
  const ManageSnsMetadata = IDL.Record({
    url: IDL.Opt(IDL.Text),
    logo: IDL.Opt(IDL.Text),
    name: IDL.Opt(IDL.Text),
    description: IDL.Opt(IDL.Text),
  });
  const ExecuteGenericNervousSystemFunction = IDL.Record({
    function_id: IDL.Nat64,
    payload: IDL.Vec(IDL.Nat8),
  });
  const Motion = IDL.Record({ motion_text: IDL.Text });
  const Action = IDL.Variant({
    ManageNervousSystemParameters: NervousSystemParameters,
    AddGenericNervousSystemFunction: NervousSystemFunction,
    RemoveGenericNervousSystemFunction: IDL.Nat64,
    UpgradeSnsToNextVersion: IDL.Record({}),
    RegisterDappCanisters: RegisterDappCanisters,
    TransferSnsTreasuryFunds: TransferSnsTreasuryFunds,
    UpgradeSnsControlledCanister: UpgradeSnsControlledCanister,
    DeregisterDappCanisters: DeregisterDappCanisters,
    Unspecified: IDL.Record({}),
    ManageSnsMetadata: ManageSnsMetadata,
    ExecuteGenericNervousSystemFunction: ExecuteGenericNervousSystemFunction,
    Motion: Motion,
  });
  const Proposal = IDL.Record({
    url: IDL.Text,
    title: IDL.Text,
    action: IDL.Opt(Action),
    summary: IDL.Text,
  });
  const StakeMaturity = IDL.Record({
    percentage_to_stake: IDL.Opt(IDL.Nat32),
  });
  const RemoveNeuronPermissions = IDL.Record({
    permissions_to_remove: IDL.Opt(NeuronPermissionList),
    principal_id: IDL.Opt(IDL.Principal),
  });
  const AddNeuronPermissions = IDL.Record({
    permissions_to_add: IDL.Opt(NeuronPermissionList),
    principal_id: IDL.Opt(IDL.Principal),
  });
  const MergeMaturity = IDL.Record({ percentage_to_merge: IDL.Nat32 });
  const Amount = IDL.Record({ e8s: IDL.Nat64 });
  const Disburse = IDL.Record({
    to_account: IDL.Opt(Account),
    amount: IDL.Opt(Amount),
  });
  const Command = IDL.Variant({
    Split: Split,
    Follow: Follow,
    DisburseMaturity: DisburseMaturity,
    ClaimOrRefresh: ClaimOrRefresh,
    Configure: Configure,
    RegisterVote: RegisterVote,
    MakeProposal: Proposal,
    StakeMaturity: StakeMaturity,
    RemoveNeuronPermissions: RemoveNeuronPermissions,
    AddNeuronPermissions: AddNeuronPermissions,
    MergeMaturity: MergeMaturity,
    Disburse: Disburse,
  });
  const Args = IDL.Record({
    neuron_type: NeuronType,
    command: Command,
    neuron_id: IDL.Vec(IDL.Nat8),
  });
  const Response = IDL.Variant({
    Success: IDL.Text,
    InternalError: IDL.Text,
  });
  const Args_1 = IDL.Record({ amount: IDL.Nat64 });
  const Response_1 = IDL.Variant({
    Success: IDL.Vec(IDL.Nat8),
    InternalError: IDL.Text,
  });
  return IDL.Service({
    list_ogy_neurons: IDL.Func([], [ListNeuronsResponse], ["query"]),
    manage_sns_neuron: IDL.Func([Args], [Response], []),
    stake_ogy_neuron: IDL.Func([Args_1], [Response_1], []),
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
    test_mode: IDL.Bool,
    ogy_sns_ledger_canister_id: IDL.Principal,
    ogy_sns_rewards_canister_id: IDL.Principal,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    sns_rewards_canister_id: IDL.Principal,
    commit_hash: IDL.Text,
    ogy_sns_governance_canister_id: IDL.Principal,
  });
  const Args_2 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  return [Args_2];
};
