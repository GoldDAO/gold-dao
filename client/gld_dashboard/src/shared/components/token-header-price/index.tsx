import { ICPSWAP_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import TokenLogo from "@shared/components/token-logo";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Token } from "@shared/utils/tokens";
import {
  useQuery,
  keepPreviousData,
} from "@tanstack/react-query";
import { Actor } from "@dfinity/agent";
import { idlFactory as idlFactoryIcpswap } from "@services/icpswap/idls/swap_pool";
import { get_token_price_usd_or_null } from "@services/icpswap/get_token_price_usd";

const TokenHeaderPrice = ({
  token,
  className,
}: {
  token: Token;
  className?: string;
}) => {
  const { unauthenticatedAgent } = useAuth();

  const price = useQuery({
    queryKey: [`FETCH_TOKEN_HEADER_PRICE`, token.canister_id],
    queryFn: async () => {
      const actor = Actor.createActor(idlFactoryIcpswap, {
        agent: unauthenticatedAgent,
        canisterId: ICPSWAP_CANISTER_ID,
      });
      // null: a token the user added that has no price
      return get_token_price_usd_or_null(
        actor,
        token.canister_id,
        token.name,
        { agent: unauthenticatedAgent }
      );
    },
    placeholderData: keepPreviousData,
    enabled: !!unauthenticatedAgent,
    staleTime: 60 * 1000,
  });

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex items-center gap-1">
          <TokenLogo token={token} className="h-6 w-6" />
          <div className="font-semibold text-xl">{token.display_name}</div>
        </div>
        <div className="text-xs xl:text-sm text-content/60">
          {price.isSuccess && !price.isFetching ? (
            price.data === null ? (
              <>No price available</>
            ) : (
              <>
                1 {token.name} ≈ $
                <NumberToLocaleString value={price.data} decimals={5} />
              </>
            )
          ) : (
            <div className="animate-pulse">
              1 {token.name} ≈ $
              <NumberToLocaleString value={0} />
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default TokenHeaderPrice;
