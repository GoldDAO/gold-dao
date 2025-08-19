import { useAtom } from "jotai";
import { IncreaseStakeStateReducerAtom } from "../../atoms";
import BalanceAvailable from "@shared/components/BalanceAvailable";
import FormButton from "@shared/ui/button/HorizontalButton";
import FormTextButton from "./components/FormTextButton";
import MaxButton from "@shared/components/MaxButton";
import StakeAmountInput from "./components/StakeAmountInput";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import StakeInformations from "./components/StakeInformations";

const FormDialogContent = () => {
  const [stakeState, dispatchStakeState] = useAtom(
    IncreaseStakeStateReducerAtom
  );
  const { user_balance_gldt } = stakeState;

  const onClickMaxBalance = () => {
    const value =
      Number(user_balance_gldt.balance_e8s - user_balance_gldt.fee_e8s) /
      10 ** user_balance_gldt.decimals;
    dispatchStakeState({
      type: "SET_STAKE_AMOUNT",
      value: value.toString(),
    });
  };

  const onSubmit = () => {
    dispatchStakeState({
      type: "SET_STEP",
      value: "details",
    });
  };

  return (
    <div>
      <div className="p-4 border border-border rounded-xl">
        <div className="text-copper mb-2">Stake GLDT</div>
        <div className="flex flex-col gap-4 p-4 border border-border rounded-xl bg-surface-secondary">
          <div className="flex justify-between items-start">
            <div>
              <div className="text-2xl">
                <StakeAmountInput />
              </div>
              <div className="text-xs text-content/60">
                <div>
                  ≈$
                  <NumberToLocaleString
                    value={
                      Number(stakeState.stake_amount) *
                      stakeState.user_balance_gldt.price_usd
                    }
                  />
                </div>
              </div>
            </div>
          </div>
          <div className="flex justify-between items-center">
            <div className="text-sm text-content/80">
              <BalanceAvailable
                token="GLDT"
                balance={stakeState.user_balance_gldt.balance}
              />
            </div>
            <MaxButton
              disabled={
                user_balance_gldt.balance_e8s < user_balance_gldt.fee_e8s
              }
              handleOnClick={onClickMaxBalance}
            />
          </div>
        </div>
      </div>

      <StakeInformations className="mt-4" />

      <FormButton
        size="lg"
        onClick={onSubmit}
        disabled={!stakeState.form_state.isValid}
        className="mt-6 w-full"
      >
        <FormTextButton />
      </FormButton>
    </div>
  );
};
export default FormDialogContent;
