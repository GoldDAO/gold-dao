import { useMutation } from "@tanstack/react-query";
import { Buffer } from "buffer";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { useQueryClient } from "@tanstack/react-query";

import { idlFactory } from "../idlFactory";

import { Response } from "../interfaces";

type RewardArgs = { token: string; neuron_id: string };
type RewardsArgs = { token: string; neuron_ids: string[] };

const claim_reward = async (
  actor: ActorSubclass,
  args: RewardArgs
): Promise<boolean> => {
  const { neuron_id, token } = args;

  const result = (await actor.claim_reward({
    token: token,
    neuron_id: { id: [...Uint8Array.from(Buffer.from(neuron_id, "hex"))] },
  })) as Response;

  if ("Ok" in result) {
    return result.Ok;
  } else {
    if ("NeuronHotKeyAbsent" in result) {
      throw new Error("NeuronHotKeyAbsent");
    } else if ("TokenSymbolInvalid" in result) {
      throw new Error(`TokenSymbolInvalid! ${result.TokenSymbolInvalid}`);
    } else if ("NeuronNotClaimed" in result) {
      throw new Error("NeuronNotClaimed");
    } else if ("NeuronOwnerInvalid" in result) {
      throw new Error("NeuronOwnerInvalid");
    } else if ("NeuronHotKeyInvalid" in result) {
      throw new Error("NeuronHotKeyInvalid");
    } else if ("TransferFailed" in result) {
      throw new Error(`TransferFailed! ${result.TransferFailed}`);
    } else if ("NeuronDoesNotExist" in result) {
      throw "NeuronDoesNotExist";
    } else if ("InternalError" in result) {
      throw new Error(`InternalError! ${result.InternalError}`);
    }
    throw new Error("Unexpected error");
  }
};

const useClaimReward = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ neuron_ids, token }: RewardsArgs) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        await Promise.all(
          neuron_ids.map(async (neuron_id) => {
            return claim_reward(actor, {
              neuron_id,
              token,
            });
          })
        );
      } catch (err) {
        console.error(err);
        throw new Error(`claim_reward error! Please retry later.`);
      }
    },
    onError: (err) => {
      console.log("claim error");
      console.log(err);
    },
    onSuccess: (res) => {
      console.log("claim success");
      console.log(res);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: ["USER_NEURONS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_NEURON_REWARDS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["USER_NEURONS_REWARDS"],
      });
    },
  });
};

export default useClaimReward;
