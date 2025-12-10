import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryNFT } from "@services/nft/idlFactory";
import { Result_14 } from "@services/nft/interfaces";

const useTransferNFT = (
  canister_id: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ to, token_id }: { to: string; token_id: bigint }) => {
      try {
        const actor = Actor.createActor(idlFactoryNFT, {
          agent,
          canisterId: canister_id,
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
        ])) as Result_14;

        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`Transfer NFT error!`);
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT", canister_id],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT_COUNT", canister_id],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT_METRICS"],
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
