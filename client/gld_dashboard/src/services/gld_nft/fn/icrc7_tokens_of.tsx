import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

const icrc7_tokens_of = async (
  actor: ActorSubclass,
  options: { owner: string; subaccount?: string[] }
) => {
  const { owner, subaccount = [] } = options;
  const result = (await actor.icrc7_tokens_of(
    {
      owner: Principal.fromText(owner),
      subaccount,
    },
    [],
    []
  )) as Array<bigint>;
  return result;
};

export default icrc7_tokens_of;
