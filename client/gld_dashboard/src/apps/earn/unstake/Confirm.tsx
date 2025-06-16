import clsx from "clsx";
import { useAtom } from "jotai";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { UnstakeStateReducerAtom } from "./atoms";
import useFetchUserStakeById from "@services/gldt_stake/hooks/useFetchUserStakeById";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";
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
      <div className="flex justify-center items-center px-4 py-16 xl:py-32">
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
        <BtnPrimary onClick={() => dispatch({ type: "CONFIRM" })}>
          Confirm
        </BtnPrimary>
      </div>
    </>
  );
};

export default Confirm;
