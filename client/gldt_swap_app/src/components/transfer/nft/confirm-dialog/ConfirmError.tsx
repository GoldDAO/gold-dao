import { XCircleIcon } from "@heroicons/react/24/outline";

import { Button } from "@components/ui";
import { useTransferProceedNft } from "@context/transfer/proceed-nft";

const FormError = () => {
  const { mutation, handleCloseDialogConfirm, handleReset } =
    useTransferProceedNft();
  const { error, reset } = mutation;

  const handleRetry = () => {
    reset();
  };

  const handleOnClose = () => {
    handleReset();
    handleCloseDialogConfirm();
  };

  return (
    <div className="px-4 pb-6 flex flex-col justify-center items-center">
      <XCircleIcon className="h-24 w-24 text-dark-orange mb-4" />
      <div className="font-semibold text-xl mb-2">Transfer error !</div>
      <div className="max-w-md overflow-x-auto text-center">
        {error?.message}
      </div>
      <div className="flex items-center mt-6">
        <Button className="mr-2" onClick={handleOnClose}>
          Close
        </Button>
        <Button onClick={handleRetry}>Retry</Button>
      </div>
    </div>
  );
};

export default FormError;
