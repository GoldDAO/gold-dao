import { useAtom } from "jotai";
import clsx from "clsx";
import { ChevronLeftIcon } from "@heroicons/react/20/solid";
import Dialog from "@components/dialogs/Dialog";
import { TransferNFTStateReducerAtom } from "@wallet/atoms/TransferNFTAtom";
import { SelectNFTStateReducerAtom } from "@wallet/atoms/NFTStateAtom";
import SendForm from "./Form";
import SendConfirm from "./Confirm";
import ReceiveAddress from "@wallet/components/transfer/receive-address";

const TransferNFTDialog = () => {
  const [transferState, dispatchTransferState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const {
    is_open_transfer_dialog,
    transfer_tab,
    is_step_send_form,
    is_step_send_confirm,
  } = transferState;

  const handleOnClose = () => {
    dispatchTransferState({ type: "RESET" });
    dispatchSelectNFTState({ type: "RESET" });
  };

  return (
    <Dialog
      open={is_open_transfer_dialog}
      handleOnClose={handleOnClose}
      title={
        is_step_send_confirm && (
          <div
            className={clsx(
              "p-1 rounded-full cursor-pointer",
              "hover:bg-secondary hover:text-white"
            )}
            onClick={() =>
              dispatchTransferState({ type: "CANCEL_SEND_CONFIRM" })
            }
          >
            <ChevronLeftIcon className="h-6 w-6" />
          </div>
        )
      }
    >
      {is_step_send_form && (
        <div className="flex justify-center items-center gap-4">
          <div
            onClick={() =>
              dispatchTransferState({ type: "SET_TAB", value: "receive" })
            }
          >
            Receive
          </div>
          <div
            onClick={() =>
              dispatchTransferState({ type: "SET_TAB", value: "send" })
            }
          >
            Send
          </div>
        </div>
      )}
      <div className="mt-8">
        {transfer_tab === "receive" && <ReceiveAddress />}
        {transfer_tab === "send" && (
          <>
            {is_step_send_form && <SendForm />}
            {is_step_send_confirm && <SendConfirm />}
          </>
        )}
      </div>
    </Dialog>
  );
};

export default TransferNFTDialog;
