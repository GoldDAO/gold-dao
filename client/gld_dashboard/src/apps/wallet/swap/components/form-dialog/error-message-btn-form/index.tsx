import { useAtom } from "jotai";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import { ErrorMessage } from "@hookform/error-message";

const ErrorMessageBtnForm = () => {
  const [swapState] = useAtom(SwapStateReducerAtom);

  const renderErrorMessage = () => {
    if (
      swapState.form_state.errors.amount?.type === "isInsufficientFunds" ||
      swapState.form_state.errors.amount?.type === "isAmountGreaterThanFee"
    ) {
      return (
        <ErrorMessage
          errors={swapState.form_state.errors}
          name="amount"
          render={({ message }) => <div className="font-normal">{message}</div>}
        />
      );
    } else {
      return "Swap";
    }
  };

  return renderErrorMessage();
};

export default ErrorMessageBtnForm;
