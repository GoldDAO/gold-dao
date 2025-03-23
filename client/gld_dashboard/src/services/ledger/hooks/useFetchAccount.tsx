import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";

import { divideBy1e8, numberToLocaleString } from "@utils/numbers";
import { fetch_one_account } from "@services/ledger";

const useFetchAccount = (
  canisterId: string,
  options: Omit<
    UseQueryOptions<{
      id: string;
      owner: string;
      subaccount: string;
      balance: string;
      total_transactions: string;
    }>,
    "queryKey" | "queryFn"
  > & {
    accountId: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    accountId,
  } = options;
  return useQuery({
    queryKey: ["FETCH_LEDGER_ACCOUNT", accountId],
    queryFn: async () => {
      const result = await fetch_one_account({
        accountId,
        canisterId,
      });
      const subaccount = result.subaccount;
      const has_subaccount =
        subaccount !== "" && subaccount !== null && subaccount !== undefined;

      return {
        id: result.id,
        owner: result.owner,
        subaccount: has_subaccount ? subaccount : "None (default subaccount)",
        balance: numberToLocaleString({
          value: divideBy1e8(Number(result.balance)),
          decimals: 8,
        }),
        total_transactions: numberToLocaleString({
          value: result.total_transactions,
          decimals: 0,
        }),
      };
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchAccount;
