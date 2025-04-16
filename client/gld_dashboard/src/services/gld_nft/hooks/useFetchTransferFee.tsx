import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../idlFactory";
import icrc7_transfer_fee from "../icrc7_transfer_fee";

const useFetchTransferFeeNFT = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<bigint>, "queryKey" | "queryFn"> & {
    nft_id: bigint;
    nft_id_string: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    refetchInterval = false,
    nft_id,
    nft_id_string,
  } = options;

  return useQuery({
    queryKey: [`FETCH_NFT_TRANSFER_FEE`, canisterId, nft_id_string],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });
        const result = await icrc7_transfer_fee(actor, nft_id);
        return result;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch NFT transfer fee error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchTransferFeeNFT;
