import {
  useInfiniteQuery,
  // keepPreviousData,
  UseInfiniteQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";

import { idlFactory } from "@services/ledger-index/idlFactory";
import { idlFactory as idlFactoryICP } from "@services/ledger-index/idlFactory_icp";
import get_account_transactions from "@services/ledger-index/get_account_transactions";
import get_account_transactions_icp from "@services/ledger-index/get_account_transactions_icp";
import { Transactions } from "@services/ledger-index/utils/interfaces";

const useFetchAccountTransactions = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  args: Omit<
    UseInfiniteQueryOptions<Transactions>,
    | "queryKey"
    | "queryFn"
    | "getNextPageParam"
    | "getPreviousPageParam"
    | "initialPageParam"
  > & {
    max_results?: number;
    account: string;
    ledger: string;
  }
) => {
  const {
    enabled = true,
    placeholderData = undefined,
    max_results = 20,
    account,
    ledger,
  } = args;

  return useInfiniteQuery({
    queryKey: ["FETCH_ACCOUNT_TRANSACTIONS", account, ledger],
    queryFn: async ({ pageParam = null }) => {
      try {
        const decodedAccount = decodeIcrcAccount(account);
        const owner = decodedAccount.owner;
        const subaccount = decodedAccount?.subaccount
          ? [decodedAccount.subaccount]
          : [];

        let results: Transactions;

        if (ledger === "icp") {
          const actor = Actor.createActor(idlFactoryICP, {
            agent,
            canisterId,
          });
          results = await get_account_transactions_icp(actor, {
            max_results,
            start: pageParam as number | null,
            owner,
            subaccount,
          });
        } else {
          const actor = Actor.createActor(idlFactory, {
            agent,
            canisterId,
          });

          results = await get_account_transactions(actor, {
            max_results,
            start: pageParam as number | null,
            owner,
            subaccount,
          });
        }

        return results;
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch account transactions error! Please retry later."
        );
      }
    },
    initialPageParam: null,
    getNextPageParam: (page) => page.cursor_index ?? null,
    placeholderData,
    enabled,
  });
};

export default useFetchAccountTransactions;
