import { useEffect } from "react";
import clsx from "clsx";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import WalletList from "@wallet/wallet-list";
import WalletListDisconnected from "@wallet/wallet-list-disconnected";
import WalletItemHeader from "@wallet/wallet-item-header";
import WalletItemAction from "@wallet/wallet-item-action";
import TxHistoryToken from "@wallet/tx-history-token";
import TxHistoryNFT from "@wallet/tx-history-nft";
import TxHistoryDisconnected from "@wallet/tx-history-disconnected";
import { TokensList, TokensWhitelist, GLDT_INDEX } from "@wallet/shared/utils";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import GradientCard from "@shared/components/ui/card/GradientCard";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import WalletListMobile from "@wallet/wallet-list-mobile";

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

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="text-center xl:text-left text-4xl xl:text-6xl text-gold font-semibold">
          Wallet
        </div>
        <div className="hidden xl:block">
          <div className="border border-border p-4 rounded-xl my-4">
            <div className="text-center xl:text-left mb-4 text-copper text-sm font-semibold">
              Tokens
            </div>
            {isConnected ? <WalletList /> : <WalletListDisconnected />}
          </div>
        </div>

        {!isConnected && <BtnConnectWallet className="mt-auto w-full" />}
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <GradientCard className="rounded-tr-[inherit]">
          <div className="pt-2 xl:pt-12 px-4 xl:px-12 pb-20">
            <WalletItemHeader />
            <WalletListMobile className="flex justify-center xl:hidden mt-4" />
          </div>
        </GradientCard>
        <div className="relative px-4">
          <WalletItemAction
            className={clsx(
              "my-4",
              "absolute -top-13 left-1/2 my-0 -translate-x-1/2"
            )}
          />
        </div>
        <div className="p-4 xl:p-8 mt-12">
          <div className="mb-4">Transactions</div>
          {isConnected ? (
            searchParams.get("token") === "nft" ? (
              <TxHistoryNFT />
            ) : (
              <TxHistoryToken />
            )
          ) : (
            <TxHistoryDisconnected />
          )}
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Wallet;
