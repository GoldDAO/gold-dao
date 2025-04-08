import clsx from "clsx";
import { useAtom } from "jotai";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import InnerAppLayout from "@components/outlets/InnerAppLayout";
import NeuronOverview from "./neuron-overview";
import RewardAssets from "./reward-assets";
import NeuronList from "./neuron-list/List";
import { ClaimRewardStateReducerAtom } from "./claim-all-reward/atoms";
import ClaimRewardDisclaimer from "./claim-all-reward-disclaimer";
import ClaimRewardsConfirm from "./claim-all-reward/Confirm";
import ClaimRewardsDetails from "./claim-all-reward/Details";

const Govern = () => {
  const { connect, isConnected } = useAuth();
  const navigate = useNavigate();
  const [claimRewardState, dispatchClaimReward] = useAtom(
    ClaimRewardStateReducerAtom
  );

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col items-center text-center lg:text-left lg:items-start lg:flex-grow px-4 lg:px-8">
          <div className="text-5xl lg:text-6xl flex flex-row lg:flex-col justify-center gap-2 lg:gap-0 font-semibold mt-4">
            <div>Govern</div>
            <div className="text-primary font-light">with gold</div>
          </div>
          <div className="text-content/60 my-3">
            Stake your GOLDAO to earn weekly rewards in governance tokens,
            unlocking passive income from your gold holdings.
          </div>
          {!isConnected && (
            <Button
              className="mt-auto w-full px-4 py-3 bg-secondary text-white rounded-md"
              onClick={connect}
            >
              Connect Wallet
            </Button>
          )}
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="flex flex-col lg:flex-grow lg:overflow-y-auto">
          <NeuronOverview />
          <div className="relative w-full px-4 lg:pb-16 pb-32">
            <div
              className={clsx(
                "my-4",
                "absolute -top-26 lg:-top-16 left-1/2 lg:my-0 -translate-x-1/2 w-full lg:w-xl px-4"
              )}
            >
              <ClaimRewardDisclaimer />
            </div>
          </div>
          <div className="p-4 lg:p-8">
            <div className="flex justify-between items-center mb-8">
              <div>Assets</div>
              <div
                className="cursor-pointer"
                onClick={() => navigate("/wallet")}
              >
                My wallet
              </div>
            </div>
            <RewardAssets />
          </div>

          <div className="p-4 lg:p-8">
            <div className="mb-4 lg:mb-8">My GOLDAO neurons</div>
            <NeuronList />
          </div>
        </div>

        {/* CLAIM REWARDS DIALOGS */}
        <Dialog
          open={claimRewardState.is_open_claim_dialog_confirm}
          handleOnClose={() => dispatchClaimReward({ type: "CANCEL" })}
          title="Confirm claim rewards"
        >
          <ClaimRewardsConfirm />
        </Dialog>

        <Dialog
          open={claimRewardState.is_open_claim_dialog_details}
          handleOnClose={() => dispatchClaimReward({ type: "RESET" })}
          title="Claim details"
        >
          <ClaimRewardsDetails />
        </Dialog>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Govern;
