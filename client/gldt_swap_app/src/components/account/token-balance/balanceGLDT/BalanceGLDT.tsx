import { LoaderSpin } from "@components/ui";
import { LogoGLDT } from "@components/shared/logos";

import { useLedgerUserBalance } from "@hooks/ledger";
import { useBalanceGLDTUSD } from "@hooks/gold_api";

const BalanceGLDT = ({ className }: { className?: string }) => {
  const {
    data: balance,
    isSuccess,
    isError,
    isLoading,
  } = useLedgerUserBalance({
    ledger: "GLDT",
  });
  const { data: balanceGLTDUSD } = useBalanceGLDTUSD({
    balance: balance?.number,
  });

  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-6">
        {isSuccess && balance && (
          <div className="flex flex-col sm:flex-row items-center justify-center sm:justify-start gap-4">
            <div className="flex items-center justify-center sm:justify-start gap-3">
              <LogoGLDT className="flex-none w-8 h-8" />
              <div className="font-semibold text-2xl">{balance.string}</div>
              <div className="font-semibold text-2xl">GLDT</div>
            </div>
            {balanceGLTDUSD && (
              <div className="font-light text-content/60">
                ={balanceGLTDUSD} $
              </div>
            )}
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

export default BalanceGLDT;
