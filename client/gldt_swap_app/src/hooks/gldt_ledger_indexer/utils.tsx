import { Principal } from "@dfinity/principal";
import { Buffer } from "buffer";
import {
  decodeIcrcAccount,
  encodeIcrcAccount,
  IcrcSubaccount,
} from "@dfinity/ledger-icrc";
import { ActorSubclass } from "@dfinity/agent";

import {
  GetTransactionsResult,
  GetTransactions,
  Account,
} from "@canisters/gldt_ledger_indexer/interface.ts";

import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";
import { getDateUTC } from "@utils/dates";

export interface TxAccount {
  owner: string | undefined;
  subaccount: string | undefined;
  full: string;
}

export interface GetAccountTransactionsParams {
  pageSize: number;
  start?: number;
  owner: string;
  subaccount?: string;
}

export interface Transaction {
  amount?: string;
  from?: TxAccount;
  to?: TxAccount;
  date?: string;
  type?: string;
  index?: number;
  memo?: string;
  fee?: number;
}

const getAccountText = ({
  from,
  to,
  kind,
  account,
}: {
  from: Account;
  to: Account;
  kind: string;
  account: string;
}) => {
  const isMint = account === "from" && kind === "mint";
  const isBurn = account === "to" && kind === "burn";
  const isMintingAccount = isMint || isBurn;

  if (
    !isMintingAccount &&
    ((!to && account === "to") || (!from && account === "from"))
  )
    return undefined;
  if ((isBurn && !from) || (isMint && !to)) return undefined;

  let owner,
    subaccount = undefined;

  if (isBurn) {
    owner = from.owner;
    subaccount = from.subaccount?.[0] ?? undefined;
  } else if (isMint) {
    owner = to.owner;
    subaccount = to.subaccount?.[0] ?? undefined;
  } else {
    if (account === "from") {
      owner = from.owner;
      subaccount = from.subaccount?.[0] ?? undefined;
    } else {
      //   console.log(to);
      owner = to.owner;
      subaccount = to.subaccount?.[0] ?? undefined;
    }
  }

  const _owner = owner ? Principal.from(owner).toText() : "";
  const _subaccount = subaccount
    ? Buffer.from(subaccount as Uint8Array).toString("hex")
    : undefined;

  return {
    owner: _owner,
    subaccount: _subaccount ?? "",
    full: isMintingAccount
      ? "Minting account"
      : encodeIcrcAccount({
          owner: Principal.fromText(_owner),
          subaccount: _subaccount
            ? ([
                ...Uint8Array.from(Buffer.from(_subaccount, "hex")),
              ] as IcrcSubaccount)
            : undefined,
        }),
  };
};

export const getAccountTransactions = async ({
  actor,
  pageSize,
  owner,
  subaccount,
  start,
}: GetAccountTransactionsParams & {
  actor: ActorSubclass;
}): Promise<GetTransactions> => {
  const account = encodeIcrcAccount({
    owner: Principal.fromText(owner),
    subaccount: subaccount
      ? [...Uint8Array.from(Buffer.from(subaccount, "hex"))]
      : [],
  });

  const decodedAccount = decodeIcrcAccount(account);
  const _owner = decodedAccount.owner;
  const _subaccount = decodedAccount?.subaccount
    ? [decodedAccount.subaccount]
    : [];

  const results = (await actor.get_account_transactions({
    max_results: BigInt(pageSize),
    start: start ? [BigInt(start)] : [],
    account: {
      owner: _owner,
      subaccount: _subaccount,
    },
  })) as GetTransactionsResult;

  if ("Err" in results) {
    console.error(results.Err.message);
    throw new Error(results.Err.message);
  }
  return results?.Ok;
};

export const formatTransactionsResults = (results: GetTransactions) => {
  return results.transactions.map((tx) => {
    const type = tx.transaction.kind;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const txData = (tx.transaction as any)[type][0];
    // console.log(tx);

    const amount = roundAndFormatLocale({
      number: divideBy1e8(txData.amount),
    });
    const date = getDateUTC(Number(tx.transaction.timestamp), {
      fromNanos: true,
    });
    const from = getAccountText({
      from: txData.from,
      to: txData.to,
      kind: type,
      account: "from",
    });
    const to =
      type === "approve"
        ? getAccountText({
            from: txData.from,
            to: txData.spender,
            kind: type,
            account: "to",
          })
        : getAccountText({
            from: txData.from,
            to: txData.to,
            kind: type,
            account: "to",
          });
    const fee = txData.fee && txData.fee[0] ? Number(txData.fee[0]) : undefined;
    const memo =
      txData.memo && txData.memo[0]
        ? Buffer.from(txData.memo[0] as Uint8Array).toString()
        : "-";

    const index = tx.id
      ? Number(tx.id)
      : from?.full === "Minting account"
      ? 0
      : undefined;

    return {
      index,
      amount,
      date,
      from,
      to,
      type,
      fee,
      memo,
    };
  });
};
