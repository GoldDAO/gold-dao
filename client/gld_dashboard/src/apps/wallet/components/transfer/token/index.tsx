import { useAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";
import { ChevronLeftIcon } from "@heroicons/react/20/solid";
import Dialog from "@components/dialogs/Dialog";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "@wallet/atoms/TransferTokenAtom";
import Form from "./form";
import Confirm from "./confirm/Confirm";
import ReceiveAddress from "@wallet/components/transfer/receive-address";

const TransferTokenDialog = () => {
  const [transferState, setTransferState] = useAtom(TransferTokenStateAtom);
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);

  const { is_open_transfer_dialog, transfer_tab } = transferState;
  const { is_step_send_form, is_step_send_confirm } = sendState;

  const handleOnChangeTab = (tab: "send" | "receive") => {
    setTransferState((state) => ({
      ...state,
      transfer_tab: tab,
    }));
  };

  const handleBackConfirm = () => {
    setSendState((state) => ({
      ...state,
      is_step_send_confirm: false,
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
      size="xxl"
      title={
        is_step_send_confirm && (
          <div
            className={clsx(
              "p-1 rounded-full cursor-pointer",
              "hover:bg-secondary hover:text-white"
            )}
            onClick={handleBackConfirm}
          >
            <ChevronLeftIcon className="h-6 w-6" />
          </div>
        )
      }
    >
      {is_step_send_form && (
        <div className="flex justify-center items-center gap-4">
          <div onClick={() => handleOnChangeTab("receive")}>Receive</div>
          <div onClick={() => handleOnChangeTab("send")}>Send</div>
        </div>
      )}
      <div className="mt-4">
        {transfer_tab === "receive" && <ReceiveAddress />}
        {transfer_tab === "send" && (
          <>
            {is_step_send_form && <Form />}
            {is_step_send_confirm && <Confirm />}
          </>
        )}
      </div>
    </Dialog>
  );
};

export default TransferTokenDialog;
