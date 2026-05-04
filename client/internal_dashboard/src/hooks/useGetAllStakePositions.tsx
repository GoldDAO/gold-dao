import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
// import { DateTime } from "luxon";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  ICPSWAP_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID_IC,
  ICP_LEDGER_CANISTER_ID_IC,
  OGY_LEDGER_CANISTER_ID_IC,
} from "../constants";
import { idlFactory as idlFactoryStake } from "../services/gldt_stake/idlFactory";
import { idlFactory as idlFactoryLedger } from "../services/ledger/idlFactory";
import { idlFactory as idlFactoryIcpswap } from "../services/icpswap/idlFactory";
import get_all_stake_positions from "../services/gldt_stake/get_all_stake_positions";
import icrc1_fee from "../services/ledger/icrc1_fee";
import icrc1_decimals from "../services/ledger/icrc1_decimals";
import get_token_price_usd from "../services/icpswap/get_token_price_usd";

export interface Reward {
  canister_id: string;
  name: string;
  is_selected: boolean;
  is_claimable: boolean;
  amount: number;
  amount_e8s: bigint;
  amount_usd: number;
  is_amount_below_fee: boolean;
}

export interface DissolveEvent {
  amount: number;
  amount_e8s: bigint;
  amount_usd: number;
  dissolved_date: number;
  is_withdrawable: boolean;
  remaining_time: number;
}

export interface Position {
  principal: string;
  rewards: Reward[];
  created_at: bigint;
  age_bonus_multiplier: number;
  staked_amount: number;
  staked_amount_e8s: bigint;
  staked_amount_usd: number;
  total_rewards_amount: number;
  total_rewards_amount_e8s: bigint;
  total_rewards_amount_usd: number;
  is_enable_claiming_rewards: boolean;
  dissolve_events: DissolveEvent[];
  dissolve_events_count: number;
  is_enable_withdrawing: boolean;
}

const useGetAllStakePositions = (
  canister_id: string,
  options?: Omit<UseQueryOptions<Position[], Error>, "queryKey" | "queryFn">
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
    queryKey: ["FETCH_STAKED_USER_POSITION", canister_id],
    queryFn: async (): Promise<Position[]> => {
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
        const actorLedgerGLDT = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: GLDT_LEDGER_CANISTER_ID,
        });

        const result = await get_all_stake_positions(actorStake);

        const TOKENS = [
          {
            name: "GOLDAO",
            canister_id: GOLDAO_LEDGER_CANISTER_ID_IC,
          },
          { name: "ICP", canister_id: ICP_LEDGER_CANISTER_ID_IC },
          { name: "OGY", canister_id: OGY_LEDGER_CANISTER_ID_IC },
        ];

        if (!result.length) return [];

        const decimalsGLDT = await icrc1_decimals(actorLedgerGLDT);
        const priceGLDT = await get_token_price_usd(
          actorIcpswap, GLDT_LEDGER_CANISTER_ID, "GLDT", { agent }
        );

        const res = await Promise.all(
          result.map(async (position) => {
            const data = position[1];

            const claimable_rewards = data.claimable_rewards.map(
              ([tokenSymbol, amount]) => {
                const tokenName = Object.keys(tokenSymbol)[0];
                return {
                  name: tokenName,
                  amount: amount,
                };
              }
            );

            const rewards: Reward[] = await Promise.all(
              TOKENS.map(async (token) => {
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
                  agent,
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

            const dissolve_events = data.dissolve_events.map((event) => {
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

            const staked_amount = Number(data.staked) / 10 ** decimalsGLDT;
            const instant_dissolve_fee =
              Number(data.instant_dissolve_fee) / 10 ** decimalsGLDT;

            const principal = encodeIcrcAccount({
              owner: data.owned_by,
              subaccount: [],
            });

            const ret = {
              principal,
              rewards,
              created_at: data.created_at,
              age_bonus_multiplier: data.age_bonus_multiplier,
              staked_amount,
              staked_amount_e8s: data.staked,
              staked_amount_usd: priceGLDT * staked_amount,
              total_rewards_amount,
              total_rewards_amount_e8s,
              total_rewards_amount_usd,
              is_enable_claiming_rewards,
              dissolve_events,
              dissolve_events_count,
              is_enable_withdrawing,
            } satisfies Position;

            return ret;
          })
        );
        return res;
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

export default useGetAllStakePositions;
