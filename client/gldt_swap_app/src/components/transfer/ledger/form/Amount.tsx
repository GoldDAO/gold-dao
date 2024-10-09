import { ArrowUpTrayIcon } from "@heroicons/react/24/outline";
import { useTransferProceedLedger } from "@context/transfer/proceed-ledger";

import { divideBy1e8, numberToE8s } from "@utils/numbers";
import Input from "@components/ui/form/Input";

const Amount = ({
  balance,
  transactionFee,
  className,
}: {
  className?: string;
  transactionFee: number;
  balance: { e8s: number; number: number; string: string };
}) => {
  const { form, state } = useTransferProceedLedger();
  const { ledger } = state;
  const {
    register,
    setValue,
    setFocus,
    formState: { errors },
  } = form;

  const isAmountUpperFee = (value: string) => {
    const nValue = Number(value);
    if (nValue && nValue > 0) {
      const amount = numberToE8s(value);
      if (amount < transactionFee) return false;
    }
    return true;
  };

  const isAmountUnderBalance = (value: string) => {
    const nValue = Number(value);
    if (balance && nValue && nValue > 0) {
      const bBalance = balance.e8s;
      const amount = numberToE8s(value);
      if (amount > bBalance) return false;
    }
    return true;
  };

  const handleSetAmountMaxBalance = () => {
    const value = divideBy1e8(balance.e8s);
    setValue("amount", value > 0 ? value : 0, {
      shouldValidate: true,
    });
    setFocus("amount");
  };

  return (
    <div className={`${className} text-center sm:text-left`}>
      <label htmlFor="amount" className="text-gold text-sm font-semibold mb-2">
        Amount
      </label>
      <div className="relative">
        <Input
          className="px-4 py-3 mt-2 mb-1 bg-surface-2 border border-border rounded-lg w-full text-center"
          placeholder={`100 ${ledger}`}
          id="amount"
          type="text"
          {...register("amount", {
            pattern: /[0-9.]/,
            required: "Amount is required.",
            validate: {
              isAmountUnderBalance: (v) =>
                isAmountUnderBalance(v) ||
                "Amount must not exceed your balance.",
              isAmountUpperFee: (v) =>
                isAmountUpperFee(v) ||
                "Amount must not be less than transaction fee.",
              isPositive: (v) =>
                Number(v) > 0 || "Amount must be a positive number.",
            },
          })}
          errors={errors?.amount}
        />
        <button
          onClick={handleSetAmountMaxBalance}
          type="button"
          className="absolute right-5 top-5 border border-border bg-surface text-gold text-sm px-2 py-1 rounded-lg"
        >
          <div className="flex items-center">
            <ArrowUpTrayIcon className="h-4 w-4 mr-2" />
            <div className="font-semibold">Max</div>
          </div>
        </button>
      </div>
    </div>
  );
};

export default Amount;
