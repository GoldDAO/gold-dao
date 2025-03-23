import { useAtom } from "jotai";

import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import Dialog from "@components/dialogs/Dialog";

import { StakeStateReducerAtom } from "./atoms";
import { Button } from "@components/index";

const ConfirmDialog = () => {
  const [stakeState, dispatch] = useAtom(StakeStateReducerAtom);

  const { is_open_stake_dialog_confirm, amount } = stakeState;

  const infos = [
    {
      icon: <div>Icon</div>,
      title: "Unlock delay of 1 week",
      description:
        "when unlocking GLDT from staking, the tokens are locked for 1 week without rewards before they can be withdrawn.",
    },
    {
      icon: <div>Icon</div>,
      title: "Age Bonus",
      description:
        "GLDT stakes start obtaining an age bonus from day 1. The older the stakes, the bigger the age bonus, growing linearly at 100% per year.",
    },
    {
      icon: <div>Icon</div>,
      title:
        "When you start unlocking your GLDT stake, you will no longer receive new rewards",
    },
  ];

  return (
    <>
      {is_open_stake_dialog_confirm && (
        <Dialog
          open={is_open_stake_dialog_confirm}
          handleOnClose={() => dispatch({ type: "CANCEL" })}
          title="Confirm stake"
        >
          {!amount ? (
            <div className="flex justify-center items-center px-4 py-16 lg:py-32">
              Loading...
            </div>
          ) : (
            <div className="mt-8 lg:mt-12">
              <div className="text-center lg:text-2xl">
                You are about to create a stake of{" "}
                <div className="text-primary lg:text-4xl mt-2">
                  <TokenValueToLocaleString value={amount} /> GLDT
                </div>
              </div>
              <div className="grid grid-cols-1 gap-4 mt-4 lg:mt-12">
                {infos.map(({ icon, title, description }, index) => (
                  <div key={index}>
                    <div className="p-4 grid grid-cols-1 lg:grid-cols-5 border border-border rounded-lg bg-surface-secondary">
                      <div className="flex justify-center items-center">
                        <div>{icon}</div>
                      </div>
                      <div className="col-span-4">
                        <span className="font-semibold">{title}:</span>{" "}
                        <span className="text-content/60">{description}</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
              <Button
                className="mt-8 w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
                type="button"
                onClick={() => dispatch({ type: "CONFIRM" })}
              >
                Confirm
              </Button>
            </div>
          )}
        </Dialog>
      )}
    </>
  );
};

export default ConfirmDialog;
