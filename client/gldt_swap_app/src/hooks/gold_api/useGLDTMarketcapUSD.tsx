import { useEffect, useState } from "react";
import { useQuery, keepPreviousData } from "@tanstack/react-query";

import { useAuth } from "@auth/index";
import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers/index";
import { fetchGoldPrice1G } from "./index";

export const useGLDTMarketcapUSD = () => {
  const { createActor } = useAuth();
  const [GLDTMarketcapUSD, setGLDTMarketcapUSD] = useState<
    string | null | undefined
  >(null);

  const icrc1_total_supply = async () => {
    const actor = createActor(`gldt_ledger`);
    const result = (await actor.icrc1_total_supply()) as number;

    return divideBy1e8(result);
  };

  const { data, isSuccess, isLoading, isError, error } = useQuery({
    queryKey: ["FETCH_GLDT_MARKETCAP_USD"],
    queryFn: async () => {
      const goldPrice = await fetchGoldPrice1G();
      const totalSupply = await icrc1_total_supply();
      return (totalSupply / 100) * goldPrice;
    },
    placeholderData: keepPreviousData,
    enabled: true,
  });

  useEffect(() => {
    if (isSuccess) {
      setGLDTMarketcapUSD(roundAndFormatLocale({ number: data }));
    }
  }, [isSuccess, data]);

  return {
    data: GLDTMarketcapUSD,
    isSuccess,
    isLoading,
    isError,
    error,
  };
};
