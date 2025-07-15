import { useAtom } from "jotai";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import Dialog from "@shared/ui/dialog/DialogV2";
import BtnForm from "@shared/ui/button/BtnPrimary";
import SendForm from "./send-form";
import ReceiveForm from "./receive-form";
import ErrorMessageBtnForm from "./error-message-btn-form";

const FormDialog = () => {
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const onSubmit = () => {
    dispatchSwapState({ type: "OPEN_DIALOG_CONFIRM" });
  };

  const onClose = () => {
    dispatchSwapState({ type: "RESET" });
  };

  return (
    <Dialog open={swapState.is_open_form_dialog} onClose={onClose}>
      <div className="flex items-center justify-between mb-8">
        <div>Swap</div>
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      <div className="flex flex-col gap-4">
        <SendForm />
        <ReceiveForm />
        <BtnForm
          size="lg"
          onClick={onSubmit}
          disabled={!swapState.form_state.isValid}
        >
          {swapState.form_state.isValid ? "Swap" : <ErrorMessageBtnForm />}
        </BtnForm>
      </div>
    </Dialog>
  );
};

export default FormDialog;
