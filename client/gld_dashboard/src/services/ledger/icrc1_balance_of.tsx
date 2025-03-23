import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { Buffer } from "buffer";

export const icrc1_balance_of = async ({
  actor,
  owner,
  subaccount,
}: {
  actor: ActorSubclass;
  owner: string;
  subaccount?: string;
}): Promise<bigint> => {
  const _subaccount = subaccount
    ? [[...Uint8Array.from(Buffer.from(subaccount, "hex"))]]
    : [];

  const result = (await actor.icrc1_balance_of({
    owner: Principal.fromText(owner),
    subaccount: _subaccount,
  })) as bigint;

  const data = result;

  return data;
};
