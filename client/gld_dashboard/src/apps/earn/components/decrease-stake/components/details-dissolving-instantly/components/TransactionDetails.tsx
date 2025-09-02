import { INSTANT_DISSOLVE_FEE_PERCENTAGE } from "@constants";
import { useAtom } from "jotai";
import { DecreaseStakeStateReducerAtom } from "@earn/components/decrease-stake/atoms";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const TransactionDetails = ({ className }: { className?: string }) => {
  const [state] = useAtom(DecreaseStakeStateReducerAtom);

  return (
    <div className={className}>
      <div className="flex items-center justify-center">
        You were charged{" "}
        <NumberToLocaleString
          value={
            Number(state.unlock_amount) *
            (INSTANT_DISSOLVE_FEE_PERCENTAGE / 100)
          }
        />{" "}
        GLDT for unlocking immediately and received{" "}
        <NumberToLocaleString
          value={
            Number(state.unlock_amount) -
            Number(state.unlock_amount) *
              (INSTANT_DISSOLVE_FEE_PERCENTAGE / 100)
          }
        />{" "}
        GLDT.
      </div>
    </div>
  );
};

export default TransactionDetails;
