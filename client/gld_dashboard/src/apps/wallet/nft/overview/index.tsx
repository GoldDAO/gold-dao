import clsx from "clsx";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";
import GradientCard from "@shared/ui/card/GradientCard";
import WalletListMobile from "@wallet/wallet-list-mobile";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";
import TransferDialogNFT from "@wallet/nft/transfer";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import AppFeatureBtn from "@shared/components/app-feature-btn";
import SwapNFTReducerAtom from "../mint-burn/atoms/SwapNFTAtom";
import MintNFT from "@wallet/nft/mint-burn/mint";
import BurnNFT from "@wallet/nft/mint-burn/burn";

const Overview = () => {
  const { unauthenticatedAgent } = useAuth();
  const [, dispatchTransferNFT] = useAtom(TransferNFTStateReducerAtom);
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);

  const priceGold = useFetchPriceGold({
    enabled: !!unauthenticatedAgent,
  });

  return (
    <>
      <GradientCard className="p-4 xl:p-8 relative">
        <div className="flex flex-col items-center">
          <div className="pb-8 xl:pb-16">
            <div className="flex flex-col items-center">
              <div className="hidden xl:block mb-8 xl:mb-12">
                <div className="flex flex-col items-center">
                  <div className="flex items-center gap-1">
                    <Logo name="gld_nft" className="h-6 w-6" />
                    <div className="font-semibold text-xl">GLD NFT</div>
                  </div>
                  <div className="text-xs xl:text-sm text-content/60">
                    1 gram Gold ≈{" "}
                    <span>
                      {priceGold.isSuccess ? (
                        <>
                          $
                          <NumberToLocaleString value={priceGold.data} />
                        </>
                      ) : (
                        <span className="animate-pulse">($0)</span>
                      )}
                    </span>
                  </div>
                </div>
              </div>
              <TotalCountUserNFTs />
            </div>

            <WalletListMobile className="flex justify-center xl:hidden mt-6 mb-8" />
          </div>
        </div>

        <div className={clsx("absolute -bottom-9 left-1/2 -translate-x-1/2")}>
          <div className="flex justify-center gap-2">
            <AppFeatureBtn
              action="transfer"
              handleOnClick={() =>
                dispatchTransferNFT({ type: "OPEN_TRANSFER_DIALOG" })
              }
            />
            <AppFeatureBtn
              action="mint-nft"
              handleOnClick={() => {
                dispatchSwapNFT({ type: "INIT_MINT_MODE" });
              }}
            />
            <AppFeatureBtn
              action="burn-nft"
              handleOnClick={() => {
                dispatchSwapNFT({ type: "INIT_BURN_MODE" });
              }}
            />
            <AppFeatureBtn action="buy-on-bity" />
            {/* <AppFeatureBtn action="redeem" /> */}
          </div>
        </div>
      </GradientCard>
      <TransferDialogNFT />
      <MintNFT />
      <BurnNFT />
    </>
  );
};

export default Overview;
