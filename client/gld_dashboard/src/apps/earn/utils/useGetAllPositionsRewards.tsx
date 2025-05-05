import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { GLDT_STAKE_CANISTER_ID, KONGSWAP_CANISTER_ID_IC } from "@constants";
import { idlFactory as idlFactoryStake } from "@services/gldt_stake/idlFactory";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongswap } from "@services/kongswap/idlFactory";
import get_active_user_positions from "@services/gldt_stake/get_active_user_positions";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import swap_amounts from "@services/kongswap/swap_amounts";
import { PositionRewards, TokenRewardsList } from "./index";

const useGetAllPositionsStakedRewards = (
  options: Omit<
    UseQueryOptions<PositionRewards[], Error>,
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
    queryKey: ["USER_POSITIONS_REWARDS", owner],
    queryFn: async (): Promise<PositionRewards[]> => {
      try {
        const actorStake = Actor.createActor(idlFactoryStake, {
          agent,
          canisterId: GLDT_STAKE_CANISTER_ID,
        });

        const actorKongswap = Actor.createActor(idlFactoryKongswap, {
          agent,
          canisterId: KONGSWAP_CANISTER_ID_IC,
        });

        const positions = await get_active_user_positions(actorStake);

        const data = await Promise.all(
          TokenRewardsList.map(async (token) => {
            const actorLedger = Actor.createActor(idlFactoryLedger, {
              agent,
              canisterId: token.canisterId,
            });
            const decimals = await icrc1_decimals(actorLedger);

            const res = await Promise.all(
              positions.map(async (position) => {
                const filtered = position.claimable_rewards.filter(
                  ([name]) => name === token.name
                );

                const positionRewards = await Promise.all(
                  filtered.map(async ([, amount]) => {
                    const price = await swap_amounts(actorKongswap, {
                      from: token.name,
                      to: "ckUSDC",
                      amount,
                    });

                    return {
                      id: position.id,
                      amount,
                      amount_usd:
                        price.mid_price * (Number(amount) / 10 ** decimals),
                    };
                  })
                );

                return positionRewards;
              })
            );
            return {
              name: token.name,
              positions: res.flatMap((item) => item),
              amount: res
                .flatMap((item) => item)
                .reduce((acc, curr) => acc + curr.amount, 0n),
              amount_usd: res
                .flatMap((item) => item)
                .reduce((acc, curr) => acc + curr.amount_usd, 0),
            };
          })
        );
        return data;
      } catch (err) {
        console.log(err);
        throw new Error(
          "Fetch all stake position rewards error! Please retry later."
        );
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
  });
};

export default useGetAllPositionsStakedRewards;
