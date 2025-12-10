import { useAtom } from "jotai";
import Dialog from "@shared/ui/dialog/Dialog";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import Form from "./Form";
import Confirm from "./Confirm";
import ReceiveAddress from "@wallet/shared/components/transfer-receive-address";
import SwitchTransfer from "@shared/components/switch/SwitchTransfer";
import Details from "./Details";

const TransferNFTDialog = () => {
  const [transferState, dispatchTransferState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const { is_open_receive_dialog, transfer_tab } = transferState;

  const handleOnClose = () => {
    dispatchTransferState({ type: "RESET" });
    dispatchSelectNFTState({ type: "RESET" });
  };

  const handleChangeTab = (value: "send" | "receive") => {
    dispatchTransferState({ type: "SET_TAB", value });
  };

  return (
    <>
      <Dialog
        open={is_open_receive_dialog}
        handleOnClose={handleOnClose}
        size="md"
      >
        <SwitchTransfer
          className="flex justify-center mb-12"
          value={transfer_tab}
          handleChange={handleChangeTab}
        />
        <div className="mt-8">
          <ReceiveAddress />
        </div>
      </Dialog>

      <Form />
      <Confirm />
      <Details />
    </>
  );
};

export default TransferNFTDialog;
