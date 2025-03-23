import { ActorSubclass } from "@dfinity/agent";

const icrc1_fee = async (actor: ActorSubclass): Promise<bigint> => {
  const result = (await actor.icrc1_fee()) as bigint;
  return result;
};

export default icrc1_fee;
