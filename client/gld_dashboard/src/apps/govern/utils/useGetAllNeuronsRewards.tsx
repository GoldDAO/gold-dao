import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  SNS_REWARDS_CANISTER_ID,
  SNS_GOVERNANCE_CANISTER_ID,
  ICPSWAP_CANISTER_ID,
} from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryGovernance } from "@services/sns_governance/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "@services/icpswap/idls/swap_pool";
import { icrc1_balance_of } from "@services/ledger/icrc1_balance_of";
import list_neurons from "@services/sns_governance/list_neurons";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import { fetch_all_tokens, find_token_price_usd } from "@services/icpswap/get_token_price_usd";
import { TOKENS } from "@shared/utils/tokens";
import { Neuron } from "./index";

export type TokensRewards = {
  id: string;
  amount: bigint;
  amount_usd: number;
  neurons: Neuron[];
};

const useGetAllNeuronsRewards = (
  options: Omit<
    UseQueryOptions<TokensRewards[], Error>,
    "queryKey" | "queryFn"
  > & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    agent,
    owner,
  } = options;

  return useQuery({
    queryKey: ["USER_NEURONS_REWARDS", owner],
    queryFn: async (): Promise<TokensRewards[]> => {
      try {
        const actor = Actor.createActor(idlFactoryGovernance, {
          agent,
          canisterId: SNS_GOVERNANCE_CANISTER_ID,
        });

        const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
          agent,
          canisterId: ICPSWAP_CANISTER_ID,
        });

        const neurons = await list_neurons(actor, {
          limit: 100,
          start_page_at: null,
          owner,
        });

        const allTokens = await fetch_all_tokens(actorIcpswap);

        const data = await Promise.all(
          TOKENS.filter((token) =>
            ["GLDT", "OGY", "ICP", "GOLDAO", "WTN"].includes(token.name)
          ).map(async (token) => {
            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent,
              canisterId: token.canister_id,
            });
            const decimals = await icrc1_decimals(actorLedger);
            const price_usd = find_token_price_usd(
              allTokens,
              token.canister_id,
              token.name
            );
            const neuronData = await Promise.all(
              neurons.map(async (neuron) => {
                const reward = await icrc1_balance_of({
                  actor: actorLedger,
                  owner: SNS_REWARDS_CANISTER_ID,
                  subaccount: neuron.id,
                });

                return {
                  id: neuron.id,
                  reward,
                  reward_usd:
                    price_usd * (Number(reward) / 10 ** decimals),
                };
              })
            );
            const amount = neuronData.reduce(
              (acc, curr) => acc + curr.reward,
              0n
            );
            const amount_usd = neuronData.reduce(
              (acc, curr) => acc + curr.reward_usd,
              0
            );
            return {
              id: token.id,
              amount,
              amount_usd,
              neurons: neuronData,
            };
          })
        );
        return data;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch neurons all rewards error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetAllNeuronsRewards;
