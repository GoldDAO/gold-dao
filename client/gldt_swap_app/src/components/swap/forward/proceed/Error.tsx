import { useEffect, useState } from "react";
import { Button } from "@components/ui";
import { useForwardSwapProceed } from "@context/index";

const Error = () => {
  const { handleClose, forwardSwap } = useForwardSwapProceed();
  const { reset, error } = forwardSwap;
  const [retryCountdown, setRetryCountdown] = useState<number | null>(null);
  const isRetryCountdown = retryCountdown !== null && retryCountdown > 0;

  const handleRetry = () => {
    reset();
  };

  useEffect(() => {
    if (error?.message === "Retry") {
      setRetryCountdown((error?.cause as { retryDelay: number }).retryDelay);
    }
  }, [error]);

  useEffect(() => {
    if (retryCountdown !== null && retryCountdown > 0) {
      const intervalId = setInterval(() => {
        setRetryCountdown((prevRetryCountdown) =>
          prevRetryCountdown !== null ? prevRetryCountdown - 1 : null
        );
      }, 1000);
      if (retryCountdown === 0) clearInterval(intervalId);
      return () => clearInterval(intervalId);
    }
  }, [retryCountdown]);

  return (
    <div className="flex flex-col items-center">
      <div className="border border-red-400 bg-red-400/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl w-full mb-8">
        <div className="font-semibold text-red-400 text-center">
          {error?.message === "Retry"
            ? `Error when swapping GLD NFTs! Please retry${
                isRetryCountdown ? ` in ${retryCountdown} seconds` : ""
              }.`
            : error?.message}
        </div>
      </div>
      <div className="flex items-center">
        <Button className="mr-4" onClick={handleClose}>
          Close
        </Button>
        <Button disabled={isRetryCountdown} onClick={handleRetry}>
          Retry
          {isRetryCountdown ? ` (${retryCountdown})` : ""}
        </Button>
      </div>
    </div>
  );
};

export default Error;
