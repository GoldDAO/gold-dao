import { HTMLAttributes, PropsWithChildren, useEffect } from "react";
import { useAtom } from "jotai";
import { UseQueryResult } from "@tanstack/react-query";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/DialogV2";
import { LoaderSpin } from "@components/loaders";
import DecreaseStakeButton from "./components/DecreaseStakeButton";
import { DecreaseStakeStateReducerAtom } from "./atoms";
import Form from "./components/form";
import Confirm from "./components/confirm";
import DetailsDissolving from "./components/details-dissolving";
import DetailsDissolvingInstantly from "./components/details-dissolving-instantly";
import Icon from "@shared/ui/icons";
import { Position } from "@earn/interfaces";
import { GLDT_LEDGER_CANISTER_ID } from "@constants";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";

interface DecreaseStakeProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  position: UseQueryResult<Position, Error>;
}

const DecreaseStake = ({ position, ...props }: DecreaseStakeProps) => {
  const { isConnected, unauthenticatedAgent, principalId } = useAuth();
  const [state, dispatch] = useAtom(DecreaseStakeStateReducerAtom);

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
    if (balance.isSuccess && position.isSuccess) {
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
          instant_dissolve_fee: position.data.instant_dissolve_fee,
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
    position.data,
    position.isSuccess,
    state.step,
    balance.data,
    balance.isSuccess,
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

  const onBackForm = () => {
    dispatch({
      type: "SET_STEP",
      value: "form",
    });
  };

  return (
    <div {...props}>
      {!isConnected ? (
        <DecreaseStakeButton disabled={true} />
      ) : (
        <DecreaseStakeButton handleOnClick={onOpenDialog} />
      )}
      <Dialog open={state.is_open_dialog} onClose={onCloseDialog}>
        <div className="mb-4">
          {state.step === "confirm" ? (
            <div className="flex items-center justify-between">
              <button className="cursor-pointer" onClick={onBackForm}>
                <Icon.Chevron width={16} className="rotate-90" />
              </button>
              <Dialog.CloseBtn onClick={onCloseDialog} />
            </div>
          ) : (
            <div className="flex items-center justify-end">
              <Dialog.CloseBtn onClick={onCloseDialog} />
            </div>
          )}
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
        {state.step === "confirm" && <Confirm />}
        {state.step === "details_dissolving" && <DetailsDissolving />}
        {state.step === "details_dissolving_instantly" && (
          <DetailsDissolvingInstantly />
        )}
      </Dialog>
    </div>
  );
};

export default DecreaseStake;
