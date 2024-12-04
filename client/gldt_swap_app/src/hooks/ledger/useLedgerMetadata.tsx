import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";

import { useAuth } from "@auth/index";
import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";

import { fetchGoldPrice1G } from "@hooks/gold_api";

interface TokenMetadata {
  name: string;
  fee: {
    e8s: bigint;
    number: number;
    string: string;
  };
  totalSupply: {
    e8s: bigint;
    number: number;
    string: string;
  };
  marketCap: string;
  decimals: number;
  symbol: string;
}

export const useLedgerMetadata = (
  {
    ledger,
    enabled,
    refetchInterval,
  }: Omit<UseQueryOptions<TokenMetadata>, "queryKey" | "queryFn"> & {
    ledger: string;
  } = { ledger: "OGY", enabled: true, refetchInterval: undefined }
) => {
  const { createActor } = useAuth();

  return useQuery({
    queryKey: ["USER_FETCH_METADATA_LEDGER_${ledger}"],
    queryFn: async () => {
      try {
        const actor = createActor(`${ledger.toLocaleLowerCase()}_ledger`);
        const decimals = (await actor.icrc1_decimals()) as number;
        const result_fee = (await actor.icrc1_fee()) as bigint;
        const symbol = (await actor.icrc1_symbol()) as string;
        const name = (await actor.icrc1_name()) as string;
        const result_total_supply =
          (await actor.icrc1_total_supply()) as bigint;
        const goldPrice = await fetchGoldPrice1G();

        const fee = divideBy1e8(result_fee);
        const totalSupply = divideBy1e8(result_total_supply);
        const marketCap = (totalSupply / 100) * goldPrice;

        const data = {
          name,
          decimals,
          fee: {
            e8s: result_fee,
            number: fee,
            string: roundAndFormatLocale({ number: fee }),
          },
          totalSupply: {
            e8s: result_total_supply,
            number: totalSupply,
            string: roundAndFormatLocale({ number: totalSupply }),
          },
          marketCap: roundAndFormatLocale({ number: marketCap }),
          symbol,
        };
        return data;
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch metadata ${ledger} ledger error! Please refresh page and/or retry later.`
        );
      }
    },
    placeholderData: keepPreviousData,
    enabled,
    refetchInterval,
  });
};
