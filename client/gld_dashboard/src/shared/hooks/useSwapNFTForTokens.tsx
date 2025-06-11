import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SWAP_CANISTER_ID } from "@constants";
import { idlFactory } from "@services/gldt_swap/idlFactory";
import { CollectionNameNFT, IdNFT } from "@services/gld_nft/utils/interfaces";
import swap_nft_for_tokens from "@services/gldt_swap/fn/swap_nft_for_tokens";

const useSwapNFTForTokens = (
  agent: Agent | HttpAgent | undefined,
  collection: { canister_id: string; name: CollectionNameNFT }
) => {
  const queryClient = useQueryClient();
  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId: SWAP_CANISTER_ID,
  });
  return useMutation({
    mutationKey: ["SWAP_NFT_FOR_TOKENS"],
    mutationFn: async ({ nft }: { nft: IdNFT }): Promise<void> => {
      try {
        await swap_nft_for_tokens(actor, [
          [nft.id_bigint, Principal.fromText(collection.canister_id)] as [
            bigint,
            Principal
          ],
        ]);
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
        queryKey: ["FETCH_USER_NFT", collection.name],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE_V2", { ledger: "GLDT" }],
      });
    },
  });
};

export default useSwapNFTForTokens;
