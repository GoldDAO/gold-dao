import { Button } from "@components/ui";
import { useForwardSwapProceed } from "@context/index";

const Error = () => {
  const { handleClose, forwardSwap } = useForwardSwapProceed();
  const { reset, error } = forwardSwap;

  const handleRetry = () => {
    reset();
  };

  return (
    <div className="flex flex-col items-center">
      <div className="border border-red-400 bg-red-400/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl w-full mb-8">
        <div className="font-semibold text-red-400">
          Error while swapping your NFT's!
        </div>
        {error?.message && (
          <div className="text-red-400 mt-6">{error?.message}</div>
        )}
      </div>
      <div className="flex items-center">
        <Button className="mr-4" onClick={handleClose}>
          Close
        </Button>
        <Button onClick={handleRetry}>Retry</Button>
      </div>
    </div>
  );
};

export default Error;
