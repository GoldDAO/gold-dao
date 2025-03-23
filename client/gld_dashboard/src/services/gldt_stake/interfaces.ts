/* eslint-disable */
import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : [] | [Principal],
  'subaccount' : [] | [Subaccount],
}
export type Action = {
    'ManageNervousSystemParameters' : NervousSystemParameters
  } |
  { 'AddGenericNervousSystemFunction' : NervousSystemFunction } |
  { 'RemoveGenericNervousSystemFunction' : bigint } |
  { 'UpgradeSnsToNextVersion' : {} } |
  { 'RegisterDappCanisters' : RegisterDappCanisters } |
  { 'TransferSnsTreasuryFunds' : TransferSnsTreasuryFunds } |
  { 'UpgradeSnsControlledCanister' : UpgradeSnsControlledCanister } |
  { 'DeregisterDappCanisters' : DeregisterDappCanisters } |
  { 'Unspecified' : {} } |
  { 'ManageSnsMetadata' : ManageSnsMetadata } |
  {
    'ExecuteGenericNervousSystemFunction' : ExecuteGenericNervousSystemFunction
  } |
  { 'Motion' : Motion };
export interface AddNeuronPermissions {
  'permissions_to_add' : [] | [NeuronPermissionList],
  'principal_id' : [] | [Principal],
}
export type AddStakePositionErrors = { 'MaxActiveStakePositions' : string } |
  { 'TransferError' : string } |
  { 'AlreadyProcessing' : string } |
  { 'InvalidStakeAmount' : string } |
  { 'InvalidPrincipal' : string } |
  { 'CallError' : string };
export interface Amount { 'e8s' : bigint }
export interface Args { 'id' : bigint, 'token' : string }
export interface Args_1 { 'amount' : bigint }
export interface Args_2 { 'amount' : bigint }
export interface Args_3 {
  'command' : Command,
  'neuron_id' : Uint8Array | number[],
}
export type Args_4 = { 'Upgrade' : UpgradeArgs } |
  { 'Init' : InitArgs };
