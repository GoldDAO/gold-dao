import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../../services/ledger/idlFactory";
import icrc1_decimals from "../../services/ledger/icrc1_decimals";

const useFetchLedgerDecimals = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<number>, "queryKey" | "queryFn"> & {
    ledger: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    refetchInterval = false,
    ledger,
  } = options;

  return useQuery({
    queryKey: ["FETCH_LEDGER_DECIMALS", ledger],
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

export default useFetchLedgerDecimals;
