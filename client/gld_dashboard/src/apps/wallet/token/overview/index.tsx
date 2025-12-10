import clsx from "clsx";
import { useNavigate } from "react-router-dom";
import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import GradientCard from "@shared/ui/card/GradientCard";
import TokenHeaderPrice from "@shared/components/token-header-price";
import WalletListMobile from "@wallet/wallet-list-mobile";
import TotalCountToken from "@shared/components/total-count-token";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "@wallet/shared/atoms/TransferTokenAtom";
import { SwapStateReducerAtom } from "@wallet/token/swap/atoms";
import TransferDialogToken from "@wallet/token/transfer";
import AppFeatureBtn from "@shared/components/app-feature-btn";
import Swap from "@wallet/token/swap";
import BurnNFT from "@wallet/nft/mint-burn/burn";
import MintNFT from "@wallet/nft/mint-burn/mint";
import SwapNFTReducerAtom from "@wallet/nft/mint-burn/atoms/SwapNFTAtom";

const Overview = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const navigate = useNavigate();
  const token = useAtomValue(TokenSelectedAtom);
  const setTransferTokenState = useSetAtom(TransferTokenStateAtom);
  const setSendTokenState = useSetAtom(SendTokenStateAtom);
  const [, dispatchSwapState] = useAtom(SwapStateReducerAtom);
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);

  const balance = useFetchLedgerBalance(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const handleOpenTransferTokenDialog = () => {
    setSendTokenState(RESET);
    setTransferTokenState((state) => ({
      ...state,
      is_open_transfer_dialog: true,
    }));
  };

  const onOpenSwap = () => {
    dispatchSwapState({
      type: "OPEN_DIALOG_FORM",
      value: { send_token: token },
    });
  };

  const renderUserBalance = () => {
    if (!isConnected) {
      return <TotalCountToken token={token} amount={0} amountUSD={0} />;
    }
    if (balance.isLoading || balance.isError || balance.isFetching) {
      return (
        <TotalCountToken
          token={token}
          isFetching={true}
          amount={0}
          amountUSD={0}
        />
      );
    }
    if (balance.isSuccess && balance.data) {
      return (
        <TotalCountToken
          token={token}
          amount={balance.data.balance}
          amountUSD={balance.data.balance_usd}
        />
      );
    }
    return <TotalCountToken token={token} amount={0} amountUSD={0} />;
  };

  const renderTokenAction = () => {
    if (token.id === "gldt") {
      return (
        <>
          <AppFeatureBtn action="buy" handleOnClick={() => navigate("/buy")} />
          <AppFeatureBtn
            action="earn"
            handleOnClick={() => navigate("/earn")}
          />
          <AppFeatureBtn
            action="transfer"
            handleOnClick={handleOpenTransferTokenDialog}
          />
          <AppFeatureBtn action="swap" handleOnClick={onOpenSwap} />
          <AppFeatureBtn
            action="burn-nft"
            handleOnClick={() => {
              dispatchSwapNFT({ type: "INIT_BURN_MODE" });
            }}
          />
        </>
      );
    } else if (token.id === "goldao") {
      return (
        <>
          <AppFeatureBtn
            action="transfer"
            handleOnClick={handleOpenTransferTokenDialog}
          />
          <AppFeatureBtn action="swap" handleOnClick={onOpenSwap} />
          <AppFeatureBtn
            action="govern"
            handleOnClick={() => navigate("/govern")}
          />
        </>
      );
    } else {
      return (
        <>
          <AppFeatureBtn
            action="transfer"
            handleOnClick={handleOpenTransferTokenDialog}
          />
          <AppFeatureBtn action="swap" handleOnClick={onOpenSwap} />
        </>
      );
    }
  };

  return (
    <>
      <GradientCard className="p-4 xl:p-8 relative">
        <div className="flex flex-col items-center">
          <div className="pb-8 xl:pb-16">
            <div className="flex flex-col items-center">
              <TokenHeaderPrice
                className="hidden xl:block mb-8 xl:mb-12"
                token={token}
              />
              {renderUserBalance()}
            </div>

            <WalletListMobile className="flex justify-center xl:hidden mt-6 mb-8" />
          </div>
        </div>

        <div className={clsx("absolute -bottom-9 left-1/2 -translate-x-1/2")}>
          <div className="flex justify-center gap-2">{renderTokenAction()}</div>
        </div>
      </GradientCard>
      <TransferDialogToken />
      <Swap />
      <BurnNFT />
      <MintNFT />
    </>
  );
};

export default Overview;
