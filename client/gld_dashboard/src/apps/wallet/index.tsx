import { useEffect } from "react";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import WalletList from "@wallet/wallet-list";
import WalletListDisconnected from "@wallet/wallet-list-disconnected";
import TxHistoryToken from "@wallet/token/tx-history";
import TxHistoryDisconnected from "@wallet/shared/components/tx-history-disconnected";
import { TOKENS, TOKEN_WHITELIST, TOKEN_GLDT } from "@shared/utils/tokens";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import OverviewToken from "@wallet/token/overview";
import OverviewNFT from "@wallet/nft/overview";
import NFTMaintenanceMode from "@shared/components/nft-maintenance-mode";
import useGetGLDDashboardMaintenanceMode from "@shared/hooks/useGetGLDDashboardMaintenanceMode";
import { GLD_DASHBOARD_MAINTENANCE_MODE_CANISTER_ID } from "@constants";
import Tabs from "@wallet/nft/tabs";

const Wallet = () => {
  const { isConnected, unauthenticatedAgent } = useAuth();
  const { data: maintenanceMode } = useGetGLDDashboardMaintenanceMode(
    GLD_DASHBOARD_MAINTENANCE_MODE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );
  const [searchParams, setSearchParams] = useSearchParams();
  const setSelectedToken = useSetAtom(TokenSelectedAtom);

  useEffect(() => {
    if (
      !searchParams.get("token") ||
      !TOKEN_WHITELIST.includes(searchParams.get("token")!)
    ) {
      setSelectedToken(TOKEN_GLDT);
      setSearchParams({ token: TOKEN_GLDT.display_name });
    } else {
      if (searchParams.get("token") !== "GLDNFT") {
        setSelectedToken(
          TOKENS.find((t) => t.display_name === searchParams.get("token"))!
        );
        // Supprimer tous les autres paramètres sauf "token"
        setSearchParams({ token: searchParams.get("token")! });
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
        {searchParams.get("token") === "GLDNFT" &&
          (maintenanceMode ? (
            <NFTMaintenanceMode />
          ) : (
            <>
              <OverviewNFT />
              <div className="p-4 xl:p-8 mt-12">
                <Tabs />
              </div>
            </>
          ))}

        {searchParams.get("token") !== "GLDNFT" && (
          <>
            <OverviewToken />
            <div className="p-4 xl:p-8 mt-12">
              <div className="mb-4">Transactions</div>
              {isConnected ? <TxHistoryToken /> : <TxHistoryDisconnected />}
            </div>
          </>
        )}
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Wallet;
