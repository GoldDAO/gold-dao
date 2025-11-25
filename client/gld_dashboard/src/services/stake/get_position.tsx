import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

import { StakePositionResponse } from "@services/stake/idlFactory";

const get_position = async (
  actor: ActorSubclass,
  owner: string
): Promise<StakePositionResponse[]> => {
  const result = (await actor.get_position(
    Principal.fromText(owner)
  )) as StakePositionResponse[];
  return result;
};

export default get_position;
