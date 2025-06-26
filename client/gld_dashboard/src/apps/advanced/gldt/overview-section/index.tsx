import clsx from "clsx";
import { Logo } from "@components/index";
import GradientCard from "@shared/components/ui/card/GradientCard";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";
import FeaturesBtn from "@advanced/gldt/overview-section/features-btn";
import TotalCountToken from "@shared/components/total-count-token";
import { TokensList, GLDT_INDEX } from "@wallet/shared/utils";

const OverviewSection = () => {
  return (
    <GradientCard className="p-4 xl:p-8 relative">
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name="gldt" className="h-10 w-10" />
            <div>
              <div>GLDT</div>
              <div className="text-content/60 text-sm">Mint & Burn</div>
            </div>
          </div>
        </div>
        <div className="py-8 xl:py-12">
          <div className="flex items-stretch gap-4 xl:gap-8 justify-center">
            <TotalCountUserNFTs />
            <div className="border-l border-border h-auto my-2" />
            <TotalCountToken token={TokensList[GLDT_INDEX]} />
          </div>
        </div>
      </div>
      <div className={clsx("absolute -bottom-9 left-1/2 -translate-x-1/2")}>
        <FeaturesBtn />
      </div>
    </GradientCard>
  );
};

export default OverviewSection;
