import {
  useInfiniteQuery,
  // keepPreviousData,
  UseInfiniteQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { AccountIdentifier } from "@dfinity/ledger-icp";
import { idlFactory } from "@services/ledger-index/idlFactory";
import { idlFactory as idlFactoryICP } from "@services/ledger-index/idlFactory_icp";
import get_account_transactions from "@services/ledger-index/get_account_transactions";
import get_account_transactions_icp from "@services/ledger-index/get_account_transactions_icp";
import { Transactions } from "@services/ledger-index/utils/interfaces";
import { Ledger } from "@services/ledger/utils/interfaces";

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
    ledger: Ledger;
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
    queryKey: ["FETCH_ACCOUNT_TRANSACTIONS", ledger, account],
    queryFn: async ({ pageParam = null }) => {
      try {
        const decodedAccount = decodeIcrcAccount(account);
        const owner = decodedAccount.owner;
        const subaccount = decodedAccount?.subaccount
          ? [decodedAccount.subaccount]
          : [];
        const accountId = AccountIdentifier.fromPrincipal({
          principal: Principal.fromText(account),
        }).toHex();

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

        const newData = results.data?.map((tx) => {
          const is_credit = tx.to === accountId || tx.to === account;
          const from = tx.from && tx.from === accountId ? account : tx.from;
          const to = tx.to && tx.to === accountId ? account : tx.to;

          return {
            ...tx,
            is_credit,
            from,
            to,
          };
        });

        return {
          data: newData,
          cursor_index: results.cursor_index,
        };
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
