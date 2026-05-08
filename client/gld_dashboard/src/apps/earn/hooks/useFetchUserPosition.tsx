import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  ICPSWAP_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  MAX_DISSOLVE_EVENTS,
  INSTANT_DISSOLVE_FEE_PERCENTAGE,
} from "@constants";
import { TOKEN_GLDT, TOKEN_GOLDAO_IC } from "@shared/utils/tokens";
import { idlFactory as idlFactoryStake } from "@services/stake/idlFactory";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "@services/icpswap/idls/swap_pool";
import get_position from "@services/stake/get_position";
import icrc1_fee from "@services/ledger/icrc1_fee";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import get_token_price_usd from "@services/icpswap/get_token_price_usd";
import { Position, Reward } from "@earn/interfaces";

const useFetchUserPosition = (
  canister_id: string,
  options: Omit<UseQueryOptions<Position, Error>, "queryKey" | "queryFn"> & {
    agent: Agent | HttpAgent | undefined;
    owner: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    agent,
    owner,
  } = options;

  return useQuery({
    queryKey: ["FETCH_STAKED_USER_POSITION", owner],
    queryFn: async (): Promise<Position> => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent: agent,
          canisterId: canister_id,
        });
        const actorIcpswap = Actor.createActor(idlFactoryIcpswap, {
          agent: agent,
          canisterId: ICPSWAP_CANISTER_ID,
        });
        const actorLedgerGLDT = Actor.createActor(idlFactoryLedger, {
          agent: agent,
          canisterId: GLDT_LEDGER_CANISTER_ID,
        });

        const result_arr = await get_position(actorStake, owner);

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
            is_enable_withdrawing: true,
            total_withdrawable_amount: 0,
          } satisfies Position;

        const result = result_arr[0];

        const decimalsGLDT = await icrc1_decimals(actorLedgerGLDT);
        const priceGLDT = await get_token_price_usd(
          actorIcpswap, TOKEN_GLDT.canister_id, TOKEN_GLDT.name, { agent }
        );

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
              agent: agent,
              canisterId: token.canister_id,
            });
            const decimals = await icrc1_decimals(actorLedger);
            const fee = await icrc1_fee(actorLedger);

            const is_claimable =
              token_reward.amount > fee && token_reward.amount > 0n;

            const is_amount_below_fee =
              token_reward.amount <= fee && token_reward.amount > 0n;

            const price_usd = await get_token_price_usd(
              actorIcpswap, token.canister_id, token.name, { agent }
            );
            const amount = Number(token_reward.amount) / 10 ** decimals;

            return {
              ...res,
              is_selected: is_claimable,
              is_claimable,
              is_amount_below_fee,
              amount,
              amount_e8s: token_reward.amount,
              amount_usd: price_usd * amount,
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
            amount_usd: priceGLDT * dissolved_amount,
            dissolved_date,
            is_withdrawable: true, //dissolved_date <= date_now
            remaining_time: Math.max(dissolved_date - date_now, 0),
          };
        });

        const total_withdrawable_amount = dissolve_events.reduce(
          (acc, curr) => acc + curr.amount,
          0
        );

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

        // const is_enable_withdrawing = dissolve_events.some(
        //   (event) => event.is_withdrawable
        // );

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
          staked_amount_usd: priceGLDT * staked_amount,
          total_rewards_amount,
          total_rewards_amount_e8s,
          total_rewards_amount_usd,
          is_enable_claiming_rewards,
          dissolve_events,
          dissolve_events_count,
          remaining_dissolve_events:
            MAX_DISSOLVE_EVENTS - dissolve_events_count,
          max_dissolve_events: MAX_DISSOLVE_EVENTS,
          is_enable_withdrawing: true,
          total_withdrawable_amount,
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
