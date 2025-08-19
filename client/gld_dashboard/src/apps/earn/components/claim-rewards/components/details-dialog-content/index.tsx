import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { ClaimRewardsStateReducerAtom } from "../../atoms";
import Button from "@shared/ui/button/HorizontalButton";
import useClaimRewards from "./hooks/useClaimRewards";
import { LoaderSpin } from "@components/loaders";
// import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const DetailsDialogContent = () => {
  const { authenticatedAgent } = useAuth();
  const [claimRewardsState, dispatchClaimRewardsState] = useAtom(
    ClaimRewardsStateReducerAtom
  );

  const claimRewards = useClaimRewards(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent
  );

  const handleClaimRewards = () => {
    claimRewards.mutate({
      tokens: claimRewardsState.rewards
        .filter((reward) => reward.is_selected)
        .map((reward) => reward.name),
    });
  };

  useEffect(() => {
    if (claimRewards.isIdle && claimRewardsState.is_step_details) {
      handleClaimRewards();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [claimRewards.isIdle, claimRewardsState.is_step_details]);

  useEffect(() => {
    return () => {
      claimRewards.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onRetry = () => {
    claimRewards.reset();
    handleClaimRewards();
  };

  const onClose = () => {
    dispatchClaimRewardsState({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <div>
      {(claimRewards.isIdle || claimRewards.isPending) && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-6">
            <LoaderSpin />
            <div>Claiming rewards....</div>
          </div>
        </div>
      )}
      {claimRewards.isError && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div className="mb-2">Claim rewards error</div>
            <div className="text-content/60">{claimRewards.error.message}</div>
          </div>
          <div className="mt-4 flex items-center gap-2 w-full">
            <Button onClick={onRetry} className="w-full">
              Retry
            </Button>
            <Button onClick={onClose} className="w-full">
              Close
            </Button>
          </div>
        </div>
      )}
      {claimRewards.isSuccess && (
        <div className="flex flex-col items-center gap-4">
          <div className="flex flex-col items-center justify-center gap-2">
            <div>Claim rewards success!</div>
          </div>
          <Button size="lg" onClick={onClose} className="mt-4 w-full">
            Close
          </Button>
        </div>
      )}
    </div>
  );
};

export default DetailsDialogContent;

// const TokenItem = ({
//   reward,
//   stake_id,
// }: {
//   reward: Reward;
//   stake_id: bigint;
// }) => {
//   const { authenticatedAgent } = useAuth();

//   const claim = useClaimReward(GLDT_STAKE_CANISTER_ID, authenticatedAgent);

//   // const decimals = useFetchLedgerDecimals(reward.canisterId, unauthenticatedAgent, {
//   //   ledger: reward.id,
//   //   enabled: !!unauthenticatedAgent && isConnected,
//   // });

//   const handleClaimReward = () => {
//     claim.mutate({
//       position_ids: [stake_id],
//       token: reward.name,
//     });
//   };

//   useEffect(() => {
//     if (claim.isIdle) handleClaimReward();
//     // eslint-disable-next-line react-hooks/exhaustive-deps
//   }, [claim.isIdle]);

//   // <div className="flex justify-center items-center">Loading...</div>

//   const handleOnRetry = () => {
//     claim.reset();
//     handleClaimReward();
//   };

//   return (
//     <div className="p-4 border border-border rounded-md">
//       <div className="flex justify-between items-center">
//         <div className="flex items-center gap-4">
//           <MutationStatusIcon status={claim.status} />
//           <div>Claiming {reward.name} reward</div>
//         </div>
//         {claim.isError && (
//           <div>
//             <BtnPrimary size="sm" onClick={handleOnRetry}>
//               Retry
//             </BtnPrimary>
//           </div>
//         )}
//       </div>
//     </div>
//   );
// };

// const Details = () => {
//   const [claimRewardState, dispatch] = useAtom(ClaimRewardStateReducerAtom);
//   const [selectedRewards] = useAtom(SelectedRewardsAtom);
//   const navigate = useNavigate();

//   const handleNavigateToWallet = () => {
//     dispatch({ type: "RESET" });
//     navigate("/wallet");
//   };

//   return (
//     <>
//       <div className="grid grid-cols-1 gap-4 my-8">
//         {selectedRewards.map((reward) => (
//           <TokenItem
//             key={reward.id}
//             reward={reward}
//             stake_id={claimRewardState.stake_id as bigint}
//           />
//         ))}
//       </div>
//       <BtnPrimary className="w-full" onClick={handleNavigateToWallet}>
//         Go to wallet view
//       </BtnPrimary>
//     </>
//   );
// };

// export default Details;
