import { useEffect } from "react";
import clsx from "clsx";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";

import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import InnerAppLayout from "@components/outlets/InnerAppLayout";

import BalanceList from "./Balance.list.component";
import BalanceHeader from "./Balance.header.component";
import BalanceBtnAction from "./Balance.btn-action.component";

import TxHistory from "./transactions-history";

import { TokensList, TokensWhitelist, GLDT_INDEX } from "./balance.utils";
import { TokenSelectedAtom } from "./balance.atoms";

const Balance = () => {
  const { isConnected, connect } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const setSelectedToken = useSetAtom(TokenSelectedAtom);

  useEffect(() => {
    if (
      !searchParams.get("token") ||
      !TokensWhitelist.includes(searchParams.get("token")!)
    ) {
      searchParams.set("token", TokensList[GLDT_INDEX].id);
      setSelectedToken(TokensList[GLDT_INDEX]);
      setSearchParams(searchParams);
    } else {
      if (searchParams.get("token") !== "nft") {
        setSelectedToken(
          TokensList.find((t) => t.id === searchParams.get("token"))!
        );
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams]);

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
        <div className="flex flex-col overflow-hidden">
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
          <div className="p-4 lg:p-8 mt-4 lg:mt-12 flex flex-col overflow-hidden">
            <div className="mb-4">Last transactions</div>
            <TxHistory />
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Balance;
