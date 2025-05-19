import { ActorSubclass } from "@dfinity/agent";
import { Buffer } from "buffer";
import { Principal } from "@dfinity/principal";
import { Subaccount } from "@dfinity/ledger-icrc/dist/candid/icrc_ledger";
import _upperFirst from "lodash/upperFirst";
import {
  GetAccountIdentifierTransactionsResult,
  TransactionWithId,
} from "./idlFactory_icp.interface";
import { Transactions } from "./utils/interfaces";
import { getDateUTC } from "@utils/dates";
// import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";

const get_account_transactions_icp = async (
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
  })) as GetAccountIdentifierTransactionsResult;

  if ("Err" in results) throw results.Err;

  const data = results.Ok.transactions?.map(
    (transaction: TransactionWithId) => {
      const tx = transaction.transaction;
      const index = transaction.id;
      const timestamp = tx.timestamp?.[0]?.timestamp_nanos;
      const operation = tx.operation;

      // console.log(tx);

      let from: string | undefined;
      let to: string | undefined;
      let amount: bigint | undefined = undefined;
      let fee: bigint | undefined = undefined;
      let memo: string | undefined;
      let kind = "";

      if ("Transfer" in operation) {
        from = operation.Transfer.from;
        to = operation.Transfer.to;
        amount = operation.Transfer.amount.e8s;
        fee = operation.Transfer.fee.e8s;
        kind = "transfer";
      } else if ("Mint" in operation) {
        to = operation.Mint.to;
        amount = operation.Mint.amount.e8s;
        kind = "mint";
      } else if ("Burn" in operation) {
        from = operation.Burn.from;
        amount = operation.Burn.amount.e8s;
        kind = "burn";
      } else if ("Approve" in operation) {
        from = operation.Approve.from;
        kind = "approve";
        amount = operation.Approve.allowance.e8s;
      }

      if (tx.icrc1_memo?.[0]) {
        memo = Buffer.from(tx.icrc1_memo[0]).toString("utf-8");
      }

      return {
        index: Number(index),
        timestamp: timestamp
          ? getDateUTC(timestamp.toString(), { fromNanos: true })
          : "-",
        from: kind === "mint" ? "Minting account" : from,
        to: kind === "burn" ? "Minting account" : to,
        amount,
        fee,
        memo,
        kind: _upperFirst(kind),
        is_credit: false,
      };
    }
  );

  return {
    data,
    cursor_index: data[data.length - 1] ? data[data.length - 1].index : null,
  };
};

export default get_account_transactions_icp;
