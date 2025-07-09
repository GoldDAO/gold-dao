import { useAtom } from "jotai";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import Dialog from "@shared/ui/dialog/DialogV2";
import BtnPrimary from "@shared/ui/button/BtnPrimary";

import SendForm from "./send-form";
import ReceiveForm from "./receive-form";
import { ErrorMessage } from "@hookform/error-message";

const FormDialog = ({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) => {
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  return (
    <Dialog open={open} onClose={onClose}>
      <div className="flex items-center justify-between mb-8">
        <div>Swap</div>
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      <div className="flex flex-col gap-4">
        <SendForm />
        <ReceiveForm />
        <BtnPrimary
          size="lg"
          onClick={() => dispatchSwapState({ type: "OPEN_DIALOG_CONFIRM" })}
          disabled={!swapState.form_state.isValid}
        >
          {swapState.form_state.isValid ? (
            "Swap"
          ) : swapState.form_state.errors.amount?.type === "required" ||
            swapState.form_state.errors.amount?.type ===
              "isAmountGreaterThanZero" ? (
            "Swap"
          ) : (
            <ErrorMessage
              errors={swapState.form_state.errors}
              name="amount"
              render={({ message }) => (
                <div className="text-sm font-normal">{message}</div>
              )}
            />
          )}
        </BtnPrimary>
      </div>
    </Dialog>
  );
};

export default FormDialog;
