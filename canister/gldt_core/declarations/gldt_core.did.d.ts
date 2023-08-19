import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Conf {
  'gldt_nft_canister_ids' : Array<[Principal, NftCanisterConf]>,
  'gldt_ledger_canister_id' : Principal,
}
export interface GldtNft {
  'requested_memo' : bigint,
  'to_subaccount' : Uint8Array | number[],
  'minted' : [] | [GldtNftMinted],
  'grams' : number,
  'gldt_nft_canister_id' : Principal,
  'gldt_minting_timestamp_seconds' : bigint,
}
export interface GldtNftBurned { 'burn_block_height' : bigint }
export interface GldtNftMinted {
  'mint_block_height' : bigint,
  'last_audited_timestamp_seconds' : bigint,
  'burned' : [] | [GldtNftBurned],
}
export interface InfoRequest {
  'nft_id' : string,
  'source_canister' : Principal,
}
export interface NftCanisterConf { 'grams' : number }
export interface NftInfo { 'info' : [] | [GldtNft] }
export interface Offer { 'block_height' : bigint, 'tokens_minted' : Tokens }
export interface OfferRequest {
  'nft_id' : string,
  'requested_memo' : bigint,
  'to_subaccount' : Uint8Array | number[],
}
export type Result = { 'Ok' : Offer } |
  { 'Err' : string };
export interface SaleStatusShared { 'token_id' : string }
export interface SubAccountInfo { 'account' : SubAccoutInfo2 }
export interface SubAccoutInfo2 { 'sub_account' : Uint8Array | number[] }
export interface SubscriberNotification {
  'sale' : SaleStatusShared,
  'escrow_info' : SubAccountInfo,
}
export interface Tokens { 'e8s' : bigint }
export interface _SERVICE {
  'get_conf' : ActorMethod<[], Conf>,
  'nft_info' : ActorMethod<[InfoRequest], NftInfo>,
  'notify_sale_nft_origyn' : ActorMethod<[SubscriberNotification], undefined>,
  'request_offer' : ActorMethod<[OfferRequest], Result>,
}
