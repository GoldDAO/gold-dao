import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { GeneralError, SwapInfo } from "./interfaces";
import { SwapData } from "./utils/interfaces";
import { getSwapData } from "./utils/index";

const get_historic_swaps_by_user = async (
  actor: ActorSubclass,
  options: {
    page: number;
    principal: string;
    limit: number;
  }
): Promise<SwapData[]> => {
  const { page, principal, limit } = options;
  const result = (await actor.get_historic_swaps_by_user({
    page: page,
    user: Principal.fromText(principal),
    limit: limit,
  })) as { Ok: Array<[bigint, SwapInfo]> } | { Err: GeneralError };
  if ("Err" in result) {
    const errKey = Object.keys(result.Err)[0];
    throw new Error(errKey, { cause: result.Err });
  }
  const data = result.Ok.map((r) => getSwapData(r[1]));
  return data;
};

export default get_historic_swaps_by_user;
