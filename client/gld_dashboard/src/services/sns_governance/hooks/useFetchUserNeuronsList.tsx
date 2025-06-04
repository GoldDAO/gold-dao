import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../idlFactory";
import { NeuronUser } from "../utils/interfaces";
import list_neurons from "../list_neurons";
import { formatRoundedTimeUnits, getRoundedYears } from "@shared/utils/dates";

const useFetchUserNeuronsList = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  args: Omit<UseQueryOptions<NeuronUser[]>, "queryKey" | "queryFn"> & {
    limit?: number;
    owner: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    limit = 20,
    owner,
  } = args;

  return useQuery({
    queryKey: ["USER_NEURONS", owner],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const results = await list_neurons(actor, {
          limit,
          start_page_at: null,
          owner,
        });

        const data = results.map((neuron) => {
          const getDissolveDelay = (dissolve_delay: number | undefined) => {
            if (dissolve_delay) {
              if (getRoundedYears(dissolve_delay) > 0) {
                return formatRoundedTimeUnits(dissolve_delay);
              }
              return formatRoundedTimeUnits(dissolve_delay, {
                startFromUnit: "days",
                includeMinutesWithHours: true,
              });
            }
            return "-";
          };

          const getAge = (age: number | undefined) => {
            if (age) {
              if (getRoundedYears(age) > 0) {
                return formatRoundedTimeUnits(age);
              }
              return formatRoundedTimeUnits(age, {
                startFromUnit: "days",
                includeMinutesWithHours: true,
              });
            }
            return "-";
          };

          return {
            id: neuron.id,
            staked_amount: neuron.staked_amount,
            state: neuron.state,
            dissolve_delay: getDissolveDelay(neuron.dissolve_delay),
            age: getAge(neuron.age),
          };
        });
        return data;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch user neurons error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchUserNeuronsList;
