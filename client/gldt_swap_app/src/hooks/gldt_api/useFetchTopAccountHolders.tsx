import { useQuery, UseQueryOptions } from "@tanstack/react-query";
import axios from "axios";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";

import { useAuth } from "@auth/index";
import { GLDT_API_BASE_URL } from "@constants";
import { divideBy1e8, roundAndFormatLocale } from "@utils/numbers";

export interface TxAccount {
  owner: string | undefined;
  subaccount: string | undefined;
  full: string;
}
export interface AccountTopHolder {
  rank: number;
  address: TxAccount;
  quantity: string;
  percentage: string;
}
interface AccountTopHolders {
  rows: AccountTopHolder[];
  hasResults: boolean;
}

interface RichListResponse {
  data: { account: string; balance: number }[];
  total: number;
}

const fetchRichList = async (limit: number) => {
  try {
    const { data } = await axios.get<RichListResponse>(
      `${GLDT_API_BASE_URL}/v1/tokens/GLDT/holders/rich-list`,
      { params: { limit } }
    );
    return data.data;
  } catch (err) {
    // The API answers 404 for an empty page
    if (axios.isAxiosError(err) && err.response?.status === 404) return [];
    throw err;
  }
};

export const useFetchTopAccountHolders = (
  {
    pageSize,
    enabled,
    refetchInterval,
  }: // ...queryParams
  Omit<UseQueryOptions<AccountTopHolders>, "queryKey" | "queryFn"> & {
    pageSize?: number;
  } = { pageSize: 100, enabled: true, refetchInterval: undefined }
) => {
  const { createActor } = useAuth();

  return useQuery({
    queryKey: ["FETCH_ACCOUNT_HOLDERS", pageSize],
    queryFn: async (): Promise<AccountTopHolders> => {
      try {
        const actorGLDTLedger = await createActor("gldt_ledger");
        const [totalSupplyGLDTLedger, richList] = await Promise.all([
          actorGLDTLedger.icrc1_total_supply() as Promise<bigint>,
          fetchRichList(pageSize ?? 100),
        ]);
        const totalSupply = divideBy1e8(totalSupplyGLDTLedger);

        const results = richList.map(({ account, balance }, index) => {
          const { owner, subaccount } = decodeIcrcAccount(account);

          return {
            rank: index + 1,
            address: {
              owner: owner.toText(),
              subaccount: subaccount
                ? Array.from(subaccount, (b) =>
                    b.toString(16).padStart(2, "0")
                  ).join("")
                : "",
              full: account,
            },
            quantity: roundAndFormatLocale({ number: balance }),
            percentage: ((balance / totalSupply) * 100).toFixed(2),
          };
        });
        return {
          rows: results,
          hasResults: !!results.length,
        };
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch top account holders error! Please refresh page and/or retry later."
        );
      }
    },
    enabled,
    refetchInterval,
  });
};
