import { ActorSubclass } from "@dfinity/agent";

const icrc7_transfer_fee = async (
  actor: ActorSubclass,
  id: bigint
): Promise<bigint> => {
  const result = (await actor.icrc7_transfer_fee(id)) as [bigint];
  return result[0];
};

export default icrc7_transfer_fee;
