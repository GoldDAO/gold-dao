import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { IncreaseStakeStateReducerAtom } from "../../atoms";
import Button from "@shared/ui/button/HorizontalButton";
import useAddStake from "./hooks/useAddStake";
import { LoaderSpin } from "@components/loaders";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Logo } from "@shared/ui/logos";
// import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const DetailsDialogContent = () => {
  const { authenticatedAgent } = useAuth();
  const [stakeState, dispatchStakeState] = useAtom(
    IncreaseStakeStateReducerAtom
  );

  const addStake = useAddStake(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  const handleAddStake = () => {
    if (!stakeState.user_balance_gldt) return;
    const stakeAmount = BigInt(
      Math.round(
        Number(stakeState.stake_amount) *
          10 ** stakeState.user_balance_gldt.decimals
      )
    );
    const amount = stakeAmount + 1n * stakeState.user_balance_gldt.fee_e8s;
    addStake.mutate({
      amount,
    });
  };

  useEffect(() => {
    if (addStake.isIdle && stakeState.step === "details") {
      handleAddStake();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [addStake.isIdle, stakeState.step]);

  useEffect(() => {
    return () => {
      addStake.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onRetry = () => {
    addStake.reset();
    handleAddStake();
  };

  const onClose = () => {
    dispatchStakeState({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <div>
      {(addStake.isIdle || addStake.isPending) && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-6">
            <LoaderSpin />
            <div>Adding stake....</div>
          </div>
        </div>
      )}
      {addStake.isError && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div className="mb-2">Add stake error</div>
            <div className="text-content/60 text-center">
              {addStake.error.message}
            </div>
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
      {addStake.isSuccess && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-4">
            <div className="text-2xl">You successfully staked</div>

            <div className="flex items-center gap-1 font-semibold text-4xl">
              <Logo name="gldt" className="w-8" />
              <NumberToLocaleString value={Number(stakeState.stake_amount)} />
              <div>GLDT</div>
            </div>
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
