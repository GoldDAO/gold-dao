import { useEffect } from "react";
import { useAtomValue, useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import WalletList from "@wallet/wallet-list";
import WalletListDisconnected from "@wallet/wallet-list-disconnected";
import WalletTotal from "@wallet/wallet-total";
import TxHistoryToken from "@wallet/token/tx-history";
import TxHistoryDisconnected from "@wallet/shared/components/tx-history-disconnected";
import { TOKENS, TOKEN_WHITELIST, TOKEN_GOLDAO } from "@shared/utils/tokens";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { CustomTokensAtom } from "@wallet/shared/atoms/CustomTokensAtom";
import AddCustomTokenDialog from "@wallet/token/custom/add-dialog";
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
  const customTokens = useAtomValue(CustomTokensAtom);

  // Runs again when the custom tokens change, so that removing the selected
  // token falls back to GOLDAO.
  useEffect(() => {
    const param = searchParams.get("token");
    if (param === "GLDNFT") return;

    const token =
      param && TOKEN_WHITELIST.includes(param)
        ? TOKENS.find((t) => t.display_name === param)
        : customTokens.find((t) => t.display_name === param);

    if (!token) {
      setSelectedToken(TOKEN_GOLDAO);
      setSearchParams({ token: TOKEN_GOLDAO.display_name });
    } else {
      setSelectedToken(token);
      // Supprimer tous les autres paramètres sauf "token"
      setSearchParams({ token: token.display_name });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams, customTokens]);

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="text-center xl:text-left text-4xl xl:text-6xl text-gold font-semibold">
          Wallet
        </div>
        <WalletTotal className="mt-3" />
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
      <AddCustomTokenDialog />
    </InnerAppLayout>
  );
};

export default Wallet;
