import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import { Ledger } from "../utils/interfaces";
import icrc1_decimals from "../icrc1_decimals";

const useFetchDecimals = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<number>, "queryKey" | "queryFn"> & {
    ledger: Ledger;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    refetchInterval = false,
    ledger,
  } = options;

  return useQuery({
    queryKey: [`FETCH_LEDGER_DECIMALS_${ledger.toLocaleUpperCase()}`, ledger],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });
        const result = await icrc1_decimals(actor);
        return result;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch decimals error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchDecimals;
