import { ActorSubclass } from "@dfinity/agent";

const icrc1_decimals = async (actor: ActorSubclass): Promise<number> => {
  const result = (await actor.icrc1_decimals()) as number;
  return result;
};

export default icrc1_decimals;
