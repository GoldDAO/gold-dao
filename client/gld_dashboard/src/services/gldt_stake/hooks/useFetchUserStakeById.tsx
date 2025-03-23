import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { DateTime } from "luxon";

import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";

import { DissolveState, Stake } from "../utils/interfaces";
import { StakePositionResponse } from "@services/gldt_stake/interfaces";

const useFetchUserStakeById = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<Stake, Error>, "queryKey" | "queryFn"> & {
    id: bigint;
    fee: bigint;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    id,
    fee,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["USER_STAKE_FETCH_ONE", id.toString()],
    queryFn: async (): Promise<Stake> => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const result = (await actor.get_position_by_id(
          id
        )) as StakePositionResponse[];

        const stake = result[0];
        const dissolve_state = Object.keys(stake.dissolve_state)[0];

        const get_dissolve_state = (state: DissolveState) => {
          switch (state) {
            case "NotDissolving":
              return "Not dissolving";
            case "Dissolving":
              return "Dissolving";
            case "Dissolved":
              return "Dissolved";
          }
        };

        const is_dissolved = dissolve_state === "Dissolved";

        const is_unstakable =
          dissolve_state === "Dissolving" &&
          DateTime.now() > DateTime.fromMillis(Number(stake.dissolved_date));

        const claimable_rewards_list = stake.claimable_rewards.map(
          ([name, amount]) => {
            return {
              name: name,
              amount: amount,
              is_claimable: amount >= fee,
            };
          }
        );

        const claimable_rewards_total_amount = claimable_rewards_list.reduce(
          (acc, { amount }) => acc + amount,
          0n
        );

        return {
          is_dissolved,
          claimable_rewards: !is_dissolved
            ? {
                list: claimable_rewards_list,
                total_amount: claimable_rewards_total_amount,
              }
            : { list: [], total_amount: 0n },
          created_at: stake.created_at,
          id: stake.id,
          unstake_early_fee: stake.early_unstake_fee,
          is_unstakable,
          is_unlockable: dissolve_state === "NotDissolving",
          dissolve_state: get_dissolve_state(dissolve_state as DissolveState),
          age_bonus_multiplier: stake.age_bonus_multiplier,
          amount: stake.staked,
        };
      } catch (err) {
        console.log(err);
        throw new Error("Fetch one user stake error! Please retry later.");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useFetchUserStakeById;
