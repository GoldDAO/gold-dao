import { ActorSubclass } from "@dfinity/agent";

import {
  Result,
  StakePositionResponse,
  Args,
} from "@services/gldt_stake/interfaces";

const claim_reward = async (
  actor: ActorSubclass,
  args: Args
): Promise<StakePositionResponse> => {
  const { id, token } = args;

  const result = (await actor.claim_reward({ id, token })) as Result;

  if ("Err" in result) throw result.Err;

  return result.Ok;
};

export default claim_reward;
