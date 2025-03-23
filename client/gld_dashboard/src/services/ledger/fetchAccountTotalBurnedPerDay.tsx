import { instance } from "./icp_icrc_api/instance";

// const ACCOUNT_URL = `/ledgers/${GOLDAO_LEDGER_CANISTER_ID_IC}/accounts/${ROOT_ACCOUNT_GLDGOV}`;

export const fetchAccountTotalBurnedPerDay = async ({
  start,
  step,
  canisterId,
  account,
}: {
  start: number;
  step: number;
  canisterId: string;
  account: string;
}) => {
  const { data } = await instance.get(
    `/ledgers/${canisterId}/accounts/${account}/total-burned-per-day?start=${start}&step=${step}`
  );
  return data;
};
