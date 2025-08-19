import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryGLDTStake } from "../services/gldt_stake/idlFactory";
import { idlFactory as idlFactoryLedger } from "../services/ledger/idlFactory";
import get_neurons from "../services/gldt_stake/get_neurons";
import icrc1_decimals from "../services/ledger/icrc1_decimals";

const useGetNeuronSize = (
  canister_id_gldt_stake,
  canister_id_gldt_ledger,
  options = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  return useQuery({
    queryKey: ["GET_NEURONS", canister_id_gldt_stake],
    queryFn: async () => {
      try {
        const agent = await HttpAgent.create({ host: "https://ic0.app" });
        const actor = Actor.createActor(idlFactoryGLDTStake, {
          agent,
          canisterId: canister_id_gldt_stake,
        });
        const actorLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: canister_id_gldt_ledger,
        });

        const decimals = await icrc1_decimals(actorLedger);

        const neurons = await get_neurons(actor);

        const neuron_size_e8s = neurons.reduce((acc, neuron) => {
          return (
            acc +
            (neuron.cached_neuron_stake_e8s +
              neuron.staked_maturity_e8s_equivalent[0])
          );
        }, 0n);

        const neuron_size = Number(neuron_size_e8s) / 10 ** decimals;

        return neuron_size;
      } catch (err) {
        console.log(err);
        throw new Error("Fetch neuron size error");
      }
    },
    enabled,
    placeholderData,
    refetchInterval,
    staleTime,
    refetchOnMount,
    refetchOnWindowFocus,
    refetchOnReconnect,
    ...queryOptions,
  });
};

export default useGetNeuronSize;
