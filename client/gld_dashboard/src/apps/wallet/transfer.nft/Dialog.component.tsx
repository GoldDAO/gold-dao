import { useAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";
import { ChevronLeftIcon } from "@heroicons/react/20/solid";

import Dialog from "@components/dialogs/Dialog";

import { TransferStateAtom, SendStateAtom } from "./atoms";

import Form from "./Dialog.form.component";
// import TransferTokenConfirm from "./Transfer.confirm.component";

const TransferNFTDialog = () => {
  const [transferState, setTransferState] = useAtom(TransferStateAtom);
  const [sendState, setSendState] = useAtom(SendStateAtom);

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
      <div className="mt-8">
        {transfer_tab === "receive" && <div>Receive</div>}
        {transfer_tab === "send" && (
          <>
            {is_step_send_form && <Form />}
            {/* {is_step_send_confirm && <TransferTokenConfirm />} */}
          </>
        )}
      </div>
    </Dialog>
  );
};

export default TransferNFTDialog;
