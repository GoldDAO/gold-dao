import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { SNS_REWARDS_CANISTER_ID, ICPSWAP_CANISTER_ID } from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "@services/icpswap/idls/swap_pool";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import get_token_price_usd from "@services/icpswap/get_token_price_usd";
import { TOKENS } from "@shared/utils/tokens";
import { Neuron } from "./index";

export type TokensRewards = {
  id: string;
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
    queryKey: ["USER_NEURON_REWARDS", owner, neuronId],
    queryFn: async (): Promise<TokensRewards[]> => {
      try {
        const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
          agent,
          canisterId: ICPSWAP_CANISTER_ID,
        });

        const data = await Promise.all(
          TOKENS.filter((token) =>
            ["GLDT", "OGY", "ICP", "GOLDAO", "WTN"].includes(token.name)
          ).map(async (token) => {
            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent,
              canisterId: token.canister_id,
            });
            const decimals = await icrc1_decimals(actorLedger);

            const reward = await icrc1_balance_of({
              actor: actorLedger,
              owner: SNS_REWARDS_CANISTER_ID,
              subaccount: neuronId,
            });

            const price_usd = await get_token_price_usd(
              actorIcpswap, token.canister_id, token.name, { agent }
            );
            return {
              id: token.id,
              amount: reward,
              amount_usd: price_usd * (Number(reward) / 10 ** decimals),
              neurons: [
                {
                  id: neuronId,
                  reward: reward,
                  reward_usd:
                    price_usd * (Number(reward) / 10 ** decimals),
                },
              ],
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
