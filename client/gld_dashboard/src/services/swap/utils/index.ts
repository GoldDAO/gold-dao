import {
  SwapInfo,
} from "../interfaces";
import { SwapData } from './interfaces'
import { getDateUTC } from "@shared/utils/dates";
import { GLDT_VALUE_1G_NFT } from '@constants'

export const getSwapData = (swap: SwapInfo): SwapData => {
  let type: "forward" | "reverse";
  let label: string;
  let send_value: number;
  let receive_value: number;
  let nft_value: number

  const value: number = Number(swap.tokens_amount.division) / 10 ** 8;
  const gldt_value = value * GLDT_VALUE_1G_NFT

  if ("Forward" in swap.swap_type) {
    type = "forward";
    label = "Forward";
    receive_value = gldt_value;
    send_value = value;
    nft_value = send_value;
  } else {
    type = "reverse";
    label = "Reverse";
    receive_value = value / GLDT_VALUE_1G_NFT;
    send_value = gldt_value;
    nft_value = receive_value;
  }

  const status =
    swapStatus[
    Object.keys(swap.status)[0] as keyof typeof swapStatus
    ];
  const created_at = getDateUTC(Number(swap.created_at), {
    fromMillis: true,
  });
  const nft_id_string = swap.nft?.id?.toString();
  const nft_id = swap.nft?.id;
  const index = swap.index?.toString();

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
  Init: { value: "Init", label: "Init" },
  Minted: { value: "Minted", label: "Minted" },
  MintFailed: { value: "MintFailed", label: "Mint failed" },
  Complete: { value: "Complete", label: "Complete" },
  Failed: { value: "Failed", label: "Failed" },
  Burned: { value: "Burned", label: "Burned" },
  BurnFailed: { value: "BurnFailed", label: "Burn failed" },
  NftTransferred: { value: "NftTransferred", label: "Nft transferred" },
  NftTransferredFrom: { value: "NftTransferredFrom", label: "Nft transferred from" },
  NftTransferFailed: { value: "NftTransferFailed", label: "Nft transfer failed" },
  NftTransferFromFailed: { value: "NftTransferFromFailed", label: "Nft transfer from failed" },
  Reimbursed: { value: "Reimbursed", label: "Reimbursed" },
  ReimbursedFailed: { value: "ReimburseFailed", label: "Reimburse failed" },
};

