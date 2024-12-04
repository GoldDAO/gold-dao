import { useQuery, UseQueryOptions } from "@tanstack/react-query";

import {
  getAccountTransactions,
  formatTransactionsResults,
  GetAccountTransactionsParams,
  Transaction,
} from "./utils";
import { useAuth } from "@auth/index";
import { TransactionWithId } from "@canisters/gldt_ledger_indexer/interface";

interface Transactions {
  rows: Transaction[];
  // hasNextPage: boolean;
  // hasPreviousPage: boolean;
  hasResults: boolean;
  // start: number | undefined;
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
        const defaultTx = await getAccountTransactions({
          actor,
          pageSize: 1,
          owner,
          subaccount,
          start,
        });
        const newestTx = defaultTx?.transactions[0]?.id ?? 0n;
        const oldestTx = defaultTx?.oldest_tx_id[0] ?? 0n;

        let results: Array<TransactionWithId> = [];

        const fetchPage = async (
          start: number | undefined = undefined,
          retries = 3
        ): Promise<Array<TransactionWithId>> => {
          try {
            const res = await getAccountTransactions({
              actor,
              pageSize: 2000,
              owner,
              subaccount,
              start,
            });

            results = results.concat(res?.transactions ?? []);
            const lastTxPage =
              res?.transactions[res?.transactions.length - 1]?.id;

            if (lastTxPage && lastTxPage !== oldestTx) {
              return await fetchPage(Number(lastTxPage));
            }

            return results;
          } catch (error) {
            if (retries > 0) {
              return await fetchPage(start, retries - 1);
            }
            throw error;
          }
        };

        await fetchPage();

        const transactions = formatTransactionsResults(results);

        return {
          rows: transactions,
          // hasNextPage: newestTx ? lastTxPage !== oldestTx : false,
          // hasPreviousPage: newestTx ? firstPageTx !== newestTx : false,
          // start: newestTx ? Number(firstPageTx) : undefined,
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
