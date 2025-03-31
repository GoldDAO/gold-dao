import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";

import { GOLDAO_LEDGER_CANISTER_ID_IC } from "@constants";

import fetch_one_transaction, { Transaction } from "../fetch_one_transaction";
import { getDateUTC } from "@utils/dates";
import { divideBy1e8, numberToLocaleString } from "@utils/numbers";

export type { Transaction };

const useFetchOneTransaction = ({
  enabled = true,
  placeholderData = keepPreviousData,
  transactionId,
}: Omit<UseQueryOptions<Transaction>, "queryKey" | "queryFn"> & {
  transactionId: string;
}) => {
  return useQuery({
    queryKey: ["FETCH_ONE_TRANSACTION", transactionId],
    queryFn: async () => {
      try {
        const result = await fetch_one_transaction({
          transactionId,
          canisterId: GOLDAO_LEDGER_CANISTER_ID_IC,
        });

        const index = result?.index;
        const timestamp = result?.timestamp;
        const from_account = result?.from_account;
        const to_account = result?.to_account;
        const amount = result?.amount;
        const fee = result?.fee;
        const memo = result?.memo ?? "-";
        const kind = result?.kind;

        const data = {
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

        return data;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch one transaction error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchOneTransaction;
