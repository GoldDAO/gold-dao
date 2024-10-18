import { FormEvent } from "react";
import { ArrowDownIcon } from "@heroicons/react/20/solid";

import { Button } from "@components/ui";
import { useTransferProceedLedger } from "@context/transfer/proceed-ledger";
import Balance from "../../shared/balance/Balance";
import BalanceAfterTransfer from "../../shared/balance/BalanceAfterTransfer";

const Confirm = () => {
  const { state, handleTransfer } = useTransferProceedLedger();
  const { amount, to, balance, balanceAfterTransfer, ledger } = state;

  const handleOnSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.stopPropagation();
    e.preventDefault();
    handleTransfer();
  };

  return (
    <>
      <div className="flex flex-col items-center gap-6 border border-gold/20 bg-gold/5 p-6 rounded-xl mb-6">
        <div className="font-semibold text-2xl">{amount} OGY</div>

        <div className="w-full flex justify-center items-center py-4">
          <div className="relative w-full">
            <div className="border-t border-border w-full"></div>
            <div className="absolute inset-x-0 top-0 flex justify-center transform -translate-y-1/2">
              <button className="bg-content text-background rounded-full p-2 cursor-default">
                <ArrowDownIcon height={24} width={24} className="text-gold" />
              </button>
            </div>
          </div>
        </div>
        <div className="font-semibold text-2xl text-center">{to}</div>
      </div>

      <BalanceAfterTransfer
        ledger={ledger}
        balance={balanceAfterTransfer as number}
      />

      <form onSubmit={(e) => handleOnSubmit(e)}>
        <Button type="submit" className="mt-8 w-full py-3 rounded-lg">
          Confirm
        </Button>
      </form>

      <div className="flex justify-center mt-6">
        <Balance ledger={ledger} balance={balance as number} />
      </div>
    </>
  );
};

export default Confirm;
