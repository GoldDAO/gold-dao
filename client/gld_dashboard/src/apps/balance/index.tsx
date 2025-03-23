import clsx from "clsx";

import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import InnerAppLayout from "@components/outlets/InnerAppLayout";

import BalanceList from "./Balance.list.component";
import BalanceHeader from "./Balance.header.component";
import BalanceBtnAction from "./Balance.btn-action.component";

const Balance = () => {
  const { isConnected, connect } = useAuth();

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col flex-grow">
          <div className="text-center lg:text-left text-2xl lg:text-4xl xl:text-5xl 2xl:text-6xl font-semibold mb-4 lg:mb-12">
            Balance
          </div>
          <div className="text-center lg:text-left mb-4 lg:mb-6">My Wallet</div>

          <BalanceList />

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
        <div
          className={clsx(
            "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800 rounded-tr-[inherit]"
          )}
        >
          <BalanceHeader className="p-4 lg:p-12" />
        </div>
        <div className="relative px-4">
          <BalanceBtnAction
            className={clsx(
              "my-4",
              "lg:absolute lg:-top-11 lg:left-1/2 lg:my-0 lg:-translate-x-1/2"
            )}
          />
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Balance;
