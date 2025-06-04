import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { SortingState } from "@tanstack/react-table";

import {
  fetch_one_account_transactions,
  Transactions,
  Transaction,
} from "@services/ledger";
import { getDateUTC } from "@shared/utils/dates";
import { divideBy1e8, roundAndFormatLocale } from "@shared/utils/numbers";

export type { Transaction };

const useFetchAccountTransactions = (
  canisterId: string,
  options: Omit<UseQueryOptions<Transactions>, "queryKey" | "queryFn"> & {
    limit?: number;
    offset?: number;
    sorting?: SortingState;
    account: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = keepPreviousData,
    limit,
    offset,
    sorting,
    account,
  } = options;

  return useQuery({
    queryKey: [
      "FETCH_ALL_GLDGOV_TRANSACTIONS_ONE_ACCOUNT",
      limit,
      offset,
      sorting,
      account,
    ],
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

        const results = await fetch_one_account_transactions({
          limit,
          offset,
          sort_by: sort_by ?? "",
          canisterId,
          account,
        });

        const data = results.data?.map((transaction) => {
          const index = transaction?.index;
          const timestamp = transaction?.timestamp;
          const from_account = transaction?.from_account;
          const to_account = transaction?.to_account;
          const amount = transaction?.amount;
          const fee = transaction?.fee;
          const memo = transaction?.memo;
          const kind = transaction?.kind;
          return {
            index,
            timestamp: timestamp
              ? getDateUTC(timestamp, { fromNanos: true })
              : "-",
            from_account: kind === "mint" ? "Minting account" : from_account,
            to_account: kind === "burn" ? "Minting account" : to_account,
            amount: roundAndFormatLocale({
              number: divideBy1e8(Number(amount)),
              decimals: 4,
            }),
            fee,
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
        throw new Error(
          "Fetch one account transactions error! Please retry later."
        );
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchAccountTransactions;
