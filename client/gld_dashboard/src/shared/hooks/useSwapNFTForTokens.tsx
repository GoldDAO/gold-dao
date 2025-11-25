import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory } from "@services/swap/idlFactory";
import { NFT } from "@services/nft/utils/interfaces";
import swap_nft_for_tokens from "@services/swap/swap_nft_for_tokens";

const useSwapNFTForTokens = (
  agent: Agent | HttpAgent | undefined,
  collection: { canister_id: string }
) => {
  const queryClient = useQueryClient();
  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId: SWAP_CANISTER_ID,
  });
  return useMutation({
    mutationKey: ["SWAP_NFT_FOR_TOKENS"],
    mutationFn: async ({ nfts }: { nfts: NFT[] }): Promise<void> => {
      const canister_id_collection = Principal.fromText(collection.canister_id);
      const params = nfts.map((nft) => ({
        id: nft.id,
        canister_id: canister_id_collection,
      }));
      try {
        await swap_nft_for_tokens(actor, params);
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
      console.log("NFT swapped for tokens successfully");
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT", collection.canister_id],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT_METRICS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", "GLDT"],
      });
    },
  });
};

export default useSwapNFTForTokens;
