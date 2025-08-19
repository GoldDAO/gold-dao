import { ActorSubclass } from "@dfinity/agent";
import { SwapAmountsResult } from "./interfaces";

const swap_amounts = async (
  actor: ActorSubclass,
  options: { from: string; to: string; amount: bigint }
) => {
  const { from, to, amount } = options;
  const result = (await actor.swap_amounts(
    from,
    amount,
    to
  )) as SwapAmountsResult;

  if ("Err" in result) throw new Error(result.Err);
  return result.Ok;
};

export default swap_amounts;
