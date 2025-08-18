import { ActorSubclass } from "@dfinity/agent";

import { StakePositionResponse } from "@services/gldt_stake/interfaces/idlFactory";

const get_position = async (
  actor: ActorSubclass
): Promise<StakePositionResponse[]> => {
  const result = (await actor.get_position([])) as StakePositionResponse[];
  return result;
};

export default get_position;
