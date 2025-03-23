import clsx from "clsx";

import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import InnerAppLayout from "@components/outlets/InnerAppLayout";
import StakeForm from "./stake-form";
import StakeOverview from "./stake-overview";
import StakeList from "./stake-list";

const Earn = () => {
  const { connect, isConnected } = useAuth();

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col lg:flex-grow">
          <div className="text-left text-2xl lg:text-4xl flex flex-row lg:flex-col gap-2 lg:gap-0 font-semibold mb-2">
            <div>Earn</div>
            <div className="text-primary">with gold</div>
          </div>
          <div className="text-left text-sm lg:text-base text-content/60">
            Stake your GLDT to{" "}
            <span className="font-semibold">earn weekly rewards</span> in
            governance tokens, unlocking passive income from your gold holdings.
          </div>
          <div className="mt-8">
            <StakeForm />
          </div>
          {!isConnected && (
            <Button
              className="mt-auto w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
              onClick={connect}
            >
              Connect Wallet
            </Button>
          )}
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="flex flex-col lg:flex-grow">
          <StakeOverview
            className={clsx(
              "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800",
              "rounded-tr-[inherit] p-4 lg:p-8"
            )}
          />
          {/* <div className="relative px-4">
          <div
            className={clsx(
              "my-4",
              "lg:absolute lg:-top-11 lg:left-1/2 lg:my-0 lg:-translate-x-1/2"
            )}
          >
            Rewards
          </div>
        </div> */}
          <div className="flex flex-col lg:flex-grow lg:h-100 p-4 lg:p-8">
            <div className="mb-4 lg:mb-8">My Stakes</div>
            <StakeList />
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Earn;
