import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";
import { idlFactory as idlFactoryLedger } from "../idlFactoryLedger";

import { NeuronUser } from "./interfaces";

import fetch_user_neurons from "../fetch_user_neurons";
import { icrc1_balance_of } from "../../ledger/icrc1_balance_of";

import { divideBy1e8, numberToLocaleString } from "@shared/utils/numbers";
import { calculateTimeDifferenceInSeconds } from "@shared/utils/dates";

const useFetchUserNeurons = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<NeuronUser[]>, "queryKey" | "queryFn"> & {
    owner: string;
    canisterIdSNSRewards: string;
    canisterIdLedgerGOLDAO: string;
    canisterIdLedgerOGY: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    owner,
    canisterIdSNSRewards,
    canisterIdLedgerGOLDAO,
    canisterIdLedgerOGY,
  } = options;
  return useQuery({
    queryKey: ["USER_FETCH_NEURONS"],
    queryFn: async () => {
      try {
        const actorGovernance = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const actorLedgerGOLDAO = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: canisterIdLedgerGOLDAO,
        });

        const actorLedgerOGY = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: canisterIdLedgerOGY,
        });

        const results = await fetch_user_neurons({
          owner,
          actor: actorGovernance,
        });

        const data = results.map((neuron) => {
          const isVotingPower =
            neuron.voting_power &&
            neuron.dissolve_delay &&
            neuron.neuron_minimum_dissolve_delay_to_vote_seconds &&
            calculateTimeDifferenceInSeconds(neuron.dissolve_delay as number) >
              neuron.neuron_minimum_dissolve_delay_to_vote_seconds
              ? true
              : false;

          return {
            id: neuron.id,
            created_at: neuron.created_at,
            auto_stake_maturity: neuron.auto_stake_maturity ? "True" : "False",
            dissolve_delay_bonus: isVotingPower
              ? `+${neuron.dissolve_delay_bonus}%`
              : "-",
            age_bonus: isVotingPower ? `+${neuron.age_bonus}%` : "-",
            total_bonus: isVotingPower ? `+${neuron.total_bonus}%` : "-",
          };
        });

        const claimBalanceGLDGov = await Promise.all(
          data.map((neuron) =>
            icrc1_balance_of({
              actor: actorLedgerGOLDAO,
              owner: canisterIdSNSRewards,
              subaccount: neuron.id,
            })
          )
        );

        const claimBalanceICP = await Promise.all(
          data.map((neuron) =>
            icrc1_balance_of({
              actor: actorLedgerGOLDAO,
              owner: canisterIdSNSRewards,
              subaccount: neuron.id,
            })
          )
        );

        const claimBalanceOGY = await Promise.all(
          data.map((neuron) =>
            icrc1_balance_of({
              actor: actorLedgerOGY,
              owner: canisterIdSNSRewards,
              subaccount: neuron.id,
            })
          )
        );

        const updatedData = data.map((neuron, index) => ({
          ...neuron,
          claim_balance_gldgov: numberToLocaleString({
            value: divideBy1e8(claimBalanceGLDGov[index]),
          }),
          claim_balance_icp: numberToLocaleString({
            value: divideBy1e8(claimBalanceICP[index]),
          }),
          claim_balance_ogy: numberToLocaleString({
            value: divideBy1e8(claimBalanceOGY[index]),
          }),
        }));

        return updatedData;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch user neurons error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchUserNeurons;
