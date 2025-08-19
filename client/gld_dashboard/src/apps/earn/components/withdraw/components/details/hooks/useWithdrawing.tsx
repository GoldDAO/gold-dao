import { useQueryClient, useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryStake } from "@services/gldt_stake/interfaces/idlFactory";
import withdraw from "@services/gldt_stake/withdraw";

const useWithdrawing = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async () => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId,
        });

        const result = await withdraw(actorStake);
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
    },
  });
};

export default useWithdrawing;
