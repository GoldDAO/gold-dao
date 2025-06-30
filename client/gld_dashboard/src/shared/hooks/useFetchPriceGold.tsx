import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { fetch_gold_price } from "@services/bity_gold_api";

const useFetchPriceGold = (
  options: Omit<UseQueryOptions<number>, "queryKey" | "queryFn">
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["FETCH_PRICE_GOLD"],
    queryFn: async () => {
      const data = await fetch_gold_price();
      return data;
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchPriceGold;
