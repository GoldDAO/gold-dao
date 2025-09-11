import { useQueryClient, useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryStake } from "@services/gldt_stake/interfaces/idlFactory";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import add_stake from "@services/gldt_stake/add_stake";
import icrc2_approve from "@services/ledger/icrc2_approve";
import { GLDT_LEDGER_CANISTER_ID } from "@constants";

const useAddStake = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ amount }: { amount: bigint }) => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId,
        });
        const actorLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: GLDT_LEDGER_CANISTER_ID,
        });
        await icrc2_approve(actorLedger, {
          amount,
          spender: { owner: canisterId },
        });

        const result = await add_stake(actorStake, amount);
        return result;
      } catch (err) {
        throw new Error(err instanceof Error ? err.message : "Unknown error.");
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["FETCH_TOTAL_STAKED_AMOUNT"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_STAKED_USER_POSITION"],
      });
      queryClient.invalidateQueries({
        queryKey: [`FETCH_LEDGER_BALANCE`, "GLDT"],
      });
    },
  });
};

export default useAddStake;
