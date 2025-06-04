import { ActorSubclass } from "@dfinity/agent";

const get_nat_as_token_id_origyn = async (
  actor: ActorSubclass,
  options: { tokenId: bigint }
) => {
  const { tokenId } = options;
  const result = (await actor.get_nat_as_token_id_origyn(tokenId)) as string;
  return result;
};

export default get_nat_as_token_id_origyn;
