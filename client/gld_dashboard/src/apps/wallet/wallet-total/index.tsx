import clsx from "clsx";
import { useAuth } from "@auth/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import useWalletTotalUSD from "@wallet/wallet-total/useWalletTotalUSD";

const WalletTotal = ({ className }: { className?: string }) => {
  const { isConnected } = useAuth();
  const { totalUSD, isLoading, isError } = useWalletTotalUSD();

  return (
    <div
      className={clsx("flex flex-col items-center xl:items-start", className)}
    >
      <div className="text-sm text-content/60">Total value</div>
      {isConnected ? (
        <div
          className={clsx("text-2xl font-semibold", {
            "animate-pulse": isLoading,
          })}
        >
          $<NumberToLocaleString value={isLoading ? 0 : totalUSD} />
        </div>
      ) : (
        <div className="h-6 w-28 my-1 bg-surface-secondary rounded-sm" />
      )}
      {isError && (
        <div className="text-xs text-content/60">
          Some balances could not be loaded.
        </div>
      )}
    </div>
  );
};

export default WalletTotal;
