import { useAtom } from "jotai";
import {
  SwapStateReducerAtom,
  IsReceiveAmountValidAtom,
} from "@wallet/token/swap/atoms";
import { ErrorMessage } from "@hookform/error-message";

const BtnFormText = () => {
  const [swapState] = useAtom(SwapStateReducerAtom);
  const [isReceiveAmountValid] = useAtom(IsReceiveAmountValidAtom);
  const { errors } = swapState.form_state;

  const renderBtnFormText = () => {
    if (!isReceiveAmountValid) {
      return "Receive amount is zero, please increase the send amount.";
    }
    if (
      errors.amount?.type === "isInsufficientFunds" ||
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
      return "Swap";
    }
  };

  return renderBtnFormText();
};

export default BtnFormText;
