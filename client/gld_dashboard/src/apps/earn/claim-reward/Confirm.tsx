import clsx from "clsx";
import { useAtom } from "jotai";

import { GLDT_STAKE_CANISTER_ID, GLDT_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { ClaimRewardStateReducerAtom } from "./atoms";
import { Token } from "./utils";
import useFetchUserStakeById from "@services/gldt_stake/hooks/useFetchUserStakeById";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";

const TokenItem = ({ name, amount }: { name: string; amount: bigint }) => {
  const { authenticatedAgent, isConnected } = useAuth();
  const token = Token[name as keyof typeof Token];

  const decimals = useFetchDecimals(token.canisterId, authenticatedAgent, {
    ledger: token.id,
    enabled: !!authenticatedAgent && !!isConnected,
  });

  return (
    <div className="p-4 border border-border rounded-md">
      {decimals.isSuccess ? (
        <div className="flex gap-2">
          <TokenValueToLocaleString
            value={amount}
            tokenDecimals={decimals.data}
          />
          <div>{name}</div>
        </div>
      ) : (
        <div className="flex justify-center items-center">Loading...</div>
      )}
    </div>
  );
};

const Confirm = () => {
  const { authenticatedAgent, isConnected } = useAuth();
  const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);

  const fee = useFetchTransferFee(GLDT_LEDGER_CANISTER_ID, authenticatedAgent, {
    ledger: "gldt",
    enabled: !!authenticatedAgent && !!isConnected,
  });

  const stake = useFetchUserStakeById(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled:
        isConnected &&
        !!authenticatedAgent &&
        fee.isSuccess &&
        !!claimRewardState.stake_id,
      fee: fee.data as bigint,
      id: claimRewardState.stake_id as bigint,
    }
  );

  if (!claimRewardState.stake_id || !stake.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 lg:py-32">
        Loading...
      </div>
    );
  }

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        {stake.data.claimable_rewards.list.map(({ name, amount }) => (
          <TokenItem key={name} name={name} amount={amount} />
        ))}
      </div>
      <Button
        className={clsx(
          "px-4 py-3 rounded-md w-full",
          "bg-secondary text-white"
        )}
        onClick={() => dispatch({ type: "CONFIRM" })}
      >
        Confirm
      </Button>
    </>
  );
};

export default Confirm;
