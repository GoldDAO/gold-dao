export const idlFactory = ({ IDL }) => {
  const SwapStatusReverse = IDL.Rec();
  const BuildVersion = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    'version' : BuildVersion,
    'commit_hash' : IDL.Text,
  });
  const InitArgs = IDL.Record({
    'test_mode' : IDL.Bool,
    'authorized_principals' : IDL.Vec(IDL.Principal),
    'commit_hash' : IDL.Text,
  });
  const Args_1 = IDL.Variant({ 'Upgrade' : UpgradeArgs, 'Init' : InitArgs });
  const BidFailError = IDL.Variant({
    'UnexpectedError' : IDL.Text,
    'CallError' : IDL.Text,
    'TransferFailed' : IDL.Text,
  });
  const ImpossibleErrorReason = IDL.Variant({
    'AmountNotFound' : IDL.Null,
    'NFTResponseInvalid' : IDL.Null,
    'PrincipalNotFound' : IDL.Null,
  });
  const NotificationError = IDL.Variant({
    'InvalidSaleSubaccount' : IDL.Null,
    'InvalidTokenSpec' : IDL.Null,
    'TimeoutInvalid' : IDL.Text,
    'InvalidEscrowSubaccount' : IDL.Text,
    'TooManyPrincipalsInAllowList' : IDL.Null,
    'OrigynStringIdDoesNotMatch' : IDL.Text,
    'SellerIsNotPrincipalOrAccount' : IDL.Text,
    'SellerAndReceiverDoesNotMatch' : IDL.Text,
    'InvalidCustomAskFeature' : IDL.Null,
    'InvalidTokenAmount' : IDL.Null,
    'InvalidPricingConfig' : IDL.Null,
    'CollectionDoesNotMatch' : IDL.Text,
    'AllowListDoesNotContainCorrectPrincipal' : IDL.Null,
  });
  const TransferError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'BadBurn' : IDL.Record({ 'min_burn_amount' : IDL.Nat }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const TransferFromError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'InsufficientAllowance' : IDL.Record({ 'allowance' : IDL.Nat }),
    'BadBurn' : IDL.Record({ 'min_burn_amount' : IDL.Nat }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const TransferFailReason = IDL.Variant({
    'TransferError' : TransferError,
    'TransferFromError' : TransferFromError,
    'CallError' : IDL.Text,
  });
  const MintError = IDL.Variant({
    'UnexpectedError' : ImpossibleErrorReason,
    'TransferFailed' : TransferFailReason,
  });
  const SwapErrorForward = IDL.Variant({
    'BidFailed' : BidFailError,
    'UnexpectedError' : ImpossibleErrorReason,
    'NotificationFailed' : NotificationError,
    'MintFailed' : MintError,
    'Expired' : IDL.Null,
  });
  const SwapStatusForward = IDL.Variant({
    'Failed' : SwapErrorForward,
    'Init' : IDL.Null,
    'MintRequest' : IDL.Null,
    'Complete' : IDL.Null,
    'BidFail' : BidFailError,
    'BidRequest' : IDL.Null,
    'NotificationFailed' : NotificationError,
    'BurnFeesRequest' : IDL.Null,
    'BurnFeesFailed' : MintError,
    'MintFailed' : MintError,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const GldtNumTokens = IDL.Record({
    'value_with_fee' : IDL.Nat,
    'value' : IDL.Nat,
  });
  const SwapDetailForward = IDL.Record({
    'nft_id' : IDL.Nat,
    'status' : SwapStatusForward,
    'escrow_sub_account' : IDL.Vec(IDL.Nat8),
    'nft_id_string' : IDL.Text,
    'created_at' : IDL.Nat64,
    'gldt_receiver' : Account,
    'tokens_to_mint' : GldtNumTokens,
    'nft_canister' : IDL.Principal,
    'index' : IDL.Nat,
    'sale_id' : IDL.Text,
  });
  const FeeTransferError = IDL.Variant({
    'TransferError' : TransferError,
    'CallError' : IDL.Text,
  });
  const ApproveError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'AllowanceChanged' : IDL.Record({ 'current_allowance' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'Expired' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const EscrowError = IDL.Variant({
    'ApproveError' : ApproveError,
    'UnexpectedError' : ImpossibleErrorReason,
    'TransferFailed' : TransferFailReason,
    'RequestFailed' : IDL.Text,
  });
  const LockError = IDL.Variant({
    'NftAlreadyLocked' : IDL.Vec(IDL.Nat),
    'UnexpectedError' : IDL.Record({}),
    'NftNotLocked' : IDL.Null,
  });
  const NftValidationError = IDL.Variant({
    'WeightParseError' : IDL.Null,
    'CanisterInvalid' : IDL.Null,
    'CantGetOrigynID' : IDL.Text,
    'CantVerifySwapCanisterOwnsNft' : IDL.Null,
    'InvalidGldtTokensFromWeight' : IDL.Null,
    'InvalidNftWeight' : IDL.Null,
    'NotOwnedBySwapCanister' : IDL.Null,
  });
  const BurnError = IDL.Variant({ 'CallError' : IDL.Text });
  const NftTransferError = IDL.Variant({
    'FailedToGetOgyFeeAllowance' : IDL.Text,
    'ApprovalError' : ApproveError,
    'ApprovalCallError' : IDL.Text,
    'InvalidFee' : IDL.Text,
    'UnexpectedError' : ImpossibleErrorReason,
    'CallError' : IDL.Text,
    'TransferFailed' : IDL.Text,
  });
  const SwapErrorReverse = IDL.Variant({
    'FeeTransferFailed' : FeeTransferError,
    'EscrowFailed' : EscrowError,
    'LockFailed' : LockError,
    'Refunded' : SwapStatusReverse,
    'NftValidationFailed' : IDL.Vec(NftValidationError),
    'BurnFailed' : BurnError,
    'NftTransferFailed' : NftTransferError,
  });
  const RefundError = IDL.Variant({
    'CallError' : IDL.Text,
    'TransferFailed' : TransferError,
  });
  SwapStatusReverse.fill(
    IDL.Variant({
      'FeeTransferFailed' : FeeTransferError,
      'Failed' : SwapErrorReverse,
      'EscrowFailed' : EscrowError,
      'Init' : IDL.Null,
      'Complete' : IDL.Null,
      'BurnFailed' : BurnError,
      'RefundRequest' : IDL.Null,
      'NftTransferRequest' : IDL.Null,
      'NftTransferFailed' : NftTransferError,
      'BurnRequest' : IDL.Null,
      'FeeTransferRequest' : IDL.Null,
      'RefundFailed' : RefundError,
      'EscrowRequest' : IDL.Null,
    })
  );
  const SwapDetailReverse = IDL.Record({
    'nft_id' : IDL.Nat,
    'status' : SwapStatusReverse,
    'tokens_to_receive' : GldtNumTokens,
    'nft_id_string' : IDL.Text,
    'user' : IDL.Principal,
    'created_at' : IDL.Nat64,
    'swap_fee' : IDL.Nat,
    'nft_canister' : IDL.Principal,
    'index' : IDL.Nat,
    'transfer_fees' : IDL.Nat,
  });
  const SwapInfo = IDL.Variant({
    'Forward' : SwapDetailForward,
    'Reverse' : SwapDetailReverse,
  });
  const Args = IDL.Record({
    'user_principal' : IDL.Opt(IDL.Principal),
    'limit' : IDL.Nat64,
    'start' : IDL.Nat,
  });
  return IDL.Service({
    'archive_swap' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo)],
        [IDL.Null],
        [],
      ),
    'archive_swaps' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo))],
        [IDL.Null],
        [],
      ),
    'get_archive_size' : IDL.Func([IDL.Null], [IDL.Nat64], ['query']),
    'get_archive_swap' : IDL.Func(
        [IDL.Tuple(IDL.Nat, IDL.Nat)],
        [IDL.Opt(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo))],
        ['query'],
      ),
    'get_archive_swaps' : IDL.Func(
        [Args],
        [IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Nat, IDL.Nat), SwapInfo))],
        ['query'],
      ),
    'get_swap_bulk' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat))],
        [IDL.Vec(SwapInfo)],
        ['query'],
      ),
    'get_swap_indexes_for_user' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Nat)))],
        ['query'],
      ),
    'get_version' : IDL.Func([IDL.Null], [BuildVersion], ['query']),
  });
};
export const init = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    'version' : BuildVersion,
    'commit_hash' : IDL.Text,
  });
  const InitArgs = IDL.Record({
    'test_mode' : IDL.Bool,
    'authorized_principals' : IDL.Vec(IDL.Principal),
    'commit_hash' : IDL.Text,
  });
  const Args_1 = IDL.Variant({ 'Upgrade' : UpgradeArgs, 'Init' : InitArgs });
  return [Args_1];
};
