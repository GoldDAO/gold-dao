import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { ListNeuronsResponse } from "./interfaces";
import get_nervous_system_parameters from "./get_nervous_system_parameters";
import { parseNeuronData, NeuronData } from "./utils/parseNeuronData";

const fetch_user_neurons = async ({
  actor,
  owner,
}: {
  actor: ActorSubclass;
  owner: string;
}): Promise<NeuronData[]> => {
  const result = (await actor.list_neurons({
    of_principal: owner ? [Principal.fromText(owner)] : [],
    limit: 0,
    start_page_at: [],
  })) as ListNeuronsResponse;

  const nervousSystemParameters = await get_nervous_system_parameters({
    actor,
  });

  const data =
    result.neurons?.map((neuron) => {
      return parseNeuronData(neuron, nervousSystemParameters);
    }) ?? [];

  return data;
};

export default fetch_user_neurons;
