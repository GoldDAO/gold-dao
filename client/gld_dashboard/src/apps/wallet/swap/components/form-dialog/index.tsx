// import { useAtom } from "jotai";
// import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import Dialog from "@shared/ui/dialog/DialogV2";

import SendForm from "./send-form";
import ReceiveForm from "./receive-form";

const FormDialog = ({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) => {
  // const [swapState] = useAtom(SwapStateReducerAtom);

  return (
    <Dialog open={open} onClose={onClose}>
      <div className="flex items-center justify-between mb-8">
        <div>Swap</div>
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      <div className="flex flex-col gap-4">
        <SendForm />
        <ReceiveForm />
      </div>
    </Dialog>
  );
};

export default FormDialog;
