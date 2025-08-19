import { useAtom } from "jotai";
import { IncreaseStakeStateReducerAtom } from "@earn/components/increase-stake/atoms";
import { ErrorMessage } from "@hookform/error-message";

const BtnFormText = () => {
  const [stakeState] = useAtom(IncreaseStakeStateReducerAtom);
  const { errors } = stakeState.form_state;

  const renderBtnFormText = () => {
    if (
      errors.amount?.type === "isInsufficientFunds" ||
      errors.amount?.type === "isAmountGreaterThanFee" ||
      errors.amount?.type === "isAmountGreaterThanMinStakeAmount" ||
      errors.amount?.type === "isAmountLessThanMaxStakeAmount" ||
      errors.amount?.type === "isAmountGreaterThanFee"
    ) {
      return (
        <ErrorMessage
          errors={errors}
          name="amount"
          render={({ message }) => <div className="font-normal">{message}</div>}
        />
      );
    } else {
      return "Confirm stake";
    }
  };

  return renderBtnFormText();
};

export default BtnFormText;
