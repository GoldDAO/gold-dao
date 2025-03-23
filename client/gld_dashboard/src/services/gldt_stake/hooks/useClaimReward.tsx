import { useMutation } from "@tanstack/react-query";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "@services/gldt_stake/idlFactory";

import {
  Result,
  StakePositionResponse,
  Args,
} from "@services/gldt_stake/interfaces";

const claim_reward = async (
  actor: ActorSubclass,
  args: Args
): Promise<StakePositionResponse> => {
  const { id, token } = args;

  const result = (await actor.claim_reward({ id, token })) as Result;

  if ("Err" in result) throw result.Err;

  return result.Ok;
};

const useClaimReward = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  return useMutation({
    mutationFn: async ({ id, token }: Args) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const result = await claim_reward(actor, {
          id,
          token,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`claim_reward error! Please retry later.`);
      }
    },
  });
};

export default useClaimReward;
