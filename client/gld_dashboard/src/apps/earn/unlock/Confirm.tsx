import clsx from "clsx";
import { useAtom } from "jotai";

import { GLDT_STAKE_CANISTER_ID, GLDT_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { UnlockStateReducerAtom } from "./atoms";
import useFetchUserStakeById from "@services/gldt_stake/hooks/useFetchUserStakeById";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";

const Confirm = () => {
  const { authenticatedAgent, isConnected } = useAuth();
  const [stateUnlock, dispatch] = useAtom(UnlockStateReducerAtom);

  const fee = useFetchTransferFee(GLDT_LEDGER_CANISTER_ID, authenticatedAgent, {
    ledger: "gldt",
    enabled: !!authenticatedAgent && !!isConnected && !!stateUnlock.stake_id,
  });

  const stake = useFetchUserStakeById(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled:
        isConnected &&
        !!authenticatedAgent &&
        fee.isSuccess &&
        !!stateUnlock.stake_id,
      fee: fee.data as bigint,
      id: stateUnlock.stake_id as bigint,
    }
  );

  console.log("stake_id", stateUnlock.stake_id);
  console.log("stake", stake);

  if (!stateUnlock.stake_id || !stake.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 lg:py-32">
        Loading...
      </div>
    );
  }

  return (
    <>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 my-8">
        <div
          className={clsx(
            "p-4 border border-border cursor-pointer",
            `${stateUnlock.unlock_type === "DISSOLVE" ? "bg-secondary text-white" : ""}`
          )}
          onClick={() =>
            dispatch({
              type: "SET_UNLOCK_TYPE",
              value: { unlock_type: "DISSOLVE" },
            })
          }
        >
          Unlock and wait one week
        </div>
        <div
          onClick={() =>
            dispatch({
              type: "SET_UNLOCK_TYPE",
              value: { unlock_type: "UNSTAKE_EARLY" },
            })
          }
          className={clsx(
            "p-4 border border-border cursor-pointer",
            `${stateUnlock.unlock_type === "UNSTAKE_EARLY" ? "bg-secondary text-white" : ""}`
          )}
        >
          Unlock immediately
        </div>
      </div>
      <div className="flex justify-center">
        <Button
          className={clsx("px-4 py-3 rounded-md", "bg-secondary text-white")}
          onClick={() => dispatch({ type: "CONFIRM" })}
        >
          Confirm
        </Button>
      </div>
    </>
  );
};

export default Confirm;
