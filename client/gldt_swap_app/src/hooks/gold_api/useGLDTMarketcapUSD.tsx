import { useEffect, useState } from "react";
import { useQuery, keepPreviousData } from "@tanstack/react-query";

import { roundAndFormatLocale } from "@utils/numbers/index";
import { fetchGoldPrice1G } from "./index";

export const useGLDTMarketcapUSD = () => {
  const [GLDTMarketcapUSD, setGLDTMarketcapUSD] = useState<
    string | null | undefined
  >(null);

  const { data, isSuccess, isLoading, isError, error } = useQuery({
    queryKey: ["FETCH_GLDT_MARKETCAP_USD"],
    queryFn: () => fetchGoldPrice1G(),
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
