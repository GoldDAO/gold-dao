import { useAtom } from "jotai";
import Button from "@shared/ui/button/HorizontalButton";
import { WithdrawStateReducerAtom } from "../../atoms";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ConfirmDialogContent = () => {
  const [state, dispatch] = useAtom(WithdrawStateReducerAtom);

  const handleConfirm = () => {
    dispatch({
      type: "SET_IS_STEP_DETAILS",
    });
  };

  return (
    <div>
      <div className="text-center">
        You are about to withdraw{" "}
        <NumberToLocaleString value={state.dissolved_data.amount} /> GLDT{" "}
        <span className="text-content/60">
          ($
          <NumberToLocaleString value={state.dissolved_data.amount_usd} />)
        </span>{" "}
        from {state.dissolved_data.position_count} dissolve{" "}
        {state.dissolved_data.position_count <= 1 ? "position" : "positions"}.
      </div>
      <Button className="mt-6 w-full" onClick={handleConfirm}>
        Confirm
      </Button>
    </div>
  );
};

export default ConfirmDialogContent;
