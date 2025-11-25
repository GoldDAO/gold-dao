import { ActorSubclass } from "@dfinity/agent";
import type { Args, Response } from "./interfaces";

const get_blocks = async (
  actor: ActorSubclass,
  options: Args
): Promise<Response> => {
  const result = await actor.get_blocks(options);
  return result as Response;
};

export default get_blocks;
