import { useMutation } from "@tanstack/react-query";
import { DateTime } from "luxon";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../idlFactory";
import { Result_2 } from "../interfaces/ledger";

const icrc2_approve = async (
  actor: ActorSubclass,
  approveArgs: {
    amount: bigint;
    spender: { owner: string; subaccount?: Uint8Array<ArrayBufferLike> | [] };
  }
): Promise<bigint> => {
  const { amount, spender } = approveArgs;
  const decodedAccount = decodeIcrcAccount(spender.owner);
  const owner = decodedAccount.owner;
  let subaccount = [];
  if (spender.subaccount) {
    subaccount = [spender.subaccount];
  } else {
    subaccount = decodedAccount?.subaccount ? [decodedAccount.subaccount] : [];
  }

  const result = (await actor.icrc2_approve({
    amount,
    fee: [],
    memo: [],
    expected_allowance: [],
    created_at_time: [],
    expires_at: [
      BigInt(DateTime.now().plus({ hours: 1 }).toMillis()) * BigInt(1_000_000),
    ],
    spender: {
      owner,
      subaccount,
    },
    from_subaccount: [],
  })) as Result_2;

  if ("Err" in result) throw result.Err;
  return result.Ok;
};

const useApprove = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  return useMutation({
    mutationFn: async ({
      amount,
      spender,
    }: {
      amount: bigint;
      spender: { owner: string; subaccount?: Uint8Array<ArrayBufferLike> | [] };
    }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const icrc2Approve = await icrc2_approve(actor, {
          amount,
          spender,
        });
        return icrc2Approve;
      } catch (err) {
        console.error(err);
        throw new Error(`icrc2_approve error! Please retry later.`);
      }
    },
  });
};

export default useApprove;
