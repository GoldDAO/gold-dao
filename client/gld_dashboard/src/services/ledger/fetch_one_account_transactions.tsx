import { instance } from "./icp_icrc_api/instance";

import { Transactions, Transaction } from "./utils/interfaces";

export type { Transactions, Transaction };

export const fetch_one_account_transactions = async ({
  limit = 20,
  offset = 0,
  sort_by = undefined,
  canisterId,
  account,
}: {
  limit?: number;
  offset?: number;
  sort_by?: string;
  canisterId: string;
  account: string;
}): Promise<Transactions> => {
  const { data } = await instance.get(
    `/ledgers/${canisterId}/accounts/${account}/transactions?limit=${limit}&offset=${offset}${
      sort_by ? `&sort_by=${sort_by}` : ""
    }`
  );
  return data;
};
