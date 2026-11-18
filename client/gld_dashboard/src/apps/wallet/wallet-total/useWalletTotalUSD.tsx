import { useQueries } from "@tanstack/react-query";
import { useAtomValue } from "jotai";
import { useAuth } from "@auth/index";
import { WALLET_LIST, Token } from "@shared/utils/tokens";
import { NFTCollections } from "@shared/utils/nfts";
import { CustomTokensAtom } from "@wallet/shared/atoms/CustomTokensAtom";
import { ledgerBalanceQuery } from "@shared/hooks/useFetchLedgerBalance";
import useFetchUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";

const WALLET_TOKENS = WALLET_LIST.filter(
  (item): item is Token => item !== "GLDNFT"
);

// Sum of the USD values shown in the wallet list: every token balance plus the
// gold value of the GLD NFTs. The queries share their cache with the list rows.
// A token the user added counts with its price, or with 0 when it has none.
const useWalletTotalUSD = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const customTokens = useAtomValue(CustomTokensAtom);
  const enabled = !!unauthenticatedAgent && isConnected;

  const balances = useQueries({
    queries: [...WALLET_TOKENS, ...customTokens].map((token) => ({
      ...ledgerBalanceQuery(token.canister_id, unauthenticatedAgent, {
        ledger: token.name,
        owner: principalId,
      }),
      enabled,
      staleTime: 60 * 1000,
      refetchOnWindowFocus: false,
    })),
  });

  const nfts = useFetchUserNFTMetrics({
    owner: principalId,
    nft_collections: NFTCollections,
    enabled,
    agent: unauthenticatedAgent,
  });

  const priceGold = useFetchPriceGold({ enabled });

  const tokensUSD = balances.reduce(
    (acc, { data }) => acc + (data?.balance_usd ?? 0),
    0
  );
  const nftsGrams = nfts.data?.totalGrams ?? 0;
  const nftsUSD = nftsGrams * (priceGold.data ?? 0);

  // The gold price only matters when there are NFTs to value.
  const queries = [...balances, nfts, ...(nftsGrams > 0 ? [priceGold] : [])];

  return {
    totalUSD: tokensUSD + nftsUSD,
    isLoading: queries.some((query) => query.isLoading),
    // When true, totalUSD only covers the balances that could be fetched.
    isError: queries.some((query) => query.isError),
  };
};

export default useWalletTotalUSD;
