import { useQueryClient, useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryStake } from "@services/stake/idlFactory";
import dissolve_instantly from "@services/stake/dissolve_instantly";

const useDissolvingInstantly = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ fraction }: { fraction: number }) => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId,
        });

        const result = await dissolve_instantly(actorStake, fraction);
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
        queryKey: ["FETCH_LEDGER_BALANCE", "GLDT"],
      });
    },
  });
};

export default useDissolvingInstantly;
