export const swapStatus = {
  Forward: {
    Init: { value: "Init", label: "Opening Sale" },
    NotificationFailed: {
      value: "NotificationFailed",
      label: "Notification Failed",
    },
    MintRequest: { value: "MintRequest", label: "Minting" },
    MintFailed: { value: "MintFailed", label: "Mint Failed" },
    BidFail: { value: "BidFail", label: "Bid Fail" },
    BidRequest: { value: "BidRequest", label: "Bid Request" },
    BurnFeesRequest: { value: "BurnFeesRequest", label: "Burning Fees" },
    BurnFeesFailed: { value: "BurnFeesFailed", label: "Burn Fees Failed" },
    Complete: { value: "Complete", label: "Success" },
    Failed: { value: "Failed", label: "Failed" },
  },
  Reverse: {
    Init: { value: "Init", label: "Opening Sale" },
    RefundRequest: { value: "RefundRequest", label: "Refunding" },
    RefundFailed: { value: "RefundFailed", label: "Refund Failed" },
    NftTransferRequest: {
      value: "NftTransferRequest",
      label: "Transfering NFT",
    },
    NftTransferFailed: {
      value: "NftTransferFailed",
      label: "NFT Transfer Failed",
    },
    BurnRequest: { value: "BurnRequest", label: "Burning" },
    BurnFailed: { value: "BurnFailed", label: "Burn Failed" },
    FeeTransferRequest: {
      value: "FeeTransferRequest",
      label: "Transfering Fee",
    },
    FeeTransferFailed: {
      value: "FeeTransferFailed",
      label: "Fee Transfer Failed",
    },
    EscrowRequest: { value: "EscrowRequest", label: "Escrow Request" },
    EscrowFailed: { value: "EscrowFailed", label: "Escrow Failed" },
    Complete: { value: "Complete", label: "Success" },
    Failed: { value: "Failed", label: "Failed" },
  },
};

export type ISwapStatus = typeof swapStatus;
