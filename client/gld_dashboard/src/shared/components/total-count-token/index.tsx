import clsx from "clsx";
import { GLDT_VALUE_1G_NFT } from "@constants";
import { useAuth } from "@auth/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import { Token } from "@wallet/shared/utils";

const Balance = ({
  balance = 0,
  tokenName,
  className,
}: {
  balance: number;
  tokenName: string;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className={clsx("flex items-center gap-2", "text-2xl xl:text-4xl")}>
        <div className="font-semibold">
          <NumberToLocaleString value={balance} />
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
  className,
}: {
  balance: number;
  balanceUSD: number;
  tokenName: string;
  className?: string;
}) => {
  const renderPrice = () => {
    switch (tokenName) {
      case "GLDT":
        return (
          <>
            <NumberToLocaleString value={balance / GLDT_VALUE_1G_NFT} /> grams
            of Gold (
            <>
              $<NumberToLocaleString value={balanceUSD} />
            </>
            )
          </>
        );
      default:
        return (
          <>
            $<NumberToLocaleString value={balanceUSD} />
          </>
        );
    }
  };

  return (
    <div className={className}>
      <div className="text-sm text-content/60">{renderPrice()}</div>
    </div>
  );
};

const TotalCountToken = ({ token }: { token: Token }) => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();

  const balance = useFetchLedgerBalance(
    token.canisterId,
    unauthenticatedAgent,
    {
      ledger: token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const renderBalance = () => {
    if (!isConnected) {
      return <Balance tokenName={token.name} balance={0} />;
    }
    if (balance.isSuccess) {
      return <Balance tokenName={token.name} balance={balance.data.balance} />;
    }
    return (
      <Balance tokenName={token.name} className="animate-pulse" balance={0} />
    );
  };

  const renderPrice = () => {
    if (!isConnected) {
      return <PriceToken tokenName={token.name} balance={0} balanceUSD={0} />;
    }
    if (balance.isSuccess) {
      return (
        <PriceToken
          tokenName={token.name}
          balance={balance.data.balance}
          balanceUSD={balance.data.balance_usd}
        />
      );
    }
    return (
      <PriceToken
        tokenName={token.name}
        balance={0}
        balanceUSD={0}
        className="animate-pulse"
      />
    );
  };

  return (
    <div className="flex flex-col items-center gap-2">
      {renderBalance()}
      {renderPrice()}
    </div>
  );
};

export default TotalCountToken;
