import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import get_sns_canisters_summary, {
  SNSCanistersSummaryData,
} from "../get_sns_canisters_summary";

import { idlFactory } from "../idlFactory";

export type { SNSCanistersSummaryData };

const useFetchAllCanisters = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<SNSCanistersSummaryData[]>,
    "queryKey" | "queryFn"
  > = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["FETCH_ALL_CANISTERS"],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });
        const results = (await get_sns_canisters_summary({
          actor,
        })) as SNSCanistersSummaryData[];
        return results;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch all canisters error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchAllCanisters;
