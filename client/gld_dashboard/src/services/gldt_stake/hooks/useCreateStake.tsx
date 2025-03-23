import { useMutation } from "@tanstack/react-query";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "@services/gldt_stake/idlFactory";

import {
  Result_2,
  StakePositionResponse,
} from "@services/gldt_stake/interfaces";

interface CreateStakeArgs {
  amount: bigint;
}

const create_stake_position = async (
  actor: ActorSubclass,
  args: CreateStakeArgs
): Promise<StakePositionResponse> => {
  const { amount } = args;

  const result = (await actor.create_stake_position({ amount })) as Result_2;

  if ("Err" in result) throw result.Err;

  return result.Ok;
};

const useCreateStake = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  return useMutation({
    mutationFn: async ({ amount }: CreateStakeArgs) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const result = await create_stake_position(actor, {
          amount,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`create_stake_position error! Please retry later.`);
      }
    },
  });
};

export default useCreateStake;
