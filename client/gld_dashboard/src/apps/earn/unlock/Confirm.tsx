import clsx from "clsx";
import { useAtom } from "jotai";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { UnlockStateReducerAtom } from "./atoms";
import useFetchUserStakeById from "@services/gldt_stake/hooks/useFetchUserStakeById";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";
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
      <div className="flex justify-center items-center px-4 py-16 xl:py-32">
        Loading...
      </div>
    );
  }

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        <div
          className={clsx(
            "p-4 xl:p-8 border border-border cursor-pointer rounded-lg",
            "grid grid-cols-1 gap-4",
            `${
              stateUnlock.unlock_type === "DISSOLVE"
                ? "bg-gold/10 border-gold"
                : "bg-surface border-border"
            }`
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
              `${
                stateUnlock.unlock_type === "DISSOLVE"
                  ? "border-gold/40"
                  : "border-border"
              }`
            )}
          >
            When you start unlocking, you will receive your GLDT liquid in your
            wallet in 1 week.
          </div>
          <div
            className={clsx(
              "p-4 border bg-surface-primary rounded-lg",
              `${
                stateUnlock.unlock_type === "DISSOLVE"
                  ? "border-gold/40"
                  : "border-border"
              }`
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
            "p-4 xl:p-8 border border-border cursor-pointer rounded-lg",
            "grid grid-cols-1 gap-4",
            `${
              stateUnlock.unlock_type === "UNSTAKE_EARLY"
                ? "bg-gold/10 border-gold"
                : "bg-surface-primary border-border"
            }`
          )}
        >
          <div className="text-xl">Unlock immediately</div>
          <div
            className={clsx(
              "p-4 border bg-surface-primary rounded-lg",
              `${
                stateUnlock.unlock_type === "UNSTAKE_EARLY"
                  ? "border-gold/40"
                  : "border-border"
              }`
            )}
          >
            When unlocking immediately, you will receive your GLDT immediately
            but are charged a 5% fee on your GLDT stake.
          </div>
          <div className="bg-surface-primary rounded-lg">
            <div
              className={clsx(
                "p-4 border rounded-lg",
                "bg-warning/10 dark:bg-surface-primary border-warning/60 text-warning"
              )}
            >
              You will be charged 50 GLDT and will only receive 950 GLDT.
            </div>
          </div>
        </div>
      </div>
      <div className="flex justify-center">
        <BtnPrimary onClick={() => dispatch({ type: "CONFIRM" })}>
          Confirm
        </BtnPrimary>
      </div>
    </>
  );
};

export default Confirm;
