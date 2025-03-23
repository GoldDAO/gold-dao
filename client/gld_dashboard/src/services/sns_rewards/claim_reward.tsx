import { ActorSubclass } from "@dfinity/agent";
import { Buffer } from "buffer";

const claim_reward = async ({
  neuronId,
  token,
  actor,
}: {
  neuronId: string;
  token: string;
  actor: ActorSubclass;
}) => {
  const result = await actor.claim_reward({
    token: token,
    neuron_id: { id: [...Uint8Array.from(Buffer.from(neuronId, "hex"))] },
  });
  return result;
};

export default claim_reward;
