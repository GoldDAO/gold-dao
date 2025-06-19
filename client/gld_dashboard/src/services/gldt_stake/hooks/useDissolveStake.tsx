import { useMutation, useQueryClient } from "@tanstack/react-query";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "@services/gldt_stake/idlFactory";

import {
  StakePositionResponse,
  Result_5,
} from "@services/gldt_stake/interfaces";

const start_dissolving = async (
  actor: ActorSubclass,
  args: { id: bigint }
): Promise<StakePositionResponse> => {
  const { id } = args;

  const result = (await actor.start_dissolving(id)) as Result_5;

  if ("Err" in result) throw result.Err;

  return result.Ok;
};

const useDissolveStake = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ id }: { id: bigint }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const result = await start_dissolving(actor, {
          id,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`start_dissolving error! Please retry later.`);
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS"],
      });
    },
  });
};

export default useDissolveStake;
