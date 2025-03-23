import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";
import { ActivitySnapshot } from "../interfaces";

import get_activity_stats from "../get_activity_stats";

const useFetchHolders = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<ActivitySnapshot[]>, "queryKey" | "queryFn"> & {
    timestamp?: number;
  } = {}
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    timestamp = 30,
  } = options;

  return useQuery({
    queryKey: ["FETCH_HOLDERS", timestamp],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const results = await get_activity_stats(actor, {
          timestamp,
        });
        // console.log(results);
        return results;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch holders error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchHolders;
