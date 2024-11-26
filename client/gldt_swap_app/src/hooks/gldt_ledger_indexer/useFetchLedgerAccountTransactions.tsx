import { useQuery, UseQueryOptions } from "@tanstack/react-query";

import {
  getAccountTransactions,
  formatTransactionsResults,
  GetAccountTransactionsParams,
  Transaction,
} from "./utils";
import { useAuth } from "@auth/index";

interface Transactions {
  rows: Transaction[];
  hasNextPage: boolean;
  hasPreviousPage: boolean;
  hasResults: boolean;
  start: number | undefined;
}

type FetchLedgerTransactions = Omit<
  UseQueryOptions<Transactions>,
  "queryKey" | "queryFn"
> &
  GetAccountTransactionsParams;

export const useFetchLedgerAccountTransactions = ({
  pageSize = 100,
  owner,
  subaccount,
  start,
  ...queryParams
}: FetchLedgerTransactions) => {
  const { createActor } = useAuth();

  return useQuery({
    queryKey: [
      "FETCH_LEDGER_ACCOUNT_TRANSACTIONS",
      start,
      pageSize,
      owner,
      subaccount,
    ],
    queryFn: async (): Promise<Transactions> => {
      const actor = createActor("gldt_ledger_indexer");

      try {
        // Get first tx to get oldest tx id and do some useful calculations for pagination
        const defaultTx = await getAccountTransactions({
          actor,
          pageSize: 1,
          owner,
          subaccount,
          start,
        });
        const newestTx = defaultTx?.transactions[0]?.id ?? 0n;
        const oldestTx = defaultTx?.oldest_tx_id[0] ?? 0n;
        const maxCount = Number(newestTx - oldestTx);

        const results = await getAccountTransactions({
          actor,
          pageSize: maxCount ? maxCount : 1,
          // pageSize,
          owner,
          subaccount,
          start,
        });

        const firstPageTx = results?.transactions[0]?.id;
        const lastTxPage =
          results?.transactions[results?.transactions.length - 1]?.id;

        // console.log(`newestTx: ${newestTx}`);
        // console.log(`oldestTx: ${oldestTx}`);
        // console.log(`firstPageTx: ${firstPageTx}`);
        // console.log(`lastTxPage: ${lastTxPage}`);
        // console.log(results);

        const transactions = formatTransactionsResults(results);

        return {
          rows: transactions,
          hasNextPage: newestTx ? lastTxPage !== oldestTx : false,
          hasPreviousPage: newestTx ? firstPageTx !== newestTx : false,
          start: newestTx ? Number(firstPageTx) : undefined,
          hasResults: !!newestTx,
        };
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch account transactions error! Please refresh page and/or retry later."
        );
      }
    },
    enabled: queryParams.enabled !== undefined ? queryParams.enabled : true,
    refetchInterval: queryParams.refetchInterval ?? undefined,
  });
};
