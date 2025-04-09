import { ActorSubclass } from "@dfinity/agent";
import { SwapAmountsResult } from "./interfaces";

const swap_amounts = async (
  actor: ActorSubclass,
  options: { from: string; to: string; amount: number }
) => {
  const { from, to, amount } = options;
  const result = (await actor.swap_amounts(
    from,
    BigInt(Math.round(amount * 1e8)),
    to
  )) as SwapAmountsResult;

  if ("Err" in result) throw new Error(result.Err);

  return result.Ok;
};

export default swap_amounts;
