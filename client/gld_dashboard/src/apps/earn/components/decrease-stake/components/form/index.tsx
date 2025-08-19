import { useAtom } from "jotai";
import { DecreaseStakeStateReducerAtom } from "../../atoms";
import BalanceAvailable from "@shared/components/BalanceAvailable";
import FormButton from "@shared/ui/button/HorizontalButton";
import ErrorMessage from "./components/ErrorMessage";
// import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import UnlockAmountSlider from "./components/AmountSlider";
import UnlockAmount from "./components/Amount";
import Icon from "@shared/ui/icons";

const Form = () => {
  const [state, dispatch] = useAtom(DecreaseStakeStateReducerAtom);

  const onSubmit = () => {
    dispatch({
      type: "SET_STEP",
      value: "confirm",
    });
  };

  const onChangeUnlockAmount = (value: number) => {
    const unlockAmount =
      (Number(state.user_staked_data.staked_amount) * value) / 100;
    dispatch({
      type: "SET_PERCENTAGE_UNLOCK_AMOUNT",
      value,
    });
    dispatch({
      type: "SET_UNLOCK_AMOUNT",
      value: unlockAmount.toString(),
    });
  };

  return (
    <div>
      <div className="p-4 border border-border rounded-xl">
        <div className="flex justify-between items-center mb-4">
          <div className="text-copper">Unlock amount</div>
          <div className="text-lg font-semibold">
            {state.percentage_unlock_amount}%
          </div>
        </div>
        <div className="flex flex-col gap-4 p-4 border border-border rounded-xl bg-surface-secondary">
          <div>
            <UnlockAmountSlider
              value={Number(state.percentage_unlock_amount)}
              handleOnChange={onChangeUnlockAmount}
            />
          </div>
          <UnlockAmount />
          <div className="text-sm text-content/60">
            <BalanceAvailable
              token="GLDT"
              balance={state.user_staked_data.staked_amount}
              icon={<Icon.CircleStack width={14} />}
            />
          </div>
        </div>
      </div>
      <FormButton
        size="lg"
        onClick={onSubmit}
        disabled={
          Number(state.unlock_amount) <= 0 ||
          state.user_staked_data.remaining_dissolve_events === 0
        }
        className="mt-4 w-full"
      >
        <ErrorMessage />
      </FormButton>
      {state.user_staked_data.remaining_dissolve_events === 0 && (
        <div className="flex justify-center items-center text-sm text-content/60 mt-4">
          Need to wait until one is dissolved before moving forward.
        </div>
      )}
    </div>
  );
};

export default Form;
