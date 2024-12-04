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
  GetBlocksResponse,
  Map,
  Value,
  TransactionWithId,
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

export interface GetBlocksParams {
  pageSize: number;
  start?: number;
}

export interface Transaction {
  amount?: string;
  from?: TxAccount;
  to?: TxAccount;
  date?: string;
  type?: string;
  index?: number;
  memo?: string;
  fee?: string;
  hash?: string;
}

const getAccountTextTransactions = ({
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

const getAccountTextBlocks = ({
  from,
  to,
  kind,
  account,
}: {
  from: Value[];
  to: Value[];
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

  const _from = {
    owner: from && from[0] && "Blob" in from[0] ? from[0]?.Blob : "",
    subaccount:
      from && from[1] && "Blob" in from[1] ? from[1]?.Blob : undefined,
  };
  const _to = {
    owner: to && to[0] && "Blob" in to[0] ? to[0]?.Blob : "",
    subaccount: to && to[1] && "Blob" in to[1] ? to[1]?.Blob : undefined,
  };

  if (isBurn) {
    owner = _from.owner;
    subaccount = _from.subaccount;
  } else if (isMint) {
    owner = _to.owner;
    subaccount = _to.subaccount;
  } else {
    if (account === "from") {
      owner = _from.owner;
      subaccount = _from.subaccount;
    } else {
      owner = _to.owner;
      subaccount = _to.subaccount;
    }
  }

  const _owner = owner
    ? Principal.fromHex(Buffer.from(owner).toString("hex")).toText()
    : "";
  const _subaccount = subaccount
    ? Buffer.from(subaccount).toString("hex")
    : undefined;

  return {
    owner: _owner,
    subaccount: _subaccount ?? "",
    full: isMintingAccount
      ? "Minting account"
      : encodeIcrcAccount({
          owner: Principal.fromText(_owner),
          subaccount: _subaccount as IcrcSubaccount | undefined,
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

export const formatTransactionsResults = (
  results: Array<TransactionWithId>
) => {
  return results.map((tx) => {
    const type = tx.transaction.kind;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const txData = (tx.transaction as any)[type][0];
    // console.log(txData);

    const amount = roundAndFormatLocale({
      number: divideBy1e8(txData.amount),
    });
    const date = getDateUTC(Number(tx.transaction.timestamp), {
      fromNanos: true,
    });
    const from = getAccountTextTransactions({
      from: txData.from,
      to: txData.to,
      kind: type,
      account: "from",
    });
    const to =
      type === "approve"
        ? getAccountTextTransactions({
            from: txData.from,
            to: txData.spender,
            kind: type,
            account: "to",
          })
        : getAccountTextTransactions({
            from: txData.from,
            to: txData.to,
            kind: type,
            account: "to",
          });
    const fee =
      txData.fee && txData.fee[0]
        ? roundAndFormatLocale({
            number: divideBy1e8(Number(txData.fee[0])),
          })
        : undefined;
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

export const getBlocks = async ({
  actor,
  start,
  pageSize,
}: GetBlocksParams & {
  actor: ActorSubclass;
}): Promise<Transaction[]> => {
  const results = (await actor.get_blocks({
    start: start,
    length: BigInt(pageSize),
  })) as GetBlocksResponse;
  const data = results.blocks.map((block) => {
    if ("Map" in block) {
      // console.log(block);
      const tx = parseTxBlock(block.Map);
      const hash = getMapValue(findMapByKey(block.Map, "phash"));
      const timestamp = getMapValue(findMapByKey(block.Map, "ts"));
      const fee = !tx?.fee
        ? getMapValue(findMapByKey(block.Map, "fee"))
        : tx.fee;

      return {
        ...tx,
        date: getDateUTC(Number(timestamp), { fromNanos: true }),
        hash: hash ? Buffer.from(hash).toString("hex") : undefined,
        index: Number(start),
        fee: fee ? roundAndFormatLocale({ number: divideBy1e8(fee) }) : "-",
      };
    }
  });
  return data as Transaction[];
};

const findMapByKey = (maps: Map, key: string) => maps.find(([k]) => k === key);
const getMapValue = (mapValue: [string, Value] | undefined) => {
  if (mapValue && typeof mapValue[1] === "object") {
    const [[, value]] = Object.entries(mapValue[1]);
    return value;
  }
};
const parseTxBlock = (map: Map) => {
  const TYPES: { [key: string]: string } = {
    xfer: "transfer",
    mint: "mint",
    approve: "approve",
    burn: "burn",
  };
  const tx = findMapByKey(map, "tx");
  if (tx && tx[1] && "Map" in tx[1]) {
    const txMap = tx[1].Map;
    const amount = getMapValue(findMapByKey(txMap, "amt"));
    const op = getMapValue(findMapByKey(txMap, "op"));
    const fee = getMapValue(findMapByKey(txMap, "fee"));
    const from = getAccountTextBlocks({
      from: getMapValue(findMapByKey(txMap, "from")),
      to: getMapValue(findMapByKey(txMap, "to")),
      kind: op,
      account: "from",
    });
    const to =
      op === "approve"
        ? getAccountTextBlocks({
            from: getMapValue(findMapByKey(txMap, "from")),
            to: getMapValue(findMapByKey(txMap, "spender")),
            kind: op,
            account: "to",
          })
        : getAccountTextBlocks({
            from: getMapValue(findMapByKey(txMap, "from")),
            to: getMapValue(findMapByKey(txMap, "to")),
            kind: op,
            account: "to",
          });
    const memo = getMapValue(findMapByKey(txMap, "memo"));
    return {
      amount: roundAndFormatLocale({ number: divideBy1e8(amount) }),
      fee: fee ? fee : undefined,
      from,
      to,
      type: TYPES[op],
      memo: memo ? Buffer.from(memo).toString() : "-",
    };
  }
};
