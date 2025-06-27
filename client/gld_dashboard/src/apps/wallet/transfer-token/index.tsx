import { useAtom } from "jotai";
import { RESET } from "jotai/utils";
import Dialog from "@shared/ui/dialog/Dialog";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "@wallet/shared/atoms/TransferTokenAtom";
import Form from "./form";
import Confirm from "./confirm";
import ReceiveAddress from "@wallet/shared/components/transfer-receive-address";
import SwitchTransfer from "@shared/components/switch/SwitchTransfer";

const TransferTokenDialog = () => {
  const [transferState, setTransferState] = useAtom(TransferTokenStateAtom);
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);

  const { is_open_transfer_dialog, transfer_tab } = transferState;
  const { is_step_send_form, is_step_send_confirm, is_send_confirm } =
    sendState;

  const handleOnChangeTab = (value: "send" | "receive") => {
    setTransferState((state) => ({
      ...state,
      transfer_tab: value,
    }));
  };

  const handleBackConfirm = () => {
    setSendState((state) => ({
      ...state,
      is_step_send_confirm: false,
      is_send_confirm: false,
      is_step_send_form: true,
    }));
  };

  const handleCloseTransferDialog = () => {
    setTransferState(RESET);
  };

  return (
    <Dialog
      open={is_open_transfer_dialog}
      handleOnClose={handleCloseTransferDialog}
      size={transfer_tab === "receive" ? "md" : "xxl"}
      handlePreviousStep={is_send_confirm ? () => handleBackConfirm : undefined}
    >
      {is_step_send_form && (
        <div className="flex justify-center mb-12">
          <SwitchTransfer
            className="flex justify-center mb-12"
            value={transfer_tab}
            handleChange={handleOnChangeTab}
          />
        </div>
      )}
      <>
        {transfer_tab === "receive" && <ReceiveAddress />}
        {transfer_tab === "send" && (
          <>
            {is_step_send_form && <Form />}
            {is_step_send_confirm && <Confirm />}
          </>
        )}
      </>
    </Dialog>
  );
};

export default TransferTokenDialog;
