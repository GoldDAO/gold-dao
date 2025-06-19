import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryNFT } from "@services/gld_nft/idlFactory";
import { TransferResult } from "@services/gld_nft/interfaces";

const useTransferNFT = (
  canisterId: string,
  nftCollectionName: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ to, token_id }: { to: string; token_id: bigint }) => {
      try {
        const actor = Actor.createActor(idlFactoryNFT, {
          agent,
          canisterId,
        });

        const result = (await actor.icrc7_transfer([
          {
            to: {
              owner: Principal.fromText(to),
              subaccount: [],
            },
            token_id,
            memo: [],
            from_subaccount: [],
            created_at_time: [],
          },
        ])) as TransferResult;
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`Transfer NFT error! Please retry later.`);
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT", nftCollectionName],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", "GLDT"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", "OGY"],
      });
    },
  });
};

export default useTransferNFT;
