import InnerAppLayout from "@shared/components/app-layout/inner-app";
import SideNav from "@advanced/side-nav";
import OverviewSection from "@advanced/gldt/overview-section";
import TxSection from "@advanced/gldt/tx-section";
import UserNFTsSection from "@advanced/gldt/user-nfts-section";

const AdvancedGLDT = () => {
  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <SideNav />
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <OverviewSection />
        <div className="p-4 xl:p-8 mt-12">
          <UserNFTsSection className="mb-8" />
          <TxSection />
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default AdvancedGLDT;
