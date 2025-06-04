import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { Result_4 } from "@services/gldt_swap/interfaces";

const swap_nft_for_tokens = async (
  actor: ActorSubclass,
  nfts: { nft_id: bigint; nft_canister_id: Principal }
) => {
  const result = (await actor.swap_tokens_for_nft(nfts)) as Result_4;
  if ("Err" in result) {
    const errKey = Object.keys(result.Err)[0];
    throw new Error(errKey, { cause: result.Err });
  }
  return result.Ok;
};

export default swap_nft_for_tokens;
