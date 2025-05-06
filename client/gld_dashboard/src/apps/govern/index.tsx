import clsx from "clsx";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import InnerAppLayout from "@components/outlets/InnerAppLayout";
import NeuronsOverview from "./neuron-overview";
import NeuronsList from "./neuron-list";
import { ClaimRewardStateReducerAtom } from "./claim-reward/claim-all/atoms";
import { AddNeuronStateReducerAtom } from "./add-neuron/atoms";
import ClaimRewardDisclaimer from "./claim-reward/claim-disclaimer";
import ClaimRewardsConfirm from "./claim-reward/claim-all/Confirm";
import ClaimRewardsDetails from "./claim-reward/claim-all/Details";
import AddNeuron from "./add-neuron";

const Govern = () => {
  const { connect, isConnected } = useAuth();
  const [claimRewardState, dispatchClaimReward] = useAtom(
    ClaimRewardStateReducerAtom
  );
  const [addNeuronState, dispatchAddNeuron] = useAtom(
    AddNeuronStateReducerAtom
  );

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col items-center text-center xl:text-left xl:items-start xl:flex-grow px-4 xl:px-8">
          <div className="text-5xl xl:text-6xl flex flex-col justify-center items-center xl:items-start font-semibold mt-4">
            <div className="font-semibold text-primary/90">Govern</div>
            <div className="font-light">the DAO</div>
          </div>
          <div className="text-content/60 my-3">
            Stake GOLDAO tokens to govern the Gold DAO and earn rewards yielding
            up to 30% APY.
          </div>
          {!isConnected && (
            <Button
              className="mt-auto w-full px-4 py-3 bg-secondary xl:text-lg text-white rounded-md"
              onClick={connect}
            >
              Connect Wallet
            </Button>
          )}
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="flex flex-col xl:flex-grow xl:overflow-y-auto">
          <NeuronsOverview />
          <div className="relative w-full px-4 xl:pb-16 pb-32">
            <div
              className={clsx(
                "my-4",
                "absolute -top-26 xl:-top-16 left-1/2 xl:my-0 -translate-x-1/2 w-full xl:w-xl px-4"
              )}
            >
              <ClaimRewardDisclaimer />
            </div>
          </div>

          <div className="p-4 xl:p-8">
            <div className="flex items-center justify-between mb-4 xl:mb-8">
              <div>My GOLDAO neurons</div>
              <Button
                className="px-4 py-2 bg-secondary text-white rounded-full"
                onClick={() => dispatchAddNeuron({ type: "OPEN_DIALOG" })}
              >
                Add neuron
              </Button>
            </div>
            <NeuronsList />
          </div>
        </div>

        {/* CLAIM REWARDS DIALOGS */}
        <Dialog
          open={claimRewardState.is_open_claim_dialog_confirm}
          handleOnClose={() => dispatchClaimReward({ type: "CANCEL" })}
          title="Claim rewards"
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

        {/* ADD NEURON DIALOG */}
        <Dialog
          open={addNeuronState.is_open}
          handleOnClose={() => dispatchAddNeuron({ type: "RESET" })}
        >
          <AddNeuron />
        </Dialog>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Govern;
