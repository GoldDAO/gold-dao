import { useEffect, useState } from "react";
import { useQuery, keepPreviousData } from "@tanstack/react-query";

import { roundAndFormatLocale } from "@utils/numbers/index";
import { fetchGoldPrice1G } from "./index";

export const useBalanceGLDTUSD = ({
  balance,
}: {
  balance: number | undefined | null;
}) => {
  const [balanceUSD, setBalanceUSD] = useState<string | null | undefined>(null);

  const { data, isSuccess, isLoading, isError, error } = useQuery({
    queryKey: ["FETCH_BALANCE_GLDT_USD"],
    queryFn: () => fetchGoldPrice1G(),
    placeholderData: keepPreviousData,
    enabled: balance !== undefined && balance !== null,
  });

  useEffect(() => {
    if (isSuccess && balance !== undefined && balance !== null) {
      setBalanceUSD(roundAndFormatLocale({ number: (balance / 100) * data }));
    }
  }, [isSuccess, data, balance]);

  return {
    data: balanceUSD,
    isSuccess,
    isLoading,
    isError,
    error,
  };
};
