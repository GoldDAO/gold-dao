import { Button } from "@components/ui";
import { useReverseSwapProceed } from "@context/index";

const Error = () => {
  const { handleClose, reverseSwap } = useReverseSwapProceed();
  const { reset, error } = reverseSwap;

  const handleRetry = () => {
    reset();
  };

  const handleOnClose = () => {
    reset();
    handleClose();
  };

  return (
    <div className="flex flex-col items-center">
      <div className="border border-red-400 bg-red-400/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl w-full mb-8">
        <div className="font-semibold text-red-400">
          Error while fetching your NFT's!
        </div>
        {error?.message && (
          <div className="text-red-400 mt-6">{error?.message}</div>
        )}
      </div>
      <div className="flex items-center">
        <Button className="mr-4" onClick={handleOnClose}>
          Close
        </Button>
        <Button onClick={handleRetry}>Retry</Button>
      </div>
    </div>
  );
};

export default Error;
