import {
  keepPreviousData,
  useQuery,
  UseQueryOptions,
} from "@tanstack/react-query";

import {
  getAccountTransactions,
  formatTransactionsResults,
  GetAccountTransactionsParams,
  Transaction,
} from "./utils";
import { useAuth } from "@auth/index";

type FetchLedgerTransactions = Omit<
  UseQueryOptions<Transaction>,
  "queryKey" | "queryFn"
> &
  GetAccountTransactionsParams;

export const useFetchLedgerOneAccountTransaction = ({
  pageSize = 1,
  owner,
  subaccount,
  start,
  ...queryParams
}: FetchLedgerTransactions) => {
  const { createActor } = useAuth();

  return useQuery({
    queryKey: [
      "FETCH_LEDGER_ONE_ACCOUNT_TRANSACTION",
      start,
      pageSize,
      owner,
      subaccount,
    ],
    queryFn: async (): Promise<Transaction> => {
      const actor = createActor("gldt_ledger_indexer");

      try {
        const results = await getAccountTransactions({
          actor,
          pageSize: 1,
          owner,
          subaccount,
          start,
        });

        const transactions = formatTransactionsResults(results);
        return transactions[0];
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch account transaction error! Please refresh page and/or retry later."
        );
      }
    },
    placeholderData: keepPreviousData,
    enabled: queryParams.enabled !== undefined ? queryParams.enabled : true,
    refetchInterval: queryParams.refetchInterval ?? undefined,
  });
};
