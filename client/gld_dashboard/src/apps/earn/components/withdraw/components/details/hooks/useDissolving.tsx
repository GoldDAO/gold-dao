import { useQueryClient, useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryStake } from "@services/stake/idlFactory";
import start_dissolving from "@services/stake/start_dissolving";

const useDissolving = (
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

        const result = await start_dissolving(actorStake, fraction);
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

export default useDissolving;
