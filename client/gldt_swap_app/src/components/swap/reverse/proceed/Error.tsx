import { useNavigate } from "react-router-dom";
import { Button } from "@components/ui";
import { useReverseSwapProceed } from "@context/index";

const Error = () => {
  const navigate = useNavigate();
  const { handleClose, reverseSwap } = useReverseSwapProceed();
  const { reset, error } = reverseSwap;
  const approveError = error?.message === "Approve";
  const swapError = error?.message === "Swap";

  const handleOnClose = () => {
    reset();
    handleClose();
  };

  const handleRetry = () => {
    reset();
  };

  const handleOnClickGoToTxView = () => {
    handleClose();
    navigate("/swap/account#active-swaps");
  };

  const renderErrorMessage = () => {
    switch (error?.message) {
      case "Approve":
        return "Reverse swap error! Approve transactions failed.";
      case "Swap":
        return "Reverse swap error! Swap tokens for NFT failed.";
      default:
        return error?.message;
    }
  };

  return (
    <div className="flex flex-col items-center">
      <div className="border border-red-400 bg-red-400/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl w-full mb-8">
        <div className="text-red-400 font-semibold text-center">
          {renderErrorMessage()}
        </div>
      </div>
      <div className="flex items-center">
        <Button className="mr-4" onClick={handleOnClose}>
          Close
        </Button>
        {approveError || swapError ? (
          <Button onClick={handleRetry}>Retry</Button>
        ) : (
          <Button onClick={handleOnClickGoToTxView}>
            Go to transactions history
          </Button>
        )}
      </div>
    </div>
  );
};

export default Error;
