import { useEffect, useState } from "react";
import { useQuery, keepPreviousData } from "@tanstack/react-query";

import { roundAndFormatLocale } from "@utils/numbers/index";
import { instance } from "./index";

const fetchOGYPrice = async () => {
  const { data } = await instance.get(`/ogy/price`);
  const { ogyPrice } = data;

  return {
    ogyPrice,
  };
};

export const useBalanceOGYUSD = ({
  balance,
}: {
  balance: number | undefined | null;
}) => {
  const [balanceUSD, setBalanceUSD] = useState<string | null | undefined>(null);

  const { data, isSuccess, isLoading, isError, error } = useQuery({
    queryKey: ["FETCH_OGY_PRICE"],
    queryFn: () => fetchOGYPrice(),
    placeholderData: keepPreviousData,
    enabled: balance !== undefined && balance !== null,
  });

  useEffect(() => {
    if (isSuccess && balance !== undefined && balance !== null) {
      setBalanceUSD(roundAndFormatLocale({ number: balance * data.ogyPrice }));
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