export interface BuildVersion {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export type By = { 'MemoAndController' : MemoAndController } |
  { 'NeuronId' : {} };
export interface ChangeAutoStakeMaturity {
  'requested_setting_for_auto_stake_maturity' : boolean,
}
export interface ClaimOrRefresh { 'by' : [] | [By] }
export type ClaimRewardErrors = { 'TransferError' : string } |
  { 'InvalidRewardToken' : string } |
  { 'AlreadyProcessing' : string } |
  { 'InvalidPrincipal' : string } |
  { 'NotFound' : string } |
  { 'NotAuthorized' : string } |
  { 'CallError' : string } |
  { 'TokenImbalance' : string };
export type Command = { 'Split' : Split } |
  { 'Follow' : Follow } |
  { 'DisburseMaturity' : DisburseMaturity } |
  { 'ClaimOrRefresh' : ClaimOrRefresh } |
  { 'Configure' : Configure } |
  { 'RegisterVote' : RegisterVote } |
  { 'MakeProposal' : Proposal } |
  { 'StakeMaturity' : StakeMaturity } |
  { 'RemoveNeuronPermissions' : RemoveNeuronPermissions } |
  { 'AddNeuronPermissions' : AddNeuronPermissions } |
  { 'MergeMaturity' : MergeMaturity } |
  { 'Disburse' : Disburse };
export interface Configure { 'operation' : [] | [Operation] }
export type CreateNeuronError = { 'TransferError' : string } |
  { 'InternalError' : string };
export interface DefaultFollowees { 'followees' : Array<[bigint, Followees]> }
export interface DeregisterDappCanisters {
  'canister_ids' : Array<Principal>,
  'new_controllers' : Array<Principal>,
}
export interface Disburse {
  'to_account' : [] | [Account],
  'amount' : [] | [Amount],
}
export interface DisburseMaturity {
  'to_account' : [] | [Account],
  'percentage_to_disburse' : number,
}
export interface DisburseMaturityInProgress {
  'timestamp_of_disbursement_seconds' : bigint,
  'amount_e8s' : bigint,
  'account_to_disburse_to' : [] | [Account],
}
export type DissolveState = { 'Dissolved' : null } |
  { 'Dissolving' : null } |
  { 'NotDissolving' : null };
export type DissolveState_1 = { 'DissolveDelaySeconds' : bigint } |
  { 'WhenDissolvedTimestampSeconds' : bigint };
export interface Duration { 'secs' : bigint, 'nanos' : number }
export interface ExecuteGenericNervousSystemFunction {
  'function_id' : bigint,
  'payload' : Uint8Array | number[],
}
export interface Follow {
  'function_id' : bigint,
  'followees' : Array<NeuronId>,
}
export interface Followees { 'followees' : Array<NeuronId> }
export type FunctionType = { 'NativeNervousSystemFunction' : {} } |
  { 'GenericNervousSystemFunction' : GenericNervousSystemFunction };
export interface GenericNervousSystemFunction {
  'validator_canister_id' : [] | [Principal],
  'target_canister_id' : [] | [Principal],
  'validator_method_name' : [] | [string],
  'target_method_name' : [] | [string],
}
export interface IncreaseDissolveDelay {
  'additional_dissolve_delay_seconds' : number,
}
export interface InitArgs {
  'test_mode' : boolean,
  'reward_types' : Array<[string, [Principal, bigint]]>,
  'authorized_principals' : Array<Principal>,
  'version' : BuildVersion,
  'gld_sns_governance_canister_id' : Principal,
  'gldt_ledger_id' : Principal,
  'commit_hash' : string,
  'gldgov_ledger_id' : Principal,
  'gld_sns_rewards_canister_id' : Principal,
}
export interface ManageSnsMetadata {
  'url' : [] | [string],
  'logo' : [] | [string],
  'name' : [] | [string],
  'description' : [] | [string],
}
export interface MemoAndController {
  'controller' : [] | [Principal],
  'memo' : bigint,
}
export interface MergeMaturity { 'percentage_to_merge' : number }
export interface Motion { 'motion_text' : string }
export interface NervousSystemFunction {
  'id' : bigint,
  'name' : string,
  'description' : [] | [string],
  'function_type' : [] | [FunctionType],
}
export interface NervousSystemParameters {
  'default_followees' : [] | [DefaultFollowees],
  'max_dissolve_delay_seconds' : [] | [bigint],
  'max_dissolve_delay_bonus_percentage' : [] | [bigint],
  'max_followees_per_function' : [] | [bigint],
  'neuron_claimer_permissions' : [] | [NeuronPermissionList],
  'neuron_minimum_stake_e8s' : [] | [bigint],
  'max_neuron_age_for_age_bonus' : [] | [bigint],
  'initial_voting_period_seconds' : [] | [bigint],
  'neuron_minimum_dissolve_delay_to_vote_seconds' : [] | [bigint],
  'reject_cost_e8s' : [] | [bigint],
  'max_proposals_to_keep_per_action' : [] | [number],
  'wait_for_quiet_deadline_increase_seconds' : [] | [bigint],
  'max_number_of_neurons' : [] | [bigint],
  'transaction_fee_e8s' : [] | [bigint],
  'max_number_of_proposals_with_ballots' : [] | [bigint],
  'max_age_bonus_percentage' : [] | [bigint],
  'neuron_grantable_permissions' : [] | [NeuronPermissionList],
  'voting_rewards_parameters' : [] | [VotingRewardsParameters],
  'maturity_modulation_disabled' : [] | [boolean],
  'max_number_of_principals_per_neuron' : [] | [bigint],
}
export interface Neuron {
  'id' : [] | [NeuronId],
  'staked_maturity_e8s_equivalent' : [] | [bigint],
  'permissions' : Array<NeuronPermission>,
  'maturity_e8s_equivalent' : bigint,
  'cached_neuron_stake_e8s' : bigint,
  'created_timestamp_seconds' : bigint,
  'source_nns_neuron_id' : [] | [bigint],
  'auto_stake_maturity' : [] | [boolean],
  'aging_since_timestamp_seconds' : bigint,
  'dissolve_state' : [] | [DissolveState_1],
  'voting_power_percentage_multiplier' : bigint,
  'vesting_period_seconds' : [] | [bigint],
  'disburse_maturity_in_progress' : Array<DisburseMaturityInProgress>,
  'followees' : Array<[bigint, Followees]>,
  'neuron_fees_e8s' : bigint,
}
export interface NeuronId { 'id' : Uint8Array | number[] }
export interface NeuronPermission {
  'principal' : [] | [Principal],
  'permission_type' : Int32Array | number[],
}
export interface NeuronPermissionList { 'permissions' : Int32Array | number[] }
export type Operation = {
    'ChangeAutoStakeMaturity' : ChangeAutoStakeMaturity
  } |
  { 'StopDissolving' : {} } |
  { 'StartDissolving' : {} } |
  { 'IncreaseDissolveDelay' : IncreaseDissolveDelay } |
  { 'SetDissolveTimestamp' : SetDissolveTimestamp };
export interface Proposal {
  'url' : string,
  'title' : string,
  'action' : [] | [Action],
  'summary' : string,
}
export interface ProposalId { 'id' : bigint }
export interface RegisterDappCanisters { 'canister_ids' : Array<Principal> }
export interface RegisterVote {
  'vote' : number,
  'proposal' : [] | [ProposalId],
}
export interface RemoveNeuronPermissions {
  'permissions_to_remove' : [] | [NeuronPermissionList],
  'principal_id' : [] | [Principal],
}
export type RemoveRewardErrors = { 'InsufficientBalance' : string } |
  { 'RewardTokenTypeDoesNotExist' : string };
export type Response = { 'Success' : string } |
  { 'InternalError' : string };
export type Result = { 'Ok' : StakePositionResponse } |
  { 'Err' : ClaimRewardErrors };
export type Result_1 = { 'Ok' : Uint8Array | number[] } |
  { 'Err' : CreateNeuronError };
export type Result_2 = { 'Ok' : StakePositionResponse } |
  { 'Err' : AddStakePositionErrors };
export type Result_3 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : StakePositionResponse } |
  { 'Err' : StartDissolvingErrors };
