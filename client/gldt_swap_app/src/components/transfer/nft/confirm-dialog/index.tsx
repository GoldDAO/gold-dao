import { Dialog } from "@components/ui";
import { useTransferProceedNft } from "@context/transfer/proceed-nft";
import Confirm from "./Confirm";
import ConfirmPending from "./ConfirmPending";
import ConfirmSuccess from "./ConfirmSuccess";
import ConfirmError from "./ConfirmError";

const ConfirmDialog = () => {
  const { show, handleCloseDialogConfirm, mutation } = useTransferProceedNft();

  const { isSuccess, isError, isPending, isIdle } = mutation;

  return (
    <>
      <Dialog
        show={show}
        handleClose={handleCloseDialogConfirm}
        enableClose={isIdle}
      >
        <div className="px-12 py-8">
          {isIdle && show && <Confirm />}
          {isPending && <ConfirmPending />}
          {isSuccess && <ConfirmSuccess />}
          {isError && <ConfirmError />}
        </div>
      </Dialog>
    </>
  );
};

export default ConfirmDialog;
