import { useEffect } from "react";
import { useAtom } from "jotai";
import { DecreaseStakeStateReducerAtom } from "@earn/components/decrease-stake/atoms";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const Amount = () => {
  const [stakeState, dispatchStakeState] = useAtom(
    DecreaseStakeStateReducerAtom
  );

  useEffect(() => {
    const unlockAmount =
      stakeState.percentage_unlock_amount > 0
        ? (
            (Number(stakeState.user_staked_data.staked_amount) *
              stakeState.percentage_unlock_amount) /
            100
          ).toString()
        : "0.00";
    dispatchStakeState({
      type: "SET_UNLOCK_AMOUNT",
      value: unlockAmount,
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stakeState.percentage_unlock_amount]);

  return (
    <div className="text-2xl text-content/60">
      {
        <NumberToLocaleString
          value={Number(stakeState.unlock_amount)}
          decimals={8}
        />
      }
    </div>
  );
};

export default Amount;
