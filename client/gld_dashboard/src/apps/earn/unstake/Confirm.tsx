import clsx from "clsx";
import { useAtom } from "jotai";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { UnstakeStateReducerAtom } from "./atoms";
import useFetchUserStakeById from "@services/gldt_stake/hooks/useFetchUserStakeById";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";

const Confirm = () => {
  const { authenticatedAgent, isConnected } = useAuth();
  const [stateUnstake, dispatch] = useAtom(UnstakeStateReducerAtom);

  const stake = useFetchUserStakeById(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled:
        isConnected &&
        !!authenticatedAgent &&
        stateUnstake.stake_id !== undefined,
      id: stateUnstake.stake_id as bigint,
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
        <div className={clsx("")}>Withdraw</div>
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
