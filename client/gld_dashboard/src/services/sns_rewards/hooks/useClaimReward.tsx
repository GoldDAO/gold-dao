import { useMutation } from "@tanstack/react-query";
import { Buffer } from "buffer";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { useQueryClient } from "@tanstack/react-query";
import { idlFactory } from "../idlFactory";
import claim_rewards_batch from "../claim_rewards_batch";

const useClaimReward = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({
      claim_reward_args,
    }: {
      claim_reward_args: { neuron_id: string; token: string }[];
    }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        await claim_rewards_batch(
          actor,
          claim_reward_args.map(({ neuron_id, token }) => ({
            neuron_id: {
              id: [...Uint8Array.from(Buffer.from(neuron_id, "hex"))],
            },
            token,
          }))
        );
      } catch (err) {
        console.error(err);
        throw new Error(`Claim rewards error! Please retry later.`);
      }
    },
    onError: (err) => {
      console.log(err);
    },
    onSuccess: () => {
      // console.log(res);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: ["USER_NEURONS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_NEURON_REWARDS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_NEURONS_REWARDS"],
      });
    },
  });
};

export default useClaimReward;
