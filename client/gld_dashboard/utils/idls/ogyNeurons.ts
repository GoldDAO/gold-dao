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
export interface Amount { 'e8s' : bigint }
export interface Args {
  'neuron_type' : NeuronType,
  'command' : Command,
  'neuron_id' : Uint8Array | number[],
}
export type By = { 'MemoAndController' : MemoAndController } |
  { 'NeuronId' : {} };
export interface ChangeAutoStakeMaturity {
  'requested_setting_for_auto_stake_maturity' : boolean,
}
export interface ClaimOrRefresh { 'by' : [] | [By] }
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
export type DissolveState = { 'DissolveDelaySeconds' : bigint } |
  { 'WhenDissolvedTimestampSeconds' : bigint };
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
  'ogy_sns_ledger_canister_id' : Principal,
  'ogy_sns_rewards_canister_id' : Principal,
  'authorized_principals' : Array<Principal>,
  'sns_rewards_canister_id' : Principal,
  'ogy_sns_governance_canister_id' : Principal,
}
export interface ListNeuronsResponse { 'neurons' : NeuronList }
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
  'dissolve_state' : [] | [DissolveState],
  'voting_power_percentage_multiplier' : bigint,
  'vesting_period_seconds' : [] | [bigint],
  'disburse_maturity_in_progress' : Array<DisburseMaturityInProgress>,
  'followees' : Array<[bigint, Followees]>,
  'neuron_fees_e8s' : bigint,
}
export interface NeuronId { 'id' : Uint8Array | number[] }
export interface NeuronList {
  'ogy_neurons' : Array<Neuron>,
  'wtn_neurons' : Array<Neuron>,
}
export interface NeuronPermission {
  'principal' : [] | [Principal],
  'permission_type' : Int32Array | number[],
}
export interface NeuronPermissionList { 'permissions' : Int32Array | number[] }
export type NeuronType = { 'Ogy' : null } |
  { 'Wtn' : null };
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
export type Response = { 'Success' : string } |
  { 'InternalError' : string };
export type Response_1 = { 'Success' : Uint8Array | number[] } |
  { 'InternalError' : string };
export interface SetDissolveTimestamp { 'dissolve_timestamp_seconds' : bigint }
export interface Split { 'memo' : bigint, 'amount_e8s' : bigint }
export interface StakeMaturity { 'percentage_to_stake' : [] | [number] }
export interface Subaccount { 'subaccount' : Uint8Array | number[] }
export interface TransferSnsTreasuryFunds {
  'from_treasury' : number,
  'to_principal' : [] | [Principal],
  'to_subaccount' : [] | [Subaccount],
  'memo' : [] | [bigint],
  'amount_e8s' : bigint,
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
  'list_ogy_neurons' : ActorMethod<[], ListNeuronsResponse>,
  'manage_sns_neuron' : ActorMethod<[Args], Response>,
  'stake_sns_neuron' : ActorMethod<[], Response_1>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
