import { ActorSubclass } from "@dfinity/agent";
import instance from "./icp_sns_api/instance";
import { Neuron } from "./interfaces";
import get_nervous_system_parameters from "./get_nervous_system_parameters";
import { parseNeuronData, NeuronData } from "./utils/parseNeuronData";

interface Neurons {
  data: Neuron[];
  total_neurons: number;
  max_neuron_index: number;
}

const fetch_all_neurons = async ({
  limit = 20,
  offset = 0,
  sort_by = "-created_timestamp_seconds",
  canisterId,
  actor,
}: {
  limit?: number;
  offset?: number;
  sort_by?: string;
  canisterId: string;
  actor: ActorSubclass;
}): Promise<{
  data: NeuronData[];
  total_neurons: number;
  max_neuron_index: number;
}> => {
  const { data: neurons } = (await instance.get(
    `/snses/${canisterId}/neurons?limit=${limit}&offset=${offset}${
      sort_by ? `&sort_by=${sort_by}` : ""
    }`
  )) as { data: Neurons };

  const nervousSystemParameters = await get_nervous_system_parameters({
    actor,
  });

  const data =
    neurons?.data?.map((neuron) => {
      return parseNeuronData(neuron, nervousSystemParameters);
    }) ?? [];

  return {
    data,
    total_neurons: neurons?.total_neurons ?? 0,
    max_neuron_index: neurons?.max_neuron_index ?? 0,
  };
};

export default fetch_all_neurons;
