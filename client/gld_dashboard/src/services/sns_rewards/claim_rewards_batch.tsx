import { ActorSubclass } from "@dfinity/agent";
import { NeuronId, Result } from "./interfaces";

const claim_rewards_batch = async (
  actor: ActorSubclass,
  claim_reward_args: { neuron_id: NeuronId; token: string }[]
) => {
  const result = (await actor.claim_rewards_batch({
    claim_reward_args: claim_reward_args.map(({ neuron_id, token }) => ({
      token: { [token]: null },
      neuron_id,
    })),
  })) as Result;
  if ("Ok" in result) {
    return result.Ok;
  } else {
    const errors = result.Err.map((claimError) => {
      const { neuron_id, token, error } = claimError;

      if ("TokenSymbolInvalid" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): TokenSymbolInvalid - ${error.TokenSymbolInvalid}`;
      } else if ("NeuronNotClaimed" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): NeuronNotClaimed`;
      } else if ("NeuronOwnerInvalid" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): NeuronOwnerInvalid - ${error.NeuronOwnerInvalid}`;
      } else if ("NeuronHotKeyInvalid" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): NeuronHotKeyInvalid`;
      } else if ("TransferFailed" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): TransferFailed - ${error.TransferFailed}`;
      } else if ("NeuronDoesNotExist" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): NeuronDoesNotExist`;
      } else if ("InternalError" in error) {
        return `Neuron ${neuron_id} (Token: ${token}): InternalError - ${error.InternalError}`;
      }
      return `Neuron ${neuron_id} (Token: ${token}): Unknown error`;
    });

    throw new Error(`Batch claim failed: ${errors.join(", ")}`);
  }
};

export default claim_rewards_batch;
