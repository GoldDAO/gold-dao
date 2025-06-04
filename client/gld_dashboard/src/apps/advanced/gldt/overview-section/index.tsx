import clsx from "clsx";
import { Logo } from "@components/index";
import GradientCard from "@shared/components/ui/card/GradientCard";
import TotalCountUserNFTs from "@advanced/gldt/overview-section/total-count-user-nfts";
import ActionBtns from "@advanced/gldt/overview-section/action-btns";
import TotalCountGLDT from "@advanced/gldt/overview-section/total-count-gldt";

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
          {/* <div className="text-sm text-content/60">1 {name} ≈ $(todo)</div> */}
        </div>
        <div className="py-8 xl:py-12">
          <div className="flex items-center gap-4 xl:gap-8 justify-center">
            <TotalCountUserNFTs />
            <div className="border-l border-border"></div>
            <TotalCountGLDT />
          </div>
        </div>
      </div>
      <div
        className={clsx(
          "my-4",
          "xl:absolute xl:-bottom-9 xl:left-1/2 xl:my-0 xl:-translate-x-1/2"
        )}
      >
        <ActionBtns />
      </div>
    </GradientCard>
  );
};

export default OverviewSection;
