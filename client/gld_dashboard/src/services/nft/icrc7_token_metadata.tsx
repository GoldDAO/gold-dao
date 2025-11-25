import { ActorSubclass } from "@dfinity/agent";
import { ICRC3Value } from "./interfaces";

const icrc7_token_metadata = async (
  actor: ActorSubclass,
  options: { token_id: bigint }
): Promise<[] | [Array<[string, ICRC3Value]>]> => {
  const { token_id } = options;
  const result = (await actor.icrc7_token_metadata([token_id])) as Array<
    [] | [Array<[string, ICRC3Value]>]
  >;
  return result[0];
};

export default icrc7_token_metadata;
