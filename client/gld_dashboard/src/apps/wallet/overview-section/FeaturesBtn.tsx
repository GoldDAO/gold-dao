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
import TransferDialogToken from "@wallet/transfer-token";
import MintNFT from "@advanced/gldt/overview-section/mint-nft";
import BurnNFT from "@advanced/gldt/overview-section/burn-nft";
import AppFeatureBtn from "@shared/components/app-feature-btn";

const FeaturesBtn = () => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const token = useAtomValue(TokenSelectedAtom);
  const setTransferTokenState = useSetAtom(TransferTokenStateAtom);
  const setSendTokenState = useSetAtom(SendTokenStateAtom);
  const [, dispatchTransferNFT] = useAtom(TransferNFTStateReducerAtom);

  const { id } = token;

  const handleOpenTransferTokenDialog = () => {
    setSendTokenState(RESET);
    setTransferTokenState((state) => ({
      ...state,
      is_open_transfer_dialog: true,
    }));
  };

  const renderTokenAction = () => {
    if (searchParams.get("token") === "nft") {
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
      if (id === "gldt") {
        return (
          <>
            <AppFeatureBtn
              action="buy-gldt"
              handleOnClick={() => navigate("/buy")}
            />
            <AppFeatureBtn
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            {/* <AppFeatureBtn action="swap" /> */}
            <AppFeatureBtn
              action="burn-nft"
              handleOnClick={() => navigate("/advanced/gldt")}
            />
            {/* <AppFeatureBtn
              action="earn"
              handleOnClick={() => navigate("/earn")}
            /> */}
          </>
        );
      } else if (id === "goldao") {
        return (
          <>
            <AppFeatureBtn
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            {/* <AppFeatureBtn action="swap" /> */}
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
            {/* <AppFeatureBtn action="swap" /> */}
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
    </>
  );
};

export default FeaturesBtn;
