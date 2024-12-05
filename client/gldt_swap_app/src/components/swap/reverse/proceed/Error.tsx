import { useNavigate } from "react-router-dom";
import { Button } from "@components/ui";
import { useReverseSwapProceed } from "@context/index";

const Error = () => {
  const navigate = useNavigate();
  const { handleClose, reverseSwap } = useReverseSwapProceed();
  const { error } = reverseSwap;
  const partialFailureError = error?.message === "swap_partial_failure";

  const handleOnClickGoToTxView = () => {
    handleClose();
    navigate("/swap/account#active-swaps");
  };

  const renderErrorMessage = () => {
    if (partialFailureError) return error.cause as string;
    return error?.message;
  };

  return (
    <div className="flex flex-col items-center">
      <div className="border border-red-400 bg-red-400/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl w-full mb-8">
        <div className="text-red-400 font-semibold text-center">
          {renderErrorMessage()}
        </div>
      </div>
      <div className="flex items-center">
        <Button className="mr-4" onClick={handleClose}>
          Close
        </Button>
        {partialFailureError && (
          <Button onClick={handleOnClickGoToTxView}>
            Go to transactions history
          </Button>
        )}
      </div>
    </div>
  );
};

export default Error;
