import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { bigIntTo32ByteArray } from "@services/gld_nft/utils";
import { idlFactory as idlFactoryNFT } from "@services/gld_nft/idlFactory";
import { IdNFT, CollectionNameNFT } from "@services/gld_nft/utils/interfaces";
import icrc7_tokens_of from "@services/gld_nft/fn/icrc7_tokens_of";
import get_nat_as_token_id_origyn from "@services/gld_nft/fn/get_nat_as_token_id_origyn";
import get_active_swaps from "@services/gldt_swap/fn/get_active_swaps";
import { idlFactory as idlFactorySwap } from "@services/gldt_swap/idlFactory";

const useFetchAvailableNFT = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<IdNFT[]>, "queryKey" | "queryFn"> & {
    subaccount?: string[];
    collection_name: CollectionNameNFT;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    subaccount,
    collection_name,
  } = options;

  return useQuery({
    queryKey: ["FETCH_AVAILABLE_NFT", collection_name, subaccount],
    queryFn: async () => {
      try {
        const actorNFT = Actor.createActor(idlFactoryNFT, {
          agent,
          canisterId,
        });
        const actorSwap = Actor.createActor(idlFactorySwap, {
          agent,
          canisterId: SWAP_CANISTER_ID,
        });

        const icrc7_tokens_of_result = await icrc7_tokens_of(actorNFT, {
          owner: SWAP_CANISTER_ID,
          subaccount,
        });

        const result_user_nft = await Promise.all(
          icrc7_tokens_of_result.map(
            async (tokenId: bigint): Promise<IdNFT> => {
              const result = (await get_nat_as_token_id_origyn(actorNFT, {
                tokenId,
              })) as string;

              return {
                id_string: result,
                id_bigint: tokenId,
                id_byte_array: bigIntTo32ByteArray(tokenId),
              };
            }
          )
        );

        // ? Filter out NFT's that are currently being swapped
        const data = await get_active_swaps(actorSwap);
        const activeSwapSet = new Set(data.map((swap) => swap.nft_id_string));
        const result = result_user_nft.filter(
          (nft) => !activeSwapSet.has(nft.id_string)
        );

        return result ?? [];
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch ${collection_name} NFTs error! Please retry later.`
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchAvailableNFT;
