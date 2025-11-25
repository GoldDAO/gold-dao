import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryNFT } from "@services/nft/idlFactory";
import { NFT } from "@services/nft/utils/interfaces";
import icrc7_tokens_of from "@services/nft/icrc7_tokens_of";
import get_active_swaps_by_user from "@services/swap/get_active_swaps_by_user";
import icrc7_token_metadata from "@services/nft/icrc7_token_metadata";
import { idlFactory as idlFactorySwap } from "@services/swap/idlFactory";
import { parseMetadata } from "@services/nft/utils/index";

const useFetchUserNft = (
  canister_id: string,
  options: Omit<UseQueryOptions<NFT[]>, "queryKey" | "queryFn"> & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
    subaccount?: string[];
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    owner,
    subaccount,
    agent,
  } = options;

  return useQuery({
    queryKey: ["FETCH_USER_NFT", canister_id, owner, subaccount],
    queryFn: async () => {
      try {
        const actorNFT = Actor.createActor(idlFactoryNFT, {
          agent,
          canisterId: canister_id,
        });
        const actorSwap = Actor.createActor(idlFactorySwap, {
          agent,
          canisterId: SWAP_CANISTER_ID,
        });

        const activeSwaps = await get_active_swaps_by_user(actorSwap, owner);

        const icrc7_tokens_of_results = await icrc7_tokens_of(actorNFT, {
          owner,
          subaccount,
        });

        const nfts = await Promise.all(
          icrc7_tokens_of_results.map(
            async (token_id: bigint): Promise<NFT> => {
              const result = await icrc7_token_metadata(actorNFT, {
                token_id,
              });
              const metadata = await parseMetadata(result);

              return {
                id: token_id,
                ...metadata,
              };
            }
          )
        );

        //? Filter out NFT's that are currently being swapped
        const activeSwapSet = new Set(activeSwaps.map((swap) => swap.nft_id));
        const filtered = nfts.filter((nft) => !activeSwapSet.has(nft.id));

        return filtered;
      } catch (err) {
        console.error(err);
        throw new Error(`Fetch NFT's collection error! Please retry later.`);
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchUserNft;
