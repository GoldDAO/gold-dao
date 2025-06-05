import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { Result_3 } from "@services/gldt_swap/interfaces";

const swap_nft_for_tokens = async (
  actor: ActorSubclass,
  nfts: [bigint, Principal][]
) => {
  const result = (await actor.swap_nft_for_tokens(nfts)) as Result_3;
  if ("Err" in result) {
    const errKey = Object.keys(result.Err)[0];
    throw new Error(errKey, { cause: result.Err });
  }
  return result.Ok;
};

export default swap_nft_for_tokens;
