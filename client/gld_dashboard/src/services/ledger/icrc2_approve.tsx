import { DateTime } from "luxon";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { ActorSubclass } from "@dfinity/agent";
import { Result_2 } from "@services/ledger/interfaces/ledger";

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
  if (spender.subaccount) subaccount = [spender.subaccount];
  else
    subaccount = decodedAccount?.subaccount ? [decodedAccount.subaccount] : [];

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

export default icrc2_approve;
