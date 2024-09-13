import { LoaderSpin } from "@components/ui";

import { useLedgerUserBalance } from "@hooks/ledger";
import { useBalanceOGYUSD } from "@hooks/ogy_api";

const BalanceOGY = ({ className }: { className?: string }) => {
  const {
    data: balance,
    isSuccess,
    isError,
    isLoading,
  } = useLedgerUserBalance({ ledger: "OGY" });
  const { data: balanceOGYUSD } = useBalanceOGYUSD({
    balance: balance?.number,
  });

  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-6">
        {isSuccess && balance && (
          <div className="flex flex-col sm:flex-row items-center gap-4 justify-center sm:justify-start">
            <div className="flex items-center justify-center sm:justify-start gap-3">
              <img className="flex-none h-8" src={`/ogy_logo.svg`} />
              <div className="font-semibold text-2xl">{balance.string}</div>
              <div className="font-semibold text-2xl">OGY</div>
            </div>
            <div className="font-light text-content/60">={balanceOGYUSD} $</div>
          </div>
        )}
        {(isLoading || isError) && (
          <div className="flex justify-center">
            <LoaderSpin />
          </div>
        )}
      </div>
    </div>
  );
};

export default BalanceOGY;
