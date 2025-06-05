import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory } from "@services/gldt_swap/idlFactory";
import { CollectionNameNFT, IdNFT } from "@services/gld_nft/utils/interfaces";
import swap_tokens_for_nft from "@services/gldt_swap/fn/swap_tokens_for_nft";

const useSwapTokensForNFT = (
  agent: Agent | HttpAgent | undefined,
  collection: { canister_id: string; name: CollectionNameNFT }
) => {
  const queryClient = useQueryClient();
  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId: SWAP_CANISTER_ID,
  });
  return useMutation({
    mutationKey: ["SWAP_TOKENS_FOR_NFT"],
    mutationFn: async ({ nft }: { nft: IdNFT }): Promise<void> => {
      try {
        await swap_tokens_for_nft(actor, {
          nft_id: nft.id_bigint,
          nft_canister_id: Principal.fromText(collection.canister_id),
        });
      } catch (err) {
        console.error(err);
        if (err instanceof Error) {
          throw new Error(err.message, { cause: err });
        } else {
          throw new Error("An unknown error occurred", { cause: err });
        }
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["FETCH_AVAILABLE_NFT", collection.name],
      });
    },
  });
};

export default useSwapTokensForNFT;
