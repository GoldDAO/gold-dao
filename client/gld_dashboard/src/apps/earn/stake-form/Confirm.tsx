import { useAtom } from "jotai";

import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

import { StakeStateReducerAtom } from "./atoms";
import { Button } from "@components/index";

const Confirm = () => {
  const [stakeState, dispatch] = useAtom(StakeStateReducerAtom);

  const infos = [
    {
      icon: <div>x</div>,
      title: "Unlock delay",
      description:
        "When unlocking GLDT from staking, the tokens are locked for one week without rewards before they can be withdrawn.",
    },
    {
      icon: <div>x</div>,
      title: "Age bonus",
      description:
        "GLDT stakes start obtaining an age bonus from day one. The older the stakes, the bigger the age bonus, growing linearly at 100% per year.",
    },
    {
      icon: <div>x</div>,
      title: "Rewards",
      description:
        "When you start unlocking your GLDT stake, you will no longer receive new rewards.",
    },
  ];

  if (!stakeState.amount) {
    return (
      <div className="flex justify-center items-center px-4 py-16">
        Loading...
      </div>
    );
  }

  return (
    <div className="mt-8 lg:mt-12">
      <div className="text-center lg:text-2xl font-semibold">
        You are about to create a stake of{" "}
        <div className="text-primary lg:text-4xl mt-2">
          <TokenValueToLocaleString value={stakeState.amount} /> GLDT
        </div>
      </div>
      <div className="grid grid-cols-1 gap-4 mt-4 lg:mt-12">
        {infos.map(({ icon, title, description }, index) => (
          <div key={index}>
            <div className="p-4 grid grid-cols-1 lg:grid-cols-5 border border-border rounded-lg">
              <div className="flex justify-center items-center">
                <div>{icon}</div>
              </div>
              <div className="col-span-4 flex flex-col gap-1">
                <div className="">{title}</div>
                <div className="text-content/60 text-sm">{description}</div>
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
  );
};

export default Confirm;
