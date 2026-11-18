import clsx from "clsx";
import { GLDT_VALUE_1G_NFT } from "@constants";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Token } from "@shared/utils/tokens";
import { amountDisplayDecimals } from "@shared/utils/tokens/custom";

const Balance = ({
  balance = 0,
  tokenName,
  decimals,
  className,
}: {
  balance: number;
  tokenName: string;
  decimals: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className={clsx("flex items-center gap-2", "text-3xl xl:text-4xl")}>
        <div className="font-semibold">
          <NumberToLocaleString value={balance} decimals={decimals} />
        </div>
        <div className="text-content/60">{tokenName}</div>
      </div>
    </div>
  );
};

const PriceToken = ({
  balance,
  balanceUSD,
  tokenName,
  hasPrice,
  className,
}: {
  balance: number;
  balanceUSD: number;
  tokenName: string;
  hasPrice: boolean;
  className?: string;
}) => {
  const renderPrice = () => {
    if (!hasPrice) {
      return <div className="text-content/60">No price available</div>;
    }
    switch (tokenName) {
      case "GLDT":
        return (
          <>
            <NumberToLocaleString value={balance / GLDT_VALUE_1G_NFT} /> grams
            of Gold{" "}
            <span className="text-content/60">
              ($
              <NumberToLocaleString value={balanceUSD} />)
            </span>
          </>
        );
      default:
        return (
          <div className="text-content/60">
            $<NumberToLocaleString value={balanceUSD} />
          </div>
        );
    }
  };

  return (
    <div className={className}>
      <div className="text-lg">{renderPrice()}</div>
    </div>
  );
};

const TotalCountToken = ({
  token,
  amount,
  amountUSD,
  className,
  isFetching = false,
  hasPrice = true,
}: {
  token: Token;
  amount: number;
  amountUSD: number;
  isFetching?: boolean;
  // False for a token the user added that has no price.
  hasPrice?: boolean;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        {!isFetching ? (
          <Balance
            tokenName={token.display_name}
            balance={amount}
            decimals={amountDisplayDecimals(token, amount)}
          />
        ) : (
          <Balance
            tokenName={token.display_name}
            className="animate-pulse"
            balance={0}
            decimals={2}
          />
        )}
        {!isFetching ? (
          <PriceToken
            tokenName={token.name}
            balance={amount}
            balanceUSD={amountUSD}
            hasPrice={hasPrice}
          />
        ) : (
          <PriceToken
            tokenName={token.name}
            className="animate-pulse"
            balance={0}
            balanceUSD={0}
            hasPrice={true}
          />
        )}
      </div>
    </div>
  );
};

export default TotalCountToken;
