import { LoaderSpin } from "@components/ui";
import { LogoGLDT } from "@components/shared/logos";

import { useLedgerAccountBalance } from "@hooks/ledger/useLedgerAccountBalance";
import { useBalanceGLDTUSD } from "@hooks/gold_api";

export const AccountBalanceGLDT = ({
  owner,
  subaccount,
  className,
}: {
  owner: string;
  subaccount?: string | undefined;
  className?: string;
}) => {
  const {
    data: accountBalance,
    isSuccess: isSuccessLedgerAccountBalance,
    isLoading: isLoadingLedgerAccountBalance,
    isError: isErrorLedgerAccountBalance,
    // error: errorLedgerAccountBalance,
  } = useLedgerAccountBalance({
    owner,
    subaccount,
  });

  const { data: balanceGLTDUSD } = useBalanceGLDTUSD({
    balance: accountBalance?.number,
  });

  return (
    <div
      className={`border border-border rounded-xl bg-surface p-6 ${className}`}
    >
      <div className="text-center lg:text-left mb-4">Balance</div>
      {isSuccessLedgerAccountBalance && accountBalance && (
        <div className="flex justify-center lg:justify-start">
          <div className="flex flex-col items-center lg:items-end">
            <div className="flex items-center gap-3">
              <LogoGLDT className="flex-none w-8 h-8" />
              <div className="font-semibold text-2xl">
                {accountBalance.string}
              </div>
              <div className="font-semibold text-2xl">GLDT</div>
            </div>
            {balanceGLTDUSD && (
              <div className="font-light text-content/60">
                {balanceGLTDUSD} $
              </div>
            )}
          </div>
        </div>
      )}
      {(isLoadingLedgerAccountBalance || isErrorLedgerAccountBalance) && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};
