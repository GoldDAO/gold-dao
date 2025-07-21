import { useAuth } from "@auth/index";
import useFetchLedgerDecimals from "@shared/hooks/useFetchLedgerDecimals";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import { Logo } from "@components/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Token } from "@shared/utils/tokens";

const HeaderToken = ({
  token,
  className,
}: {
  token: Token;
  className?: string;
}) => {
  const { unauthenticatedAgent } = useAuth();

  const decimals = useFetchLedgerDecimals(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
      enabled: !!unauthenticatedAgent,
    }
  );

  const price = useFetchTokenPrice(unauthenticatedAgent, {
    from: token.name,
    from_canister_id: token.canister_id,
    amount: BigInt(1 * 10 ** (decimals.data ?? 0)),
    enabled: !!unauthenticatedAgent && decimals.isSuccess,
  });

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex items-center gap-1">
          <Logo name={token.id} className="h-6 w-6" />
          <div className="font-semibold text-xl">{token.display_name}</div>
        </div>
        <div className="text-xs xl:text-sm text-content/60">
          {price.isSuccess ? (
            <>
              1 {token.name} ≈ $
              <NumberToLocaleString value={price.data.amount_usd} />
            </>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    </div>
  );
};

export default HeaderToken;
