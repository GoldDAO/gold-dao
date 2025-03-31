import { ActorSubclass } from "@dfinity/agent";
// import { encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Buffer } from "buffer";
import { Principal } from "@dfinity/principal";
import { Subaccount } from "@dfinity/ledger-icrc/dist/candid/icrc_ledger";

import { GetTransactionsResult } from "./idlFactory.interface";
import { Transactions } from "./utils/interfaces";
import { getDateUTC } from "@utils/dates";

const get_account_transactions = async (
  actor: ActorSubclass,
  args: {
    max_results: number;
    start: number | null;
    owner: Principal;
    subaccount: Subaccount[];
  }
): Promise<Transactions> => {
  const { max_results, start, owner, subaccount } = args;

  const results = (await actor.get_account_transactions({
    account: { owner, subaccount },
    max_results: BigInt(max_results),
    start: start ? [BigInt(start)] : [],
  })) as GetTransactionsResult;

  if ("Err" in results) throw results.Err;

  const data = results.Ok.transactions?.map((transaction) => {
    const tx = transaction?.transaction;
    const kind = tx?.kind;
    const index = transaction?.id;
    const timestamp = transaction?.transaction?.timestamp;

    let from: string | undefined;
    let to: string | undefined;
    let amount: bigint | undefined = undefined;
    let fee: bigint | undefined = undefined;
    let memo: [] | [number[] | Uint8Array<ArrayBufferLike>] | undefined;

    if (kind === "transfer") {
      const transfer = tx?.transfer?.[0];
      from = transfer?.from?.owner.toText() ?? undefined;
      to = transfer?.to?.owner.toText() ?? undefined;
      amount = transfer?.amount;
      fee = transfer?.fee?.[0];
      memo = transfer?.memo;
    }
    if (kind === "mint") {
      const mint = tx?.mint?.[0];
      to = mint?.to?.owner.toText() ?? undefined;
      amount = mint?.amount;
      memo = mint?.memo;
    } else if (kind === "burn") {
      const burn = tx?.burn?.[0];
      from = burn?.from?.owner.toText() ?? undefined;
      amount = burn?.amount;
      memo = burn?.memo;
    } else if (kind === "approve") {
      const approve = tx?.approve?.[0];
      from = approve?.from?.owner.toText() ?? undefined;
      amount = approve?.amount;
      fee = approve?.fee?.[0];
      memo = approve?.memo;
    }

    return {
      index: Number(index),
      timestamp: timestamp
        ? getDateUTC(timestamp.toString(), { fromNanos: true })
        : "-",
      from: kind === "mint" ? "Minting account" : from,
      to: kind === "burn" ? "Minting account" : to,
      amount: amount ?? undefined,
      fee: fee ?? undefined,
      memo: memo?.[0] ? Buffer.from(memo[0]).toString("utf-8") : undefined,
      kind,
    };
  });

  return {
    data,
    cursor_index: data[data.length - 1] ? data[data.length - 1].index : null,
  };
};

export default get_account_transactions;
