import clsx from "clsx";
import { useAtom } from "jotai";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { UnlockStateReducerAtom } from "./atoms";
import useFetchUserStakeById from "@services/gldt_stake/hooks/useFetchUserStakeById";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";

const Confirm = () => {
  const { authenticatedAgent, isConnected } = useAuth();
  const [stateUnlock, dispatch] = useAtom(UnlockStateReducerAtom);

  const stake = useFetchUserStakeById(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled:
        isConnected &&
        !!authenticatedAgent &&
        stateUnlock.stake_id !== undefined,
      id: stateUnlock.stake_id as bigint,
    }
  );

  if (!stake.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 lg:py-32">
        Loading...
      </div>
    );
  }

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        <div
          className={clsx(
            "p-4 lg:p-8 border border-border cursor-pointer rounded-lg",
            "grid grid-cols-1 gap-4",
            `${stateUnlock.unlock_type === "DISSOLVE" ? "bg-primary/10 border-primary" : "bg-surface border-border"}`
          )}
          onClick={() =>
            dispatch({
              type: "SET_UNLOCK_TYPE",
              value: { unlock_type: "DISSOLVE" },
            })
          }
        >
          <div className="text-xl">Unlock and wait one week</div>
          <div
            className={clsx(
              "p-4 border bg-surface-primary rounded-lg",
              `${stateUnlock.unlock_type === "DISSOLVE" ? "border-primary/40" : "border-border"}`
            )}
          >
            When you start unlocking, you will receive your GLDT liquid in your
            wallet in 1 week.
          </div>
          <div
            className={clsx(
              "p-4 border bg-surface-primary rounded-lg",
              `${stateUnlock.unlock_type === "DISSOLVE" ? "border-primary/40" : "border-border"}`
            )}
          >
            During this time, you are not receiving any new rewards.
          </div>
        </div>
        <div
          onClick={() =>
            dispatch({
              type: "SET_UNLOCK_TYPE",
              value: { unlock_type: "UNSTAKE_EARLY" },
            })
          }
          className={clsx(
            "p-4 lg:p-8 border border-border cursor-pointer rounded-lg",
            "grid grid-cols-1 gap-4",
            `${stateUnlock.unlock_type === "UNSTAKE_EARLY" ? "bg-primary/10 border-primary" : "bg-surface-primary border-border"}`
          )}
        >
          <div className="text-xl">Unlock immediately</div>
          <div
            className={clsx(
              "p-4 border bg-surface-primary rounded-lg",
              `${stateUnlock.unlock_type === "UNSTAKE_EARLY" ? "border-primary/40" : "border-border"}`
            )}
          >
            When unlocking immediately, you will receive your GLDT immediately
            but are charged a 5% fee on your GLDT stake.
          </div>
          <div className="bg-surface-primary rounded-lg">
            <div
              className={clsx(
                "p-4 border rounded-lg",
                "bg-amber-100/10 dark:bg-surface-primary border-amber-700/60 text-amber-700"
              )}
            >
              You will be charged 50 GLDT and will only receive 950 GLDT.
            </div>
          </div>
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
