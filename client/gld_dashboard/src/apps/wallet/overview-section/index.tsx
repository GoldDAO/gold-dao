import clsx from "clsx";
import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import GradientCard from "@shared/components/ui/card/GradientCard";
import WalletListMobile from "@wallet/wallet-list-mobile";
import FeaturesBtn from "@wallet/overview-section/FeaturesBtn";
import HeaderNFT from "@wallet/overview-section/HeaderNFT";
import HeaderToken from "@wallet/overview-section/HeaderToken";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";
import TotalCountToken from "@shared/components/total-count-token";

const OverviewSection = () => {
  const [searchParams] = useSearchParams();
  const token = useAtomValue(TokenSelectedAtom);

  return (
    <GradientCard className="p-4 xl:p-8 relative">
      <div className="flex flex-col items-center">
        <div className="pb-8 xl:pb-16">
          <div className="flex flex-col items-center">
            {searchParams.get("token") === "nft" ? (
              <>
                <HeaderNFT className="hidden xl:block mb-8 xl:mb-12" />
                <TotalCountUserNFTs />
              </>
            ) : (
              <>
                <HeaderToken
                  className="hidden xl:block mb-8 xl:mb-12"
                  token={token}
                />
                <TotalCountToken token={token} />
              </>
            )}
          </div>

          <WalletListMobile className="flex justify-center xl:hidden mt-6 mb-8" />
        </div>
      </div>

      <div className={clsx("absolute -bottom-9 left-1/2 -translate-x-1/2")}>
        <FeaturesBtn />
      </div>
    </GradientCard>
  );
};

export default OverviewSection;
