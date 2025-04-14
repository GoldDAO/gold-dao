import { ActorSubclass } from "@dfinity/agent";
import { Buffer } from "buffer";
import { Principal } from "@dfinity/principal";
import { ListNeuronsResponse } from "./interfaces";
import get_nervous_system_parameters from "./get_nervous_system_parameters";
import { parseNeuronData } from "./utils/parseNeuronData";
import { NeuronData } from "./utils/interfaces";

const list_neurons = async (
  actor: ActorSubclass,
  args: {
    limit: number;
    start_page_at: string | null;
    owner: string;
  }
): Promise<NeuronData[]> => {
  const { limit, start_page_at, owner } = args;

  const result = (await actor.list_neurons({
    of_principal: owner ? [Principal.fromText(owner)] : [],
    limit: BigInt(limit),
    start_page_at: start_page_at
      ? [{ id: [...Uint8Array.from(Buffer.from(start_page_at, "hex"))] }]
      : [],
  })) as ListNeuronsResponse;

  const nervousSystemParameters = await get_nervous_system_parameters({
    actor,
  });

  const data =
    result.neurons?.map((neuron) => {
      return parseNeuronData(neuron, nervousSystemParameters);
    }) ?? [];

  return data.filter((neuron) => {
    return neuron.staked_amount > 0n && neuron.state !== "Dissolved";
  }) as NeuronData[];
};

export default list_neurons;
