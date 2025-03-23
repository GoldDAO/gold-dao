import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import { Ledger } from "../utils/interfaces";
import icrc1_fee from "../icrc1_fee";

const useFetchTransferFee = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<bigint>, "queryKey" | "queryFn"> & {
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
    queryKey: [`FETCH_LEDGER_TX_FEE_${ledger.toLocaleUpperCase()}`, ledger],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });
        const result = await icrc1_fee(actor);
        return result;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch transfer fee error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchTransferFee;
