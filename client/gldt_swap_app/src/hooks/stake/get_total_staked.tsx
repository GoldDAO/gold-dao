import { ActorSubclass } from "@dfinity/agent";

const get_total_staked = async (actor: ActorSubclass): Promise<bigint> => {
  const result = (await actor.get_total_staked(null)) as bigint;
  return result;
};

export default get_total_staked;
