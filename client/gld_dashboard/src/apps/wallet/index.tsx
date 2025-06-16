import { useEffect } from "react";
import clsx from "clsx";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import WalletList from "@wallet/wallet-list";
import WalletItemHeader from "@wallet/wallet-item-header";
import WalletItemAction from "@wallet/wallet-item-action";
import TxHistoryToken from "@wallet/tx-history-token";
import TxHistoryNFT from "@wallet/tx-history-nft";
import { TokensList, TokensWhitelist, GLDT_INDEX } from "@wallet/shared/utils";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import GradientCard from "@shared/components/ui/card/GradientCard";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";

const Wallet = () => {
  const { isConnected } = useAuth();
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
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="flex flex-col flex-grow px-4 xl:px-8">
          <div className="text-center xl:text-left text-5xl xl:text-6xl text-gold/90 font-semibold my-4">
            Wallet
          </div>
          <div className="flex flex-col xl:flex-grow border border-border p-4 rounded-lg mt-4">
            <div className="text-center xl:text-left mb-4 text-copper text-sm font-semibold">
              Tokens
            </div>
            <WalletList />
          </div>

          {!isConnected && <BtnConnectWallet className="mt-auto w-full" />}
        </div>
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div className="flex flex-col overflow-hidden">
          <GradientCard className="rounded-tr-[inherit]">
            <WalletItemHeader className="p-4 xl:p-12" />
          </GradientCard>
          <div className="relative px-4">
            <WalletItemAction
              className={clsx(
                "my-4",
                "xl:absolute xl:-top-10 xl:left-1/2 xl:my-0 xl:-translate-x-1/2"
              )}
            />
          </div>
          <div className="p-4 xl:p-8 mt-4 xl:mt-12 flex flex-col overflow-hidden">
            <div className="mb-4">Transactions</div>
            {isConnected ? (
              searchParams.get("token") === "nft" ? (
                <TxHistoryNFT />
              ) : (
                <TxHistoryToken />
              )
            ) : (
              renderDisconnectedPlaceholder()
            )}
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Wallet;
