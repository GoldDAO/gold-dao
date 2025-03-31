import { instance } from "./icp_icrc_api/instance";

import { Transaction } from "./utils/interfaces";

export type { Transaction };

const fetch_one_transaction = async ({
  canisterId,
  transactionId,
}: {
  canisterId: string;
  transactionId: string;
}): Promise<Transaction> => {
  const { data } = await instance.get(
    `/ledgers/${canisterId}/transactions/${transactionId}`
  );
  return data;
};

export default fetch_one_transaction;
