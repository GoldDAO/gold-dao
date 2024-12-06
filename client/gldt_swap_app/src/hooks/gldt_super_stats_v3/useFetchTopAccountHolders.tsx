import { useQuery, UseQueryOptions } from "@tanstack/react-query";
import { Buffer } from "buffer";
import { encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";

import { useAuth } from "@auth/index";
import { HolderBalanceResponse } from "@canisters/gldt_super_stats_v3/interface";
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
        const totalSupplyGLDTLedger =
          (await actorGLDTLedger.icrc1_total_supply()) as bigint;
        const actorGLDTSuperStats = await createActor("gldt_super_stats_v3");
        const resultsGLDTSuperStats =
          (await actorGLDTSuperStats.get_top_account_holders(
            pageSize
          )) as HolderBalanceResponse[];

        const results = resultsGLDTSuperStats.map(
          ({ data, holder }: HolderBalanceResponse, index) => {
            const [owner, subaccount] = holder.split(".");

            const address = encodeIcrcAccount({
              owner: Principal.fromText(owner),
              subaccount: subaccount
                ? [...Uint8Array.from(Buffer.from(subaccount, "hex"))]
                : [],
            });

            return {
              rank: index + 1,
              address: {
                owner,
                subaccount:
                  !isNaN(Number(subaccount)) && Number(subaccount) !== 0
                    ? subaccount
                    : "",
                full: address,
              },
              quantity: data.balance
                ? roundAndFormatLocale({ number: divideBy1e8(data.balance) })
                : "-",
              percentage: (
                (divideBy1e8(data.balance) /
                  divideBy1e8(totalSupplyGLDTLedger)) *
                100
              ).toFixed(2),
            };
          }
        );
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
