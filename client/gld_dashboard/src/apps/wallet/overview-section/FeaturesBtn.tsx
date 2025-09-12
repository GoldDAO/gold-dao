import { useAtom } from "jotai";
import { useNavigate, useSearchParams } from "react-router-dom";
import { useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "@wallet/shared/atoms/TransferTokenAtom";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import TransferDialogToken from "@wallet/transfer-token";
import MintNFT from "@advanced/gldt/overview-section/mint-nft";
import BurnNFT from "@advanced/gldt/overview-section/burn-nft";
import AppFeatureBtn from "@shared/components/app-feature-btn";
import Swap from "@wallet/swap";

const FeaturesBtn = () => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const token = useAtomValue(TokenSelectedAtom);
  const setTransferTokenState = useSetAtom(TransferTokenStateAtom);
  const setSendTokenState = useSetAtom(SendTokenStateAtom);
  const [, dispatchTransferNFT] = useAtom(TransferNFTStateReducerAtom);
  const [, dispatchSwapState] = useAtom(SwapStateReducerAtom);

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

  const renderTokenAction = () => {
    if (searchParams.get("token") === "GLDNFT") {
      return (
        <>
          <AppFeatureBtn
            action="transfer"
            handleOnClick={() =>
              dispatchTransferNFT({ type: "OPEN_TRANSFER_DIALOG" })
            }
          />
          <AppFeatureBtn
            action="mint-nft"
            handleOnClick={() => navigate("/advanced/gldt")}
          />
          <AppFeatureBtn action="buy-on-bity" />
          {/* <AppFeatureBtn action="redeem" /> */}
        </>
      );
    } else {
      if (token.id === "gldt") {
        return (
          <>
            <AppFeatureBtn
              action="buy"
              handleOnClick={() => navigate("/buy")}
            />
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
              handleOnClick={() => navigate("/advanced/gldt")}
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
    }
  };

  return (
    <>
      <div className={clsx("flex justify-center gap-2")}>
        {renderTokenAction()}
      </div>

      <TransferDialogToken />
      <MintNFT />
      <BurnNFT />

      <Swap />
    </>
  );
};

export default FeaturesBtn;
