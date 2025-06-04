import { ActorSubclass } from "@dfinity/agent";
import {
  MarketTransferRequest,
  MarketTransferResult,
} from "@services/gld_nft/interfaces";

const market_transfer_batch_nft_origyn = async (
  actor: ActorSubclass,
  request: MarketTransferRequest[]
) => {
  const result = (await actor.market_transfer_batch_nft_origyn(
    request
  )) as MarketTransferResult;
  if ("err" in result) {
    throw new Error(result.err.text, { cause: result.err });
  }
  return result.ok;
};

export default market_transfer_batch_nft_origyn;
