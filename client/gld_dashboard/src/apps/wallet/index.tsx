import { useEffect } from "react";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import WalletList from "@wallet/wallet-list";
import WalletListDisconnected from "@wallet/wallet-list-disconnected";
import TxHistoryToken from "@wallet/tx-history-token";
import TxHistoryNFT from "@wallet/tx-history-nft";
import TxHistoryDisconnected from "@wallet/tx-history-disconnected";
import { TOKENS, TOKEN_WHITELIST, TOKEN_GLDT } from "@shared/utils/tokens";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import OverviewSection from "@wallet/overview-section";

const Wallet = () => {
  const { isConnected } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const setSelectedToken = useSetAtom(TokenSelectedAtom);

  useEffect(() => {
    if (
      !searchParams.get("token") ||
      !TOKEN_WHITELIST.includes(searchParams.get("token")!)
    ) {
      searchParams.set("token", TOKEN_GLDT.id);
      setSelectedToken(TOKEN_GLDT);
      setSearchParams(searchParams);
    } else {
      if (searchParams.get("token") !== "gldnft") {
        setSelectedToken(
          TOKENS.find((t) => t.id === searchParams.get("token"))!
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

        {!isConnected && (
          <BtnConnectWallet className="hidden xl:block mt-auto w-full" />
        )}
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <OverviewSection />
        <div className="p-4 xl:p-8 mt-12">
          <div className="mb-4">Transactions</div>
          {isConnected ? (
            searchParams.get("token") === "gldnft" ? (
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
