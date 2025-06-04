import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { SwapInfo } from "../interfaces";
import { getSwapData, SwapData } from "../utils/index";

const get_active_swaps_by_user = async (
  actor: ActorSubclass,
  principal: string
): Promise<SwapData[]> => {
  const data = (await actor.get_active_swaps_by_user([
    Principal.fromText(principal),
  ])) as Array<[[bigint, bigint], SwapInfo]>;
  const result = data.map((r) => getSwapData(r[1]));
  return result;
};

export default get_active_swaps_by_user;
