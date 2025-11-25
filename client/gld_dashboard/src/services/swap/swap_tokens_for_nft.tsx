import { ActorSubclass } from "@dfinity/agent";
import {
  Nft,
  Result_3,
  SwapTokensForNftErrors,
  GeneralError,
} from "@services/swap/interfaces";

function parseError(error: SwapTokensForNftErrors): {
  type: string;
  message?: string;
} {
  if ("Limit" in error) {
    return { type: "Limit", message: error.Limit };
  }
  if ("GeneralError" in error) {
    const general = error.GeneralError as GeneralError;
    const key = Object.keys(general)[0] as keyof GeneralError;
    const value = general[key];
    return { type: `GeneralError:${key}`, message: value };
  }
  if ("Retry" in error) {
    const [code, msg] = error.Retry;
    return { type: "Retry", message: `Code: ${code}, Message: ${msg}` };
  }
  if ("NotOwnedBySwapCanister" in error) {
    return { type: "NotOwnedBySwapCanister" };
  }
  if ("SwapCreationError" in error) {
    return { type: "SwapCreationError" };
  }
  return { type: "UnknownError" };
}

const swap_tokens_for_nft = async (actor: ActorSubclass, nfts: Nft[]) => {
  const result = (await actor.swap_tokens_for_nft(nfts)) as Result_3;
  if ("Err" in result) {
    const parsed = parseError(result.Err);
    throw new Error(parsed.type, { cause: parsed });
  }
  return result.Ok;
};

export default swap_tokens_for_nft;
