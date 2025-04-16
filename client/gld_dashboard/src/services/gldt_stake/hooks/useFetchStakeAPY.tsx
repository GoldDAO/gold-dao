import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../idlFactory";
import { ActorSubclass } from "@dfinity/agent";

const get_apy_overall = async (actor: ActorSubclass): Promise<number> => {
  const result = (await actor.get_apy_overall(null)) as number;
  return result;
};

const useFetchStakeAPY = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<number, Error>, "queryKey" | "queryFn">
) => {
  const {
    enabled,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["STAKE_APY"],
    queryFn: async (): Promise<number> => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const res = await get_apy_overall(actor);

        return res;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch stake APY error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useFetchStakeAPY;
