import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import SideNav from "@advanced/side-nav";
import OverviewSection from "@advanced/gldt/overview-section";
import UserNFTsSection from "@advanced/gldt/user-nfts-section";
// import TxHistory from "./tx-history";
import NFTMaintenanceMode from "@shared/components/nft-maintenance-mode";
import useGetGLDDashboardMaintenanceMode from "@shared/hooks/useGetGLDDashboardMaintenanceMode";
import { GLD_DASHBOARD_MAINTENANCE_MODE_CANISTER_ID } from "@constants";

const AdvancedGLDT = () => {
  const { unauthenticatedAgent } = useAuth();
  const { data: maintenanceMode } = useGetGLDDashboardMaintenanceMode(
    GLD_DASHBOARD_MAINTENANCE_MODE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );
  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <SideNav />
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        {maintenanceMode ? (
          <NFTMaintenanceMode />
        ) : (
          <>
            <OverviewSection />
            <div className="p-4 xl:p-8 mt-12">
              <UserNFTsSection className="mb-0" />
            </div>
          </>
        )}
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default AdvancedGLDT;
