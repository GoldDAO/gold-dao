import {
  SwapDetailForward,
  SwapDetailReverse,
  SwapInfo,
} from "@canisters/gldt_swap/interfaces";
import { getDateUTC } from "@utils/dates";
import { GLDT_VALUE_1G_NFT } from '@constants'

export const bigintTo32ByteArray = (value: bigint) => {
  const byteArray = new Uint8Array(32);
  for (let i = byteArray.length - 1; i >= 0; i--) {
    byteArray[i] = Number(value & 0xffn);
    value >>= 8n;
  }
  return byteArray.reverse();
};

export const getSwapData = (swap: SwapInfo) => {
  let tx: SwapDetailForward | SwapDetailReverse;
  let type;
  let label;
  let status:
    | (typeof swapStatus.Forward)[keyof typeof swapStatus.Forward]
    | (typeof swapStatus.Reverse)[keyof typeof swapStatus.Reverse];
  let send_value;
  let receive_value;
  let gldt_value;
  let nft_value;

  if ("Forward" in swap) {
    type = "forward";
    label = "Forward";
    tx = swap.Forward;
    status =
      swapStatus.Forward[
        Object.keys(tx.status)[0] as keyof typeof swapStatus.Forward
      ];
    receive_value = Number(tx.tokens_to_mint.value) / 10 ** 8;
    send_value = receive_value / GLDT_VALUE_1G_NFT;
    gldt_value = receive_value
    nft_value = send_value
  } else {
    type = "reverse";
    label = "Reverse";
    tx = swap.Reverse;
    status =
      swapStatus.Reverse[
        Object.keys(tx.status)[0] as keyof typeof swapStatus.Reverse
      ];
    send_value = Number(tx.tokens_to_receive.value) / 10 ** 8;
    receive_value = send_value / GLDT_VALUE_1G_NFT;
    gldt_value = send_value
    nft_value = receive_value
  }

  const created_at = getDateUTC(Number(tx?.created_at), {
    fromMillis: true,
  });
  const nft_id_string = tx?.nft_id_string;
  const nft_id = tx?.nft_id.toString();
  const index = tx?.index.toString();

  return {
    type,
    label,
    created_at,
    nft_id_string,
    send_value,
    receive_value,
    gldt_value,
    nft_value,
    status,
    nft_id,
    index,
  };
};

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
