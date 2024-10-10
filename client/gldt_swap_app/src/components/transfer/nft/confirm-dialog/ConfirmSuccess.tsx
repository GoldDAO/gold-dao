import { CheckCircleIcon } from "@heroicons/react/24/outline";

import { Button } from "@components/ui";

import { useTransferProceedNft } from "@context/transfer/proceed-nft";

const ConfirmSuccess = () => {
  const { handleCloseDialogConfirm, handleReset } = useTransferProceedNft();

  const handleClose = () => {
    handleCloseDialogConfirm();
    handleReset();
  };

  return (
    <div className="px-4 pb-6 flex flex-col justify-center items-center">
      <CheckCircleIcon className="h-24 w-24 text-gold mb-4" />
      <div className="font-semibold text-xl mb-8">
        Transfer was successful !
      </div>
      <Button className="px-8" onClick={handleClose}>
        Close
      </Button>
    </div>
  );
};

export default ConfirmSuccess;
