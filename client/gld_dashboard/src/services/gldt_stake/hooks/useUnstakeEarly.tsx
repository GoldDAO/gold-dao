import { useMutation, useQueryClient } from "@tanstack/react-query";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "@services/gldt_stake/idlFactory";

import {
  StakePositionResponse,
  Result_7,
} from "@services/gldt_stake/interfaces";

const unstake_early = async (
  actor: ActorSubclass,
  args: { id: bigint }
): Promise<StakePositionResponse> => {
  const { id } = args;
  const result = (await actor.unstake_early(id)) as Result_7;
  if ("Err" in result) throw result.Err;
  return result.Ok;
};

const useUnstakeEarly = (
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

        const result = await unstake_early(actor, {
          id,
        });
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`unstake_early error! Please retry later.`);
      }
    },
    onSuccess: () => {
      // console.log(res);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", "GLDT"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS_REWARDS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_POSITIONS_TOTAL_STAKED_AMOUNT"],
      });
    },
  });
};

export default useUnstakeEarly;
