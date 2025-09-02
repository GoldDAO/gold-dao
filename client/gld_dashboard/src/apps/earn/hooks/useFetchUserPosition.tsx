import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
// import { DateTime } from "luxon";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  KONGSWAP_CANISTER_ID_IC,
  GLDT_LEDGER_CANISTER_ID,
  MAX_DISSOLVE_EVENTS,
  INSTANT_DISSOLVE_FEE_PERCENTAGE,
} from "@constants";
import {
  TOKEN_GLDT,
  TOKEN_GOLDAO_IC,
  // TOKEN_ICP_IC,
  // TOKEN_OGY_IC,
} from "@shared/utils/tokens";
import { idlFactory as idlFactoryStake } from "@services/gldt_stake/interfaces/idlFactory";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import get_position from "@services/gldt_stake/get_position";
import icrc1_fee from "@services/ledger/icrc1_fee";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import swap_amounts from "@services/kongswap/swap_amounts";
import { Position, Reward } from "@earn/interfaces";

const useFetchUserPosition = (
  canister_id: string,
  authenticatedAgent: Agent | HttpAgent | undefined,
  unauthenticatedAgent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<Position, Error>, "queryKey" | "queryFn">
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
  } = options;

  return useQuery({
    queryKey: ["FETCH_STAKED_USER_POSITION"],
    queryFn: async (): Promise<Position> => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent: authenticatedAgent,
          canisterId: canister_id,
        });
        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent: unauthenticatedAgent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });
        const actorLedgerGLDT = Actor.createActor(idlFactoryLedger, {
          agent: unauthenticatedAgent,
          canisterId: GLDT_LEDGER_CANISTER_ID,
        });

        const result_arr = await get_position(actorStake);

        if (!result_arr.length && !result_arr[0])
          return {
            rewards: [],
            created_at: 0n,
            instant_dissolve_fee: 0,
            instant_dissolve_fee_e8s: 0n,
            instant_dissolve_fee_percentage: 0,
            age_bonus_multiplier: 0,
            staked_amount: 0,
            staked_amount_e8s: 0n,
            staked_amount_usd: 0,
            total_rewards_amount: 0,
            total_rewards_amount_e8s: 0n,
            total_rewards_amount_usd: 0,
            is_enable_claiming_rewards: false,
            dissolve_events: [],
            dissolve_events_count: 0,
            remaining_dissolve_events: 0,
            max_dissolve_events: MAX_DISSOLVE_EVENTS,
            is_enable_withdrawing: false,
          } satisfies Position;

        const result = result_arr[0];

        const decimalsGLDT = await icrc1_decimals(actorLedgerGLDT);
        const priceGLDT = await swap_amounts(actorKongswap, {
          from: TOKEN_GLDT.name,
          to: "ckUSDT",
          amount: 1n,
        });

        const claimable_rewards = result.claimable_rewards.map(
          ([tokenSymbol, amount]) => {
            const tokenName = Object.keys(tokenSymbol)[0];
            return {
              name: tokenName,
              amount: amount,
            };
          }
        );

        const rewards: Reward[] = await Promise.all(
          [TOKEN_GOLDAO_IC].map(async (token) => {
            const res = {
              ...token,
              is_selected: false,
              is_claimable: false,
              amount: 0,
              amount_e8s: 0n,
              amount_usd: 0,
              is_amount_below_fee: false,
            };

            const token_reward = claimable_rewards.find(
              ({ name }) => name === token.name
            );

            if (!token_reward) return res;

            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent: unauthenticatedAgent,
              canisterId: token.canister_id,
            });
            const decimals = await icrc1_decimals(actorLedger);
            const fee = await icrc1_fee(actorLedger);

            const is_claimable =
              token_reward.amount > fee && token_reward.amount > 0n;

            const is_amount_below_fee =
              token_reward.amount <= fee && token_reward.amount > 0n;

            const price = await swap_amounts(actorKongswap, {
              from: token.name,
              to: "ckUSDT",
              amount: BigInt(1 * 10 ** decimals),
            });

            const amount = Number(token_reward.amount) / 10 ** decimals;

            return {
              ...res,
              is_selected: is_claimable,
              is_claimable,
              is_amount_below_fee,
              amount,
              amount_e8s: token_reward.amount,
              amount_usd: price.mid_price * amount,
            };
          })
        );

        const dissolve_events = result.dissolve_events.map((event) => {
          const { amount } = event;
          const dissolved_amount = Number(amount) / 10 ** decimalsGLDT;
          const dissolved_date = Number(event.dissolved_date);
          const date_now = Date.now();
          return {
            ...event,
            amount_e8s: amount,
            amount: dissolved_amount,
            amount_usd: priceGLDT.mid_price * dissolved_amount,
            dissolved_date,
            is_withdrawable: dissolved_date <= date_now,
            remaining_time: Math.max(dissolved_date - date_now, 0),
          };
        });

        const dissolve_events_count = dissolve_events.length;

        const total_rewards_amount = rewards.reduce(
          (acc, curr) => acc + curr.amount,
          0
        );
        const total_rewards_amount_e8s = rewards.reduce(
          (acc, curr) => acc + curr.amount_e8s,
          0n
        );
        const total_rewards_amount_usd = rewards.reduce(
          (acc, curr) => acc + curr.amount_usd,
          0
        );

        const is_enable_claiming_rewards = rewards.some(
          (reward) => reward.is_claimable
        );

        const is_enable_withdrawing = dissolve_events.some(
          (event) => event.is_withdrawable
        );

        const staked_amount = Number(result.staked) / 10 ** decimalsGLDT;
        const instant_dissolve_fee =
          Number(result.instant_dissolve_fee) / 10 ** decimalsGLDT;

        const ret = {
          rewards,
          created_at: result.created_at,
          instant_dissolve_fee,
          instant_dissolve_fee_e8s: result.instant_dissolve_fee,
          instant_dissolve_fee_percentage: INSTANT_DISSOLVE_FEE_PERCENTAGE,
          age_bonus_multiplier: result.age_bonus_multiplier,
          staked_amount,
          staked_amount_e8s: result.staked,
          staked_amount_usd: priceGLDT.mid_price * staked_amount,
          total_rewards_amount,
          total_rewards_amount_e8s,
          total_rewards_amount_usd,
          is_enable_claiming_rewards,
          dissolve_events,
          dissolve_events_count,
          remaining_dissolve_events:
            MAX_DISSOLVE_EVENTS - dissolve_events_count,
          max_dissolve_events: MAX_DISSOLVE_EVENTS,
          is_enable_withdrawing,
        } satisfies Position;

        return ret;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch user position error! Please retry later.");
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

export default useFetchUserPosition;
