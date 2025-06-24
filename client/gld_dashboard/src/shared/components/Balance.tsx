import { Logo } from "@components/logos";
import NumberToLocaleString from "./numbers/NumberToLocaleString";

const Balance = ({
  ledger,
  balance,
  className = "",
}: {
  ledger: string;
  balance: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="inline-flex justify-start items-center px-2 py-1 bg-surface-secondary text-content/60 text-xs rounded-lg">
        <div>Your balance: </div>
        <div className="flex items-center font-semibold gap-2 mx-2">
          <Logo name={ledger.toLocaleLowerCase()} className="h-4 w-4" />
          <div>
            <NumberToLocaleString value={balance} /> {ledger}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Balance;