export type Result_6 = { 'Ok' : StakePositionResponse } |
  { 'Err' : UnstakeRequestErrors };
export type Result_7 = { 'Ok' : StakePositionResponse } |
  { 'Err' : UnstakeEarlyRequestErrors };
export interface SetDissolveTimestamp { 'dissolve_timestamp_seconds' : bigint }
export interface Split { 'memo' : bigint, 'amount_e8s' : bigint }
export interface StakeMaturity { 'percentage_to_stake' : [] | [number] }
export type StakePositionError = { 'StartDissolvingError' : string } |
  { 'AddStakePositionError' : AddStakePositionErrors } |
  { 'UnStakeError' : UnstakeErrors } |
  { 'AddRewardError' : string } |
  { 'RemoveRewardError' : RemoveRewardErrors };
export interface StakePositionResponse {
  'id' : bigint,
  'staked' : bigint,
  'dissolve_delay' : Duration,
  'early_unstake_fee' : bigint,
  'claimable_rewards' : Array<[string, bigint]>,
  'dissolved_date' : [] | [bigint],
  'created_at' : bigint,
  'age_bonus_multiplier' : number,
  'owned_by' : Principal,
  'dissolve_state' : DissolveState,
  'weighted_stake' : bigint,
}
export type StartDissolvingErrors = { 'InvalidPrincipal' : string } |
  { 'NotFound' : string } |
  { 'NotAuthorized' : string } |
  { 'StakePositionError' : StakePositionError };
export interface Subaccount { 'subaccount' : Uint8Array | number[] }
export interface TransferSnsTreasuryFunds {
  'from_treasury' : number,
  'to_principal' : [] | [Principal],
  'to_subaccount' : [] | [Subaccount],
  'memo' : [] | [bigint],
  'amount_e8s' : bigint,
}
export type UnstakeEarlyRequestErrors = { 'TransferError' : string } |
  { 'UnstakeErrors' : UnstakeErrors } |
  { 'AlreadyProcessing' : string } |
  { 'AlreadyUnstakedEarly' : string } |
  { 'InvalidPrincipal' : string } |
  { 'NotFound' : string } |
  { 'NotAuthorized' : string } |
  { 'CallError' : string };
export type UnstakeErrors = { 'NoDissolveDateSet' : string } |
  { 'AlreadyProcessing' : string } |
  { 'AlreadyUnstaked' : string } |
  { 'DissolveDateNotSatisfied' : string } |
  { 'InvalidDissolveState' : string } |
  { 'CantUnstakeWithRewardsBalance' : string };
export type UnstakeRequestErrors = { 'TransferError' : string } |
  { 'UnstakeErrors' : UnstakeErrors } |
  { 'InvalidPrincipal' : string } |
  { 'NotFound' : string } |
  { 'AlreadyUnstaked' : string } |
  { 'NotAuthorized' : string } |
  { 'CallError' : string } |
  { 'InvalidState' : string };
export interface UpgradeArgs {
  'version' : BuildVersion,
  'commit_hash' : string,
}
export interface UpgradeSnsControlledCanister {
  'new_canister_wasm' : Uint8Array | number[],
  'mode' : [] | [number],
  'canister_id' : [] | [Principal],
  'canister_upgrade_arg' : [] | [Uint8Array | number[]],
}
export interface VotingRewardsParameters {
  'final_reward_rate_basis_points' : [] | [bigint],
  'initial_reward_rate_basis_points' : [] | [bigint],
  'reward_rate_transition_duration_seconds' : [] | [bigint],
  'round_duration_seconds' : [] | [bigint],
}
export interface _SERVICE {
  'claim_reward' : ActorMethod<[Args], Result>,
  'commit' : ActorMethod<[], undefined>,
  'create_neuron' : ActorMethod<[Args_1], Result_1>,
  'create_stake_position' : ActorMethod<[Args_2], Result_2>,
  'get_active_user_positions' : ActorMethod<
    [[] | [Principal]],
    Array<StakePositionResponse>
  >,
  'get_neurons' : ActorMethod<[null], Array<Neuron>>,
  'get_position_by_id' : ActorMethod<[bigint], [] | [StakePositionResponse]>,
  'get_total_allocated_rewards' : ActorMethod<[null], Array<[string, bigint]>>,
  'get_total_staked' : ActorMethod<[null], bigint>,
  'manage_sns_neuron' : ActorMethod<[Args_3], Response>,
  'manual_sync_neurons' : ActorMethod<[null], Result_3>,
  'process_oldest_reward_round' : ActorMethod<[null], Result_4>,
  'start_dissolving' : ActorMethod<[bigint], Result_5>,
  'unstake' : ActorMethod<[bigint], Result_6>,
  'unstake_early' : ActorMethod<[bigint], Result_7>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];