import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface Args {
  'sort_by' : [] | [SortBy],
  'filters' : Array<IndexType>,
  'start' : bigint,
  'length' : bigint,
}
export type Args_1 = { 'Upgrade' : UpgradeArgs } |
  { 'Init' : InitArgs };
export interface BlockWithId { 'id' : bigint, 'block' : ICRC3Value }
export interface BuildVersion {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export type ICRC3Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, ICRC3Value]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<ICRC3Value> };
export type IndexType = { 'Account' : Account } |
  { 'TokenId' : bigint } |
  { 'BlockType' : string };
export interface InitArgs {
  'test_mode' : boolean,
  'authorized_principals' : Array<Principal>,
  'version' : BuildVersion,
  'ledger_canister_id' : Principal,
  'commit_hash' : string,
}
export interface Response { 'total' : bigint, 'blocks' : Array<BlockWithId> }
export interface Response_1 { 'ledger_id' : Principal }
export interface Response_2 { 'last_block_id' : bigint }
export type SortBy = { 'Descending' : null } |
  { 'Ascending' : null };
export interface UpgradeArgs {
  'version' : BuildVersion,
  'commit_hash' : string,
}
export interface _SERVICE {
  'get_blocks' : ActorMethod<[Args], Response>,
  'ledger_id' : ActorMethod<[], Response_1>,
  'status' : ActorMethod<[], Response_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];