import { useEffect } from "react";
import clsx from "clsx";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import InnerAppLayout from "@components/outlets/InnerAppLayout";
import WalletList from "./components/wallet-list";
import WalletItemOverviewHeader from "./components/wallet-item-overview/Header";
import WalletItemOverviewBtnAction from "./components/wallet-item-overview/BtnAction";
import Transactions from "./components/transactions";
import { TokensList, TokensWhitelist, GLDT_INDEX } from "./utils";
import { TokenSelectedAtom } from "./atoms/WalletAtom";

const Wallet = () => {
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

  const renderDisconnectedPlaceholder = () => {
    return (
      <div className="flex flex-col gap-4 relative">
        {[...Array(2)].map((_, index) => (
          <div key={index}>
            <div
              className={clsx(
                "@container",
                "shrink-0",
                "rounded-md xl:rounded-xl border border-border/40 p-4 cursor-pointer"
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
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col flex-grow px-4 xl:px-8">
          <div className="text-center xl:text-left text-5xl xl:text-6xl text-primary/90 font-semibold my-4">
            Wallet
          </div>
          <div className="flex flex-col xl:flex-grow border border-border p-4 rounded-lg mt-4">
            <div className="text-center xl:text-left mb-4 text-primary/90">
              Tokens
            </div>
            <WalletList />
          </div>

          {!isConnected && (
            <div className="w-full mt-auto">
              <Button
                className="w-full mt-4 xl:mt-16 px-4 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
                onClick={connect}
              >
                Connect Wallet
              </Button>
            </div>
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
            <WalletItemOverviewHeader className="p-4 xl:p-12" />
          </div>
          <div className="relative px-4">
            <WalletItemOverviewBtnAction
              className={clsx(
                "my-4",
                "xl:absolute xl:-top-11 xl:left-1/2 xl:my-0 xl:-translate-x-1/2"
              )}
            />
          </div>
          <div className="p-4 xl:p-8 mt-4 xl:mt-12 flex flex-col overflow-hidden">
            <div className="mb-4">Transactions</div>
            {isConnected ? <Transactions /> : renderDisconnectedPlaceholder()}
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Wallet;
