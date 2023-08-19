export const idlFactory = ({ IDL }) => {
  const NftCanisterConf = IDL.Record({ 'grams' : IDL.Nat16 });
  const Conf = IDL.Record({
    'gldt_nft_canister_ids' : IDL.Vec(
      IDL.Tuple(IDL.Principal, NftCanisterConf)
    ),
    'gldt_ledger_canister_id' : IDL.Principal,
  });
  const InfoRequest = IDL.Record({
    'nft_id' : IDL.Text,
    'source_canister' : IDL.Principal,
  });
  const GldtNftBurned = IDL.Record({ 'burn_block_height' : IDL.Nat64 });
  const GldtNftMinted = IDL.Record({
    'mint_block_height' : IDL.Nat64,
    'last_audited_timestamp_seconds' : IDL.Nat64,
    'burned' : IDL.Opt(GldtNftBurned),
  });
  const GldtNft = IDL.Record({
    'requested_memo' : IDL.Nat64,
    'to_subaccount' : IDL.Vec(IDL.Nat8),
    'minted' : IDL.Opt(GldtNftMinted),
    'grams' : IDL.Nat16,
    'gldt_nft_canister_id' : IDL.Principal,
    'gldt_minting_timestamp_seconds' : IDL.Nat64,
  });
  const NftInfo = IDL.Record({ 'info' : IDL.Opt(GldtNft) });
  const SaleStatusShared = IDL.Record({ 'token_id' : IDL.Text });
  const SubAccoutInfo2 = IDL.Record({ 'sub_account' : IDL.Vec(IDL.Nat8) });
  const SubAccountInfo = IDL.Record({ 'account' : SubAccoutInfo2 });
  const SubscriberNotification = IDL.Record({
    'sale' : SaleStatusShared,
    'escrow_info' : SubAccountInfo,
  });
  const OfferRequest = IDL.Record({
    'nft_id' : IDL.Text,
    'requested_memo' : IDL.Nat64,
    'to_subaccount' : IDL.Vec(IDL.Nat8),
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const Offer = IDL.Record({
    'block_height' : IDL.Nat64,
    'tokens_minted' : Tokens,
  });
  const Result = IDL.Variant({ 'Ok' : Offer, 'Err' : IDL.Text });
  return IDL.Service({
    'get_conf' : IDL.Func([], [Conf], ['query']),
    'nft_info' : IDL.Func([InfoRequest], [NftInfo], ['query']),
    'notify_sale_nft_origyn' : IDL.Func([SubscriberNotification], [], []),
    'request_offer' : IDL.Func([OfferRequest], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
