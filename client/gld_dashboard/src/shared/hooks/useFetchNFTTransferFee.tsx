import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { OGY_LEDGER_CANISTER_ID } from "@constants";
import { idlFactory } from "@services/gld_nft/idlFactory";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import icrc7_transfer_fee from "@services/gld_nft/fn/icrc7_transfer_fee";
import icrc1_fee from "@services/ledger/icrc1_fee";
import icrc1_decimals from "@services/ledger/icrc1_decimals";

const useFetchNFTTransferFee = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<
    UseQueryOptions<{ amount: number; amount_e8s: bigint }>,
    "queryKey" | "queryFn"
  > & {
    nft_id: bigint;
    nft_id_string: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    refetchInterval = false,
    nft_id,
    nft_id_string,
  } = options;

  return useQuery({
    queryKey: [`FETCH_NFT_TRANSFER_FEE`, canisterId, nft_id_string],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });
        const actorLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: OGY_LEDGER_CANISTER_ID,
        });
        const feeLedger = await icrc1_fee(actorLedger);
        const decimals = await icrc1_decimals(actorLedger);
        const result = await icrc7_transfer_fee(actor, nft_id);
        const resultNumber = Number(result) / 10 ** decimals;
        const resultFeeNumber = Number(feeLedger) / 10 ** decimals;
        return {
          amount: resultNumber + resultFeeNumber,
          amount_e8s: result + feeLedger,
        };
      } catch (err) {
        console.log(err);
        throw new Error("Fetch NFT transfer fee error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchNFTTransferFee;
