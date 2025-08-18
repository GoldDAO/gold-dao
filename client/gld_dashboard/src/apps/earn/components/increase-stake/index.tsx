import { HTMLAttributes, PropsWithChildren, useEffect } from "react";
import { UseQueryResult } from "@tanstack/react-query";
import { useAtom } from "jotai";
import { GLDT_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/DialogV2";
import { LoaderSpin } from "@components/loaders";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import IncreaseStakeButton from "./components/IncreaseStakeButton";
import { IncreaseStakeStateReducerAtom } from "./atoms";
import Form from "./components/form";
import Details from "./components/details";
import { Position } from "@earn/interfaces";

interface IncreaseStakeProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  position: UseQueryResult<Position, Error>;
}

const IncreaseStake = ({ position, ...props }: IncreaseStakeProps) => {
  const { isConnected, principalId, unauthenticatedAgent } = useAuth();
  const [state, dispatch] = useAtom(IncreaseStakeStateReducerAtom);

  const balance = useFetchLedgerBalance(
    GLDT_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "GLDT",
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  useEffect(() => {
    if (!state.is_open_dialog) {
      const timeoutId = setTimeout(() => {
        dispatch({
          type: "RESET",
        });
      }, 300);

      return () => clearTimeout(timeoutId);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state.is_open_dialog]);

  useEffect(() => {
    if (position.isSuccess && balance.isSuccess) {
      dispatch({
        type: "SET_USER_BALANCE_GLDT",
        value: balance.data,
      });
      dispatch({
        type: "SET_USER_STAKED_DATA",
        value: {
          staked_amount: position.data.staked_amount,
          staked_amount_e8s: position.data.staked_amount_e8s,
          staked_amount_usd: position.data.staked_amount_usd,
        },
      });
      if (state.step === "init") {
        dispatch({
          type: "SET_STEP",
          value: "form",
        });
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    balance.data,
    balance.isSuccess,
    position.data,
    position.isSuccess,
    state.step,
  ]);

  const onOpenDialog = () => {
    dispatch({
      type: "SET_IS_OPEN_DIALOG",
      value: true,
    });
  };

  const onCloseDialog = () => {
    dispatch({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <div {...props}>
      {!isConnected ? (
        <IncreaseStakeButton disabled={true} />
      ) : (
        <IncreaseStakeButton handleOnClick={onOpenDialog} />
      )}
      <Dialog open={state.is_open_dialog} onClose={onCloseDialog}>
        <div className="flex items-center justify-end mb-4">
          <Dialog.CloseBtn onClick={onCloseDialog} />
        </div>
        {state.step === "init" && (
          <div className="flex flex-col items-center gap-4">
            <div className="flex flex-col items-center justify-center gap-6">
              <LoaderSpin />
              <div>Loading data...</div>
            </div>
          </div>
        )}
        {state.step === "form" && <Form />}
        {state.step === "details" && <Details />}
      </Dialog>
    </div>
  );
};

export default IncreaseStake;
