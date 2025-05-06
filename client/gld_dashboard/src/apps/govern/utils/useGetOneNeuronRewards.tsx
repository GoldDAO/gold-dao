import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SNS_REWARDS_CANISTER_ID, KONGSWAP_CANISTER_ID_IC } from "@constants";

import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import swap_amounts from "@services/kongswap/swap_amounts";
import { TokensList } from "./index";
import { Ledger } from "@services/ledger/utils/interfaces";
import { Neuron } from "./index";

export type TokensRewards = {
  id: Ledger;
  amount: bigint;
  amount_usd: number;
  neurons: Neuron[];
};

const useGetOneNeuronRewards = (
  options: Omit<
    UseQueryOptions<TokensRewards[], Error>,
    "queryKey" | "queryFn"
  > & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
    neuronId: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    agent,
    owner,
    neuronId,
  } = options;

  return useQuery({
    queryKey: ["USER_ONE_NEURON_REWARDS", owner, neuronId],
    queryFn: async (): Promise<TokensRewards[]> => {
      try {
        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });

        const data = await Promise.all(
          TokensList.map(async (token) => {
            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent,
              canisterId: token.canisterId,
            });
            const decimals = await icrc1_decimals(actorLedger);

            const reward = await icrc1_balance_of({
              actor: actorLedger,
              owner: SNS_REWARDS_CANISTER_ID,
              subaccount: neuronId,
            });

            const price = await swap_amounts(actorKongswap, {
              from: token.name,
              to: "ckUSDC",
              amount: reward,
            });
            return {
              id: token.id,
              amount: reward,
              amount_usd: price.mid_price * (Number(reward) / 10 ** decimals),
              neurons: [],
            };
          })
        );
        return data;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch one neuron rewards error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetOneNeuronRewards;
