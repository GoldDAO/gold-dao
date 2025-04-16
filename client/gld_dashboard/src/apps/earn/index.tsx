import clsx from "clsx";
import { useAtom } from "jotai";
import { ExportSquare } from "iconsax-react";
import { useAuth } from "@auth/index";
import Dialog from "@components/dialogs/Dialog";
import { Button } from "@components/index";
import InnerAppLayout from "@components/outlets/InnerAppLayout";
import StakeForm from "./stake-form";
import StakeOverview from "./stake-overview";
import StakeList from "./stake-list";
import { ClaimRewardStateReducerAtom } from "./claim-all-reward/atoms";
import ClaimRewardDisclaimer from "./claim-all-reward-disclaimer";
import ClaimRewardsConfirm from "./claim-all-reward/Confirm";
import ClaimRewardsDetails from "./claim-all-reward/Details";

const Earn = () => {
  const { connect, isConnected } = useAuth();
  const [claimRewardState, dispatchClaimReward] = useAtom(
    ClaimRewardStateReducerAtom
  );

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col items-center text-center lg:text-left lg:items-start lg:flex-grow">
          <div className="text-5xl lg:text-6xl flex flex-col justify-center items-center lg:items-start font-semibold mt-4 px-4 lg:px-8">
            <div className="font-semibold text-primary/90">Earn</div>
            <div className="font-light">with gold</div>
          </div>
          <div className="text-content/60 my-3 px-4 lg:px-8">
            Stake your GLDT to{" "}
            <span className="font-semibold">earn weekly rewards</span> in
            governance tokens, unlocking passive income from your gold holdings.
            <a
              href="https://docs.gold-dao.org/resources/gldt-staking/"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 mt-4"
            >
              <div className="text-sm font-semibold tracking-widest">
                LEARN MORE
              </div>
              <div className="px-4 py-1 border border-border rounded-full">
                <ExportSquare size={16} />
              </div>
            </a>
          </div>

          <div className="lg:hidden mt-8 w-full">
            <StakeOverview />
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
          </div>
          <div className="mt-8 w-full px-4 lg:px-8">
            <StakeForm />
          </div>
          {!isConnected && (
            <div className="px-4 lg:px-8 mt-auto w-full">
              <Button
                className="w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
                onClick={connect}
              >
                Connect Wallet
              </Button>
            </div>
          )}
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="flex flex-col lg:flex-grow lg:overflow-y-auto">
          <div className="hidden lg:block">
            <StakeOverview />
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
          </div>

          <div className="p-4 lg:p-8">
            <div className="mb-4 lg:mb-8">My Stakes</div>
            <StakeList />
          </div>
        </div>

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
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Earn;
