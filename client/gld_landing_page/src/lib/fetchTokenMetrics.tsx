export interface TokenMetrics {
  gold_price: string;
  total_gold_grams: string;
  tvl: string;
}

export const fetchTokenMetrics = async (): Promise<TokenMetrics> => {
  const response = await fetch(
    "https://teiwz-pqaaa-aaaap-ag7hq-cai.raw.icp0.io/gold_nft_metrics"
  );
  if (!response.ok) {
    throw new Error("Failed to fetch token metrics");
  }
  return response.json();
};
