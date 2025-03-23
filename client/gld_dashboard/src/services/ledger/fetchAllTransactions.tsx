import { instance } from "./icp_icrc_api/instance";

import { Transactions, Transaction } from "./utils/interfaces";

export type { Transactions, Transaction };

export const fetchAllTransactions = async ({
  limit = 20,
  offset = 0,
  sort_by = undefined,
  canisterId,
}: {
  limit?: number;
  offset?: number;
  sort_by?: string;
  canisterId: string;
}): Promise<Transactions> => {
  const { data } = await instance.get(
    `/ledgers/${canisterId}/transactions?limit=${limit}&offset=${offset}${
      sort_by ? `&sort_by=${sort_by}` : ""
    }`
  );
  return data;
};
