/* eslint-disable */
import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface AccountIdentifier { 'hash' : Uint8Array | number[] }
export interface AddHotKey { 'new_hot_key' : [] | [Principal] }
export interface Amount { 'e8s' : bigint }
export type Args = { 'Upgrade' : UpgradeArgs } |
  { 'Init' : InitArgs };
export interface BuildVersion {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export type By = { 'NeuronIdOrSubaccount' : {} } |
  { 'MemoAndController' : MemoAndController } |
  { 'Memo' : bigint };
export interface ChangeAutoStakeMaturity {
  'requested_setting_for_auto_stake_maturity' : boolean,
}
export interface ClaimOrRefresh { 'by' : [] | [By] }
export type Command = { 'Spawn' : Spawn } |
  { 'Split' : Split } |
  { 'Follow' : Follow } |
  { 'ClaimOrRefresh' : ClaimOrRefresh } |
  { 'Configure' : Configure } |
  { 'RegisterVote' : RegisterVote } |
  { 'Merge' : Merge } |
  { 'DisburseToNeuron' : DisburseToNeuron } |
  { 'StakeMaturity' : StakeMaturity } |
  { 'MergeMaturity' : MergeMaturity } |
  { 'Disburse' : Disburse };
export interface Configure { 'operation' : [] | [Operation] }
export interface DepositAccount {
  'legacy_account_id' : string,
  'icrc_account' : Account,
}
export interface Disburse {
  'to_account' : [] | [AccountIdentifier],
  'amount' : [] | [Amount],
}
export interface DisburseToNeuron {
  'dissolve_delay_seconds' : bigint,
  'kyc_verified' : boolean,
  'amount_e8s' : bigint,
  'new_controller' : [] | [Principal],
  'nonce' : bigint,
}
export interface Follow { 'topic' : number, 'followees' : Array<NeuronId> }
export interface IncreaseDissolveDelay {
  'additional_dissolve_delay_seconds' : number,
}
export interface InitArgs { 'test_mode' : boolean, 'commit_hash' : string }
export interface ListNeuronsResponse { 'neurons' : NeuronList }
export interface ManageNnsNeuronRequest {
  'command' : Command,
  'neuron_id' : bigint,
}
export type ManageNnsNeuronResponse = { 'Success' : string } |
  { 'InternalError' : string };
export interface ManageRewardRecipientsRequest {
  'list' : Array<RewardsRecipient>,
}
export type ManageRewardRecipientsResponse = { 'Success' : null } |
  { 'InternalError' : string };
export interface MemoAndController {
  'controller' : [] | [Principal],
  'memo' : bigint,
}
export interface Merge { 'source_neuron_id' : [] | [NeuronId] }
export interface MergeMaturity { 'percentage_to_merge' : number }
export interface NeuronId { 'id' : bigint }
export interface NeuronList {
  'active' : Array<NeuronWithMetric>,
  'disbursed' : BigUint64Array | bigint[],
  'spawning' : BigUint64Array | bigint[],
}
export interface NeuronWithMetric {
  'id' : bigint,
  'dissolve_delay' : bigint,
  'maturity' : bigint,
  'staked_amount' : bigint,
  'deposit_account' : [] | [DepositAccount],
  'dissolving' : boolean,
}
export type Operation = { 'RemoveHotKey' : RemoveHotKey } |
  { 'AddHotKey' : AddHotKey } |
  { 'ChangeAutoStakeMaturity' : ChangeAutoStakeMaturity } |
  { 'StopDissolving' : {} } |
  { 'StartDissolving' : {} } |
  { 'IncreaseDissolveDelay' : IncreaseDissolveDelay } |
  { 'JoinCommunityFund' : {} } |
  { 'LeaveCommunityFund' : {} } |
  { 'SetDissolveTimestamp' : SetDissolveTimestamp };
export interface RegisterVote { 'vote' : number, 'proposal' : [] | [NeuronId] }
export interface RemoveHotKey { 'hot_key_to_remove' : [] | [Principal] }
export interface RewardsRecipient {
  'tag' : string,
  'reward_weight' : number,
  'account' : Account,
}
export interface SetDissolveTimestamp { 'dissolve_timestamp_seconds' : bigint }
export interface Spawn {
  'percentage_to_spawn' : [] | [number],
  'new_controller' : [] | [Principal],
  'nonce' : [] | [bigint],
}
export interface Split { 'amount_e8s' : bigint }
export interface StakeMaturity { 'percentage_to_stake' : [] | [number] }
export type StakeNnsNeuronResponse = { 'Success' : bigint } |
  { 'InternalError' : string };
export interface UpgradeArgs {
  'version' : BuildVersion,
  'commit_hash' : string,
}
export interface _SERVICE {
  'list_neurons' : ActorMethod<[], ListNeuronsResponse>,
  'manage_nns_neuron' : ActorMethod<
    [ManageNnsNeuronRequest],
    ManageNnsNeuronResponse
  >,
  'manage_reward_recipients' : ActorMethod<
    [ManageRewardRecipientsRequest],
    ManageRewardRecipientsResponse
  >,
  'stake_nns_neuron' : ActorMethod<[], StakeNnsNeuronResponse>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];