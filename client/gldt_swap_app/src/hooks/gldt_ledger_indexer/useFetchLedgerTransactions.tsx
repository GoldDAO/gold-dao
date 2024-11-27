import {
  useQuery,
  UseQueryOptions,
  keepPreviousData,
} from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Buffer } from "buffer";
import { encodeIcrcAccount, IcrcSubaccount } from "@dfinity/ledger-icrc";

import { useAuth } from "@auth/index";
import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";
import { getDateUTC } from "@utils/dates";
import {
  GetBlocksRequest,
  GetBlocksResponse,
  Status,
  Map,
  Value,
} from "@canisters/gldt_ledger_indexer/interface.ts";

export interface TxAccount {
  owner: string | undefined;
  subaccount: string | undefined;
  full: string;
}

export interface Transaction {
  amount: string;
  from: undefined | TxAccount;
  to: undefined | TxAccount;
  date: string;
  type: string;
  hash: string | undefined;
  index: number;
}

interface Transactions {
  rowCount: number;
  pageCount: number;
  rows: Transaction[];
}

type FetchLedgerTransactions = Omit<
  UseQueryOptions<Transactions>,
  "queryKey" | "queryFn"
> & {
  pageSize?: number;
  page?: number;
};

const findMapByKey = (maps: Map, key: string) => maps.find(([k]) => k === key);
const getMapValue = (mapValue: [string, Value] | undefined) => {
  if (mapValue && typeof mapValue[1] === "object") {
    const [[, value]] = Object.entries(mapValue[1]);
    return value;
  }
};

const getAccountText = ({
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

const parseTxBlock = (map: Map) => {
  const TYPES: { [key: string]: string } = {
    xfer: "transfer",
    mint: "mint",
    approve: "approve",
    burn: "burn",
  };
  const tx = findMapByKey(map, "tx");
  // console.log(tx);
  if (tx && tx[1] && "Map" in tx[1]) {
    const txMap = tx[1].Map;
    const amount = getMapValue(findMapByKey(txMap, "amt"));
    const op = getMapValue(findMapByKey(txMap, "op"));
    const from = getAccountText({
      from: getMapValue(findMapByKey(txMap, "from")),
      to: getMapValue(findMapByKey(txMap, "to")),
      kind: op,
      account: "from",
    });
    const to =
      op === "approve"
        ? getAccountText({
            from: getMapValue(findMapByKey(txMap, "from")),
            to: getMapValue(findMapByKey(txMap, "spender")),
            kind: op,
            account: "to",
          })
        : getAccountText({
            from: getMapValue(findMapByKey(txMap, "from")),
            to: getMapValue(findMapByKey(txMap, "to")),
            kind: op,
            account: "to",
          });
    // const memo = getMapValue(findMapByKey(txMap, "memo"));
    // console.log(Buffer.from(memo).toString());
    return {
      amount: roundAndFormatLocale({ number: divideBy1e8(amount) }),
      from,
      to,
      type: TYPES[op],
    };
  }
};

export const useFetchLedgerTransactions = ({
  pageSize = 20,
  page = 0,
  ...queryParams
}: FetchLedgerTransactions = {}) => {
  const { createActor } = useAuth();

  const fetch_transactions_total_count = async (): Promise<number> => {
    const actor = createActor("gldt_ledger_indexer");
    const result = (await actor.status()) as Status;
    const data = Number(result.num_blocks_synced);
    return data;
  };

  const get_blocks = async ({
    start,
    length,
  }: GetBlocksRequest): Promise<Transaction[]> => {
    const actor = createActor("gldt_ledger_indexer");
    const results = (await actor.get_blocks({
      start,
      length,
    })) as GetBlocksResponse;
    const data = results.blocks.map((block, index) => {
      // console.log(block);
      if ("Map" in block) {
        const tx = parseTxBlock(block.Map);
        const hash = getMapValue(findMapByKey(block.Map, "phash"));
        const timestamp = getMapValue(findMapByKey(block.Map, "ts"));

        return {
          ...tx,
          date: getDateUTC(Number(timestamp), { fromNanos: true }),
          hash: hash ? Buffer.from(hash).toString("hex") : undefined,
          index: Number(start) + index,
        };
      }
    });
    return data as Transaction[];
  };

  return useQuery({
    queryKey: ["FETCH_LEDGER_TRANSACTIONS", page, pageSize],
    queryFn: async () => {
      try {
        const totalCount = await fetch_transactions_total_count();
        let start = BigInt(totalCount - pageSize * (page + 1));
        let length = BigInt(pageSize);

        if (start <= length) {
          length = start;
          start = 0n;
        }

        const transactions = await get_blocks({ start, length });
        const pageCount = Math.ceil(totalCount / pageSize);

        const ret = {
          rowCount: totalCount - pageSize,
          pageCount,
          rows: transactions.reverse(),
        };
        return ret;
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch ledger transactions error! Please refresh page and/or retry later."
        );
      }
    },
    placeholderData: keepPreviousData,
    enabled: queryParams.enabled !== undefined ? queryParams.enabled : true,
    refetchInterval: queryParams.refetchInterval ?? undefined,
  });
};
