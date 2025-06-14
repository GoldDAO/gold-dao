import {
  useQuery,
  // keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import { Ledger } from "../utils/interfaces";

const useFetchUserBalance = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<bigint>, "queryKey" | "queryFn"> & {
    ledger: Ledger;
    owner: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = undefined,
    ledger,
    owner,
  } = options;

  return useQuery({
    queryKey: ["FETCH_LEDGER_BALANCE", ledger, owner],
    queryFn: async () => {
      const actor = Actor.createActor(idlFactory, {
        agent,
        canisterId,
      });

      const result = await icrc1_balance_of({
        actor,
        owner,
      });
      return result;
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchUserBalance;
