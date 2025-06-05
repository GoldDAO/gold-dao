import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { SortingState } from "@tanstack/react-table";

import { fetchAllTransactions, Transactions, Transaction } from "../index";
import { getDateUTC } from "@shared/utils/dates";
import { divideBy1e8, numberToLocaleString } from "@shared/utils/numbers";

export type { Transaction };

const useFetchAllTransactions = (
  canisterId: string,
  options: Omit<UseQueryOptions<Transactions>, "queryKey" | "queryFn"> & {
    limit?: number;
    offset?: number;
    sorting?: SortingState;
  } = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    limit,
    offset,
    sorting = undefined,
  } = options;

  return useQuery({
    queryKey: ["FETCH_ALL_TRANSACTIONS", limit, offset, sorting],
    queryFn: async () => {
      try {
        let sort_by = undefined;
        if (sorting && sorting[0]) {
          const { id, desc } = sorting[0];
          const index =
            id === "kind" || id === "timestamp" || id === "amount"
              ? ",-index"
              : "";

          sort_by = desc ? `-${id}${index}` : `${id}${index}`;
        }

        const results = await fetchAllTransactions({
          limit,
          offset,
          sort_by: sort_by ?? "",
          canisterId,
        });

        const data = results.data?.map((transaction) => {
          const index = transaction?.index;
          const timestamp = transaction?.timestamp;
          const from_account = transaction?.from_account;
          const to_account = transaction?.to_account;
          const amount = transaction?.amount;
          const fee = transaction?.fee;
          const memo = transaction?.memo ?? "-";
          const kind = transaction?.kind;
          return {
            index,
            timestamp: timestamp
              ? getDateUTC(timestamp, { fromNanos: true })
              : "-",
            from_account: kind === "mint" ? "Minting account" : from_account,
            to_account: kind === "burn" ? "Minting account" : to_account,
            amount: numberToLocaleString({
              value: divideBy1e8(amount),
              decimals: 4,
            }),
            fee: numberToLocaleString({ value: divideBy1e8(fee), decimals: 4 }),
            memo,
            kind,
          };
        });

        return {
          data,
          total_transactions: results.total_transactions,
        };
      } catch (err) {
        console.error(err);
        throw new Error("Fetch all transactions error! Please retry later.");
      }
    },
    placeholderData,
    refetchInterval,
    enabled,
  });
};

export default useFetchAllTransactions;
