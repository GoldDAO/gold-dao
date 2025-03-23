import { useEffect } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";

import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
// import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { ClaimRewardStateReducerAtom } from "./atoms";
import { TokenData } from "./utils";
// import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useClaimReward from "@services/gldt_stake/hooks/useClaimReward";

const TokenItem = ({
  token,
  stake_id,
}: {
  token: TokenData;
  stake_id: bigint;
}) => {
  const { authenticatedAgent } = useAuth();

  const claim = useClaimReward(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

  // const decimals = useFetchDecimals(token.canisterId, authenticatedAgent, {
  //   ledger: token.id,
  //   enabled: !!authenticatedAgent && !!isConnected,
  // });

  useEffect(() => {
    if (claim.isIdle) {
      claim.mutate(
        {
          id: stake_id,
          token: token.name,
        },
        {
          onSuccess: (res) => {
            console.log("claimed");
            console.log(res);
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [claim.isIdle]);

  // <div className="flex justify-center items-center">Loading...</div>

  const handleOnRetry = () => {
    claim.reset();
    claim.mutate(
      {
        id: stake_id,
        token: token.name,
      },
      {
        onSuccess: (res) => {
          console.log("claimed");
          console.log(res);
        },
      }
    );
  };

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-4">
          <MutationStatusIcons status={claim.status} />
          <div>Claim {token.name} reward</div>
        </div>
        {claim.isError && (
          <div>
            <Button
              className={clsx(
                "px-2 py-1 rounded-md",
                "bg-secondary text-white text-sm"
              )}
              onClick={handleOnRetry}
            >
              Retry
            </Button>
          </div>
        )}
      </div>
    </div>
  );
};

const Details = () => {
  const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);

  return (
    <>
      <div className="grid grid-cols-1 gap-4 my-8">
        {claimRewardState.token_selected.map((token) => (
          <TokenItem
            key={token.id}
            token={token}
            stake_id={claimRewardState.stake_id as bigint}
          />
        ))}
      </div>
      <Button
        className={clsx(
          "px-4 py-3 rounded-md w-full",
          "bg-secondary text-white"
        )}
        onClick={() => dispatch({ type: "RESET" })}
      >
        Go to balance view
      </Button>
    </>
  );
};

export default Details;
