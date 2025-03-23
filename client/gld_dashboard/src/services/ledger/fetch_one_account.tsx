import { instance } from "./icp_icrc_api/instance";
import { Account } from "./utils/interfaces";

export const fetch_one_account = async ({
  accountId,
  canisterId,
}: {
  accountId: string;
  canisterId: string;
}): Promise<Account> => {
  const { data } = await instance.get(
    `/ledgers/${canisterId}/accounts/${accountId}`
  );
  return data;
};
