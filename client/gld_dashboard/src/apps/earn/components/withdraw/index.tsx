import { HTMLAttributes, PropsWithChildren, useEffect } from "react";
import { useAtom } from "jotai";
import { UseQueryResult } from "@tanstack/react-query";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/DialogV2";
import { WithdrawStateReducerAtom } from "./atoms";
import Confirm from "./components/confirm";
import Details from "./components/details";
import { Position } from "@earn/interfaces";
import BtnPrimary from "@shared/ui/button/HorizontalButton";

interface WithdrawProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  position: UseQueryResult<Position, Error>;
}

const Withdraw = ({ position, ...props }: WithdrawProps) => {
  const { isConnected } = useAuth();
  const [state, dispatch] = useAtom(WithdrawStateReducerAtom);

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

  const onOpen = (position: Position) => {
    dispatch({
      type: "SET_DISSOLVED_DATA",
      value: position.dissolve_events,
    });
    dispatch({
      type: "SET_TOTAL_AMOUNT",
      value: position.staked_amount + position.total_withdrawable_amount,
    });
    dispatch({
      type: "SET_IS_OPEN_DIALOG",
      value: true,
    });
  };

  const onClose = () => {
    dispatch({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  const renderBtn = () => {
    if (!isConnected) return <BtnPrimary disabled={true}>Withdraw</BtnPrimary>;
    if (position.isLoading || position.isError)
      return (
        <BtnPrimary disabled={true} className="animate-pulse">
          Withdraw
        </BtnPrimary>
      );
    if (
      position.isSuccess &&
      position.data.is_enable_withdrawing &&
      !position.data.is_enable_claiming_rewards
    ) {
      return (
        <BtnPrimary onClick={() => onOpen(position.data)}>Withdraw</BtnPrimary>
      );
    }
    return <BtnPrimary disabled={true}>Withdraw</BtnPrimary>;
  };

  return (
    <div {...props}>
      {renderBtn()}
      <Dialog size="sm" open={state.is_open_dialog} onClose={onClose}>
        <div className="flex items-center justify-end mb-8">
          <Dialog.CloseBtn onClick={onClose} />
        </div>

        {state.is_step_confirm && <Confirm />}
        {state.is_step_details && <Details />}
      </Dialog>
    </div>
  );
};

export default Withdraw;
