import { ActorSubclass } from "@dfinity/agent";
import instance from "./icp_sns_api/instance";
import get_nervous_system_parameters from "./get_nervous_system_parameters";
import { parseNeuronData, NeuronData } from "./utils/parseNeuronData";

const fetch_one_neuron = async ({
  canisterId,
  actor,
  neuronId,
}: {
  canisterId: string;
  actor: ActorSubclass;
  neuronId: string;
}): Promise<NeuronData> => {
  const { data: neuron } = await instance.get(
    `/snses/${canisterId}/neurons/${neuronId}`
  );
  const nervousSystemParameters = await get_nervous_system_parameters({
    actor,
  });
  const data = parseNeuronData(neuron, nervousSystemParameters);

  return data;
};

export type { NeuronData };

export default fetch_one_neuron;
