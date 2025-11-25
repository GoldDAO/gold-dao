import { ActorSubclass } from "@dfinity/agent";
import {
  Nft,
  Result_2,
  SwapNftForTokensErrors,
  GeneralError,
} from "@services/swap/interfaces";

const parseError = (
  error: SwapNftForTokensErrors
): {
  type: string;
  message?: string;
} => {
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
  if ("CantBeAnonymous" in error) {
    return { type: "CantBeAnonymous", message: error.CantBeAnonymous };
  }
  return { type: "UnknownError" };
};

const swap_nft_for_tokens = async (actor: ActorSubclass, nfts: Nft[]) => {
  const result = (await actor.swap_nft_for_tokens(nfts)) as Result_2;
  if ("Err" in result) {
    const parsed = parseError(result.Err);
    throw new Error(parsed.type, { cause: parsed });
  }
  return result.Ok;
};

export default swap_nft_for_tokens;
