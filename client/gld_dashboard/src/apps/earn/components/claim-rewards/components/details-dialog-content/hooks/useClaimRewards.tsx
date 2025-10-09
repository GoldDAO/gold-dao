import { useQueryClient, useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryStake } from "@services/gldt_stake/idlFactory";
import claim_rewards from "@services/gldt_stake/claim_rewards";

const useClaimRewards = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ tokens }: { tokens: string[] }) => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId,
        });
        const result = await claim_rewards(actorStake, tokens);
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

export default useClaimRewards;
