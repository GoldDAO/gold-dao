import { ActorSubclass } from "@dfinity/agent";

const get_apy_overall = async (actor: ActorSubclass): Promise<number> => {
  const result = (await actor.get_apy_overall(null)) as number;
  return result;
};

export default get_apy_overall;
