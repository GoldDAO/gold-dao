import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Args { 'token' : string, 'neuron_id' : NeuronId }
export interface Args_1 { 'transfer_amounts' : Array<[string, bigint]> }
export interface Args_2 { 'token_list' : Array<[string, TokenInfo]> }
export type Args_3 = { 'Upgrade' : UpgradeArgs } |
  { 'Init' : InitArgs };
export interface BuildVersion {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface InitArgs {
  'sns_gov_canister_id' : Principal,
  'test_mode' : boolean,
  'version' : BuildVersion,
  'ogy_ledger_canister_id' : Principal,
  'icp_ledger_canister_id' : Principal,
  'sns_ledger_canister_id' : Principal,
  'commit_hash' : string,
}
export interface NeuronId { 'id' : Uint8Array | number[] }
export type Response = { 'Ok' : boolean } |
  { 'NeuronHotKeyAbsent' : null } |
  { 'TokenSymbolInvalid' : string } |
  { 'NeuronNotClaimed' : null } |
  { 'NeuronOwnerInvalid' : [] | [Principal] } |
  { 'NeuronHotKeyInvalid' : null } |
  { 'TransferFailed' : string } |
  { 'NeuronDoesNotExist' : null } |
  { 'InternalError' : string };
export type Response_1 = { 'Success' : null } |
  { 'InternalError' : string };
export type Response_2 = { 'Success' : null } |
  { 'InternalError' : string };
export interface TokenInfo {
  'fee' : bigint,
  'decimals' : bigint,
  'ledger_id' : Principal,
}
export interface UpgradeArgs {
  'version' : BuildVersion,
  'commit_hash' : string,
}
export interface _SERVICE {
  'claim_reward' : ActorMethod<[Args], Response>,
  'set_daily_gldgov_burn_rate' : ActorMethod<[bigint], Response_1>,
  'set_reserve_transfer_amounts' : ActorMethod<[Args_1], Response_1>,
  'set_reward_token_types' : ActorMethod<[Args_2], Response_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];