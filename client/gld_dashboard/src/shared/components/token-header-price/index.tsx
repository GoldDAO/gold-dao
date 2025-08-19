import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Token } from "@shared/utils/tokens";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";

const TokenHeaderPrice = ({
  token,
  className,
}: {
  token: Token;
  className?: string;
}) => {
  const { unauthenticatedAgent } = useAuth();

  const price = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: token.name,
      from_canister_id: token.canister_id,
      to: "ckUSDT",
      amount: 1,
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex items-center gap-1">
          <Logo name={token.id} className="h-6 w-6" />
          <div className="font-semibold text-xl">{token.display_name}</div>
        </div>
        <div className="text-xs xl:text-sm text-content/60">
          {price.isSuccess && !price.isFetching ? (
            <>
              1 {token.name} ≈ $
              <NumberToLocaleString value={price.data.mid_price} decimals={5} />
            </>
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
