import { ActorSubclass } from "@dfinity/agent";

import { StakePositionResponse } from "@services/gldt_stake/interfaces";

const get_active_user_positions = async (
  actor: ActorSubclass
): Promise<StakePositionResponse[]> => {
  const result = (await actor.get_active_user_positions(
    []
  )) as StakePositionResponse[];
  return result;
};

export default get_active_user_positions;
