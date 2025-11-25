export const idlFactory = ({ IDL }) => {
  const ICRC3Value = IDL.Rec();
  const SortBy = IDL.Variant({
    'Descending' : IDL.Null,
    'Ascending' : IDL.Null,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const IndexType = IDL.Variant({
    'Account' : Account,
    'TokenId' : IDL.Nat,
    'BlockType' : IDL.Text,
  });
  const Args = IDL.Record({
    'sort_by' : IDL.Opt(SortBy),
    'filters' : IDL.Vec(IndexType),
    'start' : IDL.Nat64,
    'length' : IDL.Nat64,
  });
  ICRC3Value.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, ICRC3Value)),
      'Nat' : IDL.Nat,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Text' : IDL.Text,
      'Array' : IDL.Vec(ICRC3Value),
    })
  );
  const BlockWithId = IDL.Record({ 'id' : IDL.Nat, 'block' : ICRC3Value });
  const Response = IDL.Record({
    'total' : IDL.Nat64,
    'blocks' : IDL.Vec(BlockWithId),
  });
  const Response_1 = IDL.Record({ 'ledger_id' : IDL.Principal });
  const Response_2 = IDL.Record({ 'last_block_id' : IDL.Nat64 });
  return IDL.Service({
    'get_blocks' : IDL.Func([Args], [Response], []),
    'ledger_id' : IDL.Func([], [Response_1], ['query']),
    'status' : IDL.Func([], [Response_2], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };