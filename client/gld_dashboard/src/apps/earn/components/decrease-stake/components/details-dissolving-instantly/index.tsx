import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { DecreaseStakeStateReducerAtom } from "../../atoms";
import Button from "@shared/ui/button/HorizontalButton";
import useDissolving from "./hooks/useDissolvingInstantly";
import { LoaderSpin } from "@components/loaders";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Logo } from "@shared/ui/logos";
import TransactionDetails from "./components/TransactionDetails";

const Details = () => {
  const { authenticatedAgent } = useAuth();
  const [state, dispatch] = useAtom(DecreaseStakeStateReducerAtom);

  const dissolveStake = useDissolving(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  const onDissolving = () => {
    dissolveStake.mutate({ fraction: Number(state.percentage_unlock_amount) });
  };

  useEffect(() => {
    if (dissolveStake.isIdle && state.step === "details_dissolving_instantly") {
      onDissolving();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [dissolveStake.isIdle, state.step]);

  useEffect(() => {
    return () => {
      dissolveStake.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onRetry = () => {
    dissolveStake.reset();
    onDissolving();
  };

  const onClose = () => {
    dispatch({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <div>
      {(dissolveStake.isIdle || dissolveStake.isPending) && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-6">
            <LoaderSpin />
            <div>Dissolving stake....</div>
          </div>
        </div>
      )}
      {dissolveStake.isError && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div className="mb-2">Dissolving stake error</div>
            <div className="text-content/60">{dissolveStake.error.message}</div>
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
      {dissolveStake.isSuccess && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-4">
            <div className="text-2xl">You successfully unlocked</div>

            <div className="flex items-center gap-1 font-semibold text-4xl">
              <Logo name="gldt" className="w-8" />
              <NumberToLocaleString value={Number(state.unlock_amount)} />
              <div>GLDT</div>
            </div>
          </div>
          <TransactionDetails className="mt-4" />
          <Button size="lg" onClick={onClose} className="mt-4 w-full">
            Close
          </Button>
        </div>
      )}
    </div>
  );
};

export default Details;
