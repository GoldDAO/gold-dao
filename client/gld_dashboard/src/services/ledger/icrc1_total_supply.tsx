import { ActorSubclass } from "@dfinity/agent";

const icrc1_total_supply = async (actor: ActorSubclass): Promise<bigint> => {
  const result = (await actor.icrc1_total_supply()) as bigint;
  return result;
};

export default icrc1_total_supply;
