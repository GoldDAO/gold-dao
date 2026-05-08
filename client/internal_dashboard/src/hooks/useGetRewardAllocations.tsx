import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryStake } from "../services/gldt_stake/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "../services/icpswap/idlFactory";
import { ICPSWAP_CANISTER_ID } from "../constants";
import get_daily_analytics from "../services/gldt_stake/get_daily_analytics";
import get_token_price_usd from "../services/icpswap/get_token_price_usd";

export interface TokenReward {
  name: string;
  amount: number;
  amount_e8s: bigint;
  amount_usd: number;
}

export interface DailyAnalytic {
  date: number;
  apy: number;
  staked_gldt: number;
  staked_gldt_e8s: bigint;
  staked_gldt_usd: number;
  weighted_stake: number;
  weighted_stake_e8s: bigint;
  weighted_stake_usd: number;
  rewards: TokenReward[];
  total_rewards_usd: number;
}

interface useGetRewardAllocationsParams {
  starting_day: bigint;
  limit?: bigint;
}

const useGetRewardAllocations = (
  canister_id: string,
  options?: Omit<
    UseQueryOptions<DailyAnalytic[], Error>,
    "queryKey" | "queryFn"
  >
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
  } = options || {};

  return useQuery({
    queryKey: ["FETCH_REWARD_ALLOCATIONS", canister_id],
    queryFn: async (): Promise<DailyAnalytic[]> => {
      try {
        const agent = await HttpAgent.create({ host: "https://ic0.app" });

        const actorStake = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId: canister_id,
        });
        const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
          agent,
          canisterId: ICPSWAP_CANISTER_ID,
        });

        const result = await get_daily_analytics(actorStake);

        const GLDT_DECIMALS = 8;

        const priceGLDT = await get_token_price_usd(
          actorIcpswap, undefined, "GLDT", { agent }
        );

        const TOKEN_SYMBOLS = ["GOLDAO", "ICP", "OGY"];

        const analytics = await Promise.all(
          result.map(async ([timestamp, data]) => {
            const staked_gldt = Number(data.staked_gldt) / 10 ** GLDT_DECIMALS;
            const weighted_stake =
              Number(data.weighted_stake) / 10 ** GLDT_DECIMALS;

            const rewards: TokenReward[] = await Promise.all(
              data.rewards.map(async ([tokenSymbol, amount]) => {
                const tokenName = Object.keys(tokenSymbol)[0];

                const price_usd = await get_token_price_usd(
                  actorIcpswap, undefined, tokenName, { agent }
                );

                // Get token decimals based on symbol
                let decimals = 8; // Default for most tokens
                if (tokenName === "ICP") decimals = 8;
                else if (tokenName === "OGY") decimals = 8;
                else if (tokenName === "GOLDAO") decimals = 8;

                const tokenAmount = Number(amount) / 10 ** decimals;

                return {
                  name: tokenName,
                  amount: tokenAmount,
                  amount_e8s: amount,
                  amount_usd: price_usd * tokenAmount,
                };
              })
            );

            const total_rewards_usd = rewards.reduce(
              (acc, reward) => acc + reward.amount_usd,
              0
            );

            return {
              date: Number(timestamp),
              apy: data.apy,
              staked_gldt,
              staked_gldt_e8s: data.staked_gldt,
              staked_gldt_usd: priceGLDT * staked_gldt,
              weighted_stake,
              weighted_stake_e8s: data.weighted_stake,
              weighted_stake_usd: priceGLDT * weighted_stake,
              rewards,
              total_rewards_usd,
            } satisfies DailyAnalytic;
          })
        );

        return analytics;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch daily analytics error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
    staleTime,
    refetchOnMount,
    refetchOnWindowFocus,
    refetchOnReconnect,
  });
};

export default useGetRewardAllocations;
