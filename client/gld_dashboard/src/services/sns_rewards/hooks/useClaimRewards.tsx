import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import claim_reward from "../claim_reward";

const useClaimRewards = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: {
    neuronIds: string[];
  }
) => {
  const { neuronIds } = options;
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      tokens,
    }: {
      tokens: ("OGY" | "ICP" | "GLDGov")[];
    }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const promises = tokens.flatMap((token) =>
          neuronIds.map((neuronId) =>
            claim_reward({
              neuronId,
              token,
              actor,
            })
          )
        );
        await Promise.all(promises);
      } catch (err) {
        console.error(err);
        throw new Error("Claim rewards error! Please retry later.");
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

export default useClaimRewards;
