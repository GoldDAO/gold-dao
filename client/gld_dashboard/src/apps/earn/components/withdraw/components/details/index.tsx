import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { WithdrawStateReducerAtom } from "../../atoms";
import Button from "@shared/ui/button/HorizontalButton";
import useWithdrawing from "./hooks/useWithdrawing";
import { LoaderSpin } from "@components/loaders";
// import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const DetailsDialogContent = () => {
  const { authenticatedAgent } = useAuth();
  const [state, dispatch] = useAtom(WithdrawStateReducerAtom);

  const withdraw = useWithdrawing(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  const onWithdrawing = () => {
    withdraw.mutate();
  };

  useEffect(() => {
    if (withdraw.isIdle && state.is_step_details) {
      onWithdrawing();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [withdraw.isIdle, state.is_step_details]);

  useEffect(() => {
    return () => {
      withdraw.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onRetry = () => {
    withdraw.reset();
    onWithdrawing();
  };

  const onClose = () => {
    dispatch({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <div>
      {(withdraw.isIdle || withdraw.isPending) && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-6">
            <LoaderSpin />
            <div>Withdrawing....</div>
          </div>
        </div>
      )}
      {withdraw.isError && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div className="mb-2">Withdrawing error</div>
            <div className="text-content/60">{withdraw.error.message}</div>
          </div>
          <div className="mt-4 flex items-center gap-2 w-full">
            <Button onClick={onRetry} className="w-full">
              Retry
            </Button>
            <Button onClick={onClose} className="w-full">
              Close
            </Button>
          </div>
        </div>
      )}
      {withdraw.isSuccess && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div>Withdrawing success!</div>
          </div>
          <Button size="lg" onClick={onClose} className="mt-4 w-full">
            Close
          </Button>
        </div>
      )}
    </div>
  );
};

export default DetailsDialogContent;
