import { ActorSubclass } from "@dfinity/agent";
import { SwapInfo } from "../interfaces";
import { getSwapData, SwapData } from "../utils/index";

const get_active_swaps = async (actor: ActorSubclass): Promise<SwapData[]> => {
  const data = (await actor.get_active_swaps(null)) as Array<
    [[bigint, bigint], SwapInfo]
  >;
  const result = data.map((r) => getSwapData(r[1]));
  return result;
};

export default get_active_swaps;
