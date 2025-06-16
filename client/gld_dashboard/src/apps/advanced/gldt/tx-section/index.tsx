import clsx from "clsx";
import {
  Disclosure,
  DisclosureButton,
  DisclosurePanel,
} from "@headlessui/react";
import { ChevronUpIcon } from "@heroicons/react/24/solid";
import { useAuth } from "@auth/index";
import PastTxHistory from "@advanced/gldt/tx-section/past-tx-history";
import OngoingTxHistory from "@advanced/gldt/tx-section/ongoing-tx-history";

const TxSection = ({ className }: { className?: string }) => {
  const { isConnected } = useAuth();

  const renderDisconnectedPlaceholder = () => {
    return (
      <div className="flex flex-col gap-4 relative">
        {[...Array(3)].map((_, index) => (
          <div key={index}>
            <div
              className={clsx(
                "@container",
                "shrink-0",
                "rounded-md xl:rounded-xl border border-border/40 p-4"
              )}
            >
              <div className="flex justify-between items-center p-2">
                <div className="flex items-center gap-2">
                  <div className="h-5 w-5 bg-surface-secondary rounded-full" />
                  <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
                </div>
                <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
              </div>
            </div>
          </div>
        ))}
        <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-background to-transparent" />
      </div>
    );
  };

  return (
    <div className={className}>
      <div className="flex items-center justify-between mb-4 xl:mb-8">
        <div>My transactions</div>
      </div>
      {isConnected ? (
        <div className="flex flex-col gap-4">
          <Disclosure defaultOpen={true}>
            {({ open }) => (
              <div>
                <DisclosureButton
                  className={clsx(
                    "flex items-center justify-between w-full p-4 bg-surface-secondary border border-border focus:outline-none",
                    open ? "rounded-t-xl rounded-b-none" : "rounded-xl"
                  )}
                >
                  <div>Ongoing</div>
                  <ChevronUpIcon
                    className={clsx(
                      "w-5 h-5 transition-transform",
                      open ? "rotate-180" : "rotate-0"
                    )}
                  />
                </DisclosureButton>
                <DisclosurePanel className="bg-surface-primary p-4 border-x border-b border-border rounded-b-xl -mt-px">
                  <div className="border border-border rounded-lg">
                    <OngoingTxHistory />
                  </div>
                </DisclosurePanel>
              </div>
            )}
          </Disclosure>
          <Disclosure defaultOpen={true}>
            {({ open }) => (
              <div>
                <DisclosureButton
                  className={clsx(
                    "flex items-center justify-between w-full p-4 bg-surface-secondary border border-border focus:outline-none",
                    open ? "rounded-t-xl rounded-b-none" : "rounded-xl"
                  )}
                >
                  <div>Past</div>
                  <ChevronUpIcon
                    className={clsx(
                      "w-5 h-5 transition-transform",
                      open ? "rotate-180" : "rotate-0"
                    )}
                  />
                </DisclosureButton>
                <DisclosurePanel className="bg-surface-primary p-4 border-x border-b border-border rounded-b-xl -mt-px">
                  <div className="border border-border rounded-lg">
                    <PastTxHistory />
                  </div>
                </DisclosurePanel>
              </div>
            )}
          </Disclosure>
        </div>
      ) : (
        renderDisconnectedPlaceholder()
      )}
    </div>
  );
};

export default TxSection;
