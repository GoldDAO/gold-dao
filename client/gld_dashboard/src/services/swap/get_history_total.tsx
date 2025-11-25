import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

const get_history_total = async (
  actor: ActorSubclass,
  principal: string
): Promise<number> => {
  const data = (await actor.get_history_total([
    Principal.fromText(principal),
  ])) as bigint;
  const result = Number(data);
  return result;
};

export default get_history_total;
