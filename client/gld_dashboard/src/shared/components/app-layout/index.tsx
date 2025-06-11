import { Outlet, useNavigation } from "react-router-dom";
import TopNav from "@shared/components/app-layout/navbars/top-nav";
import SideNav from "@shared/components/app-layout/navbars/side-nav";
import BottomNav from "@shared/components/app-layout/navbars/bottom-nav";

const NavigationProgress = () => {
  return (
    <div className="flex items-center justify-center my-32 xl:my-64">
      <div className="text-center">
        <div className="border-4 xl:border-8 border-accent/20 border-t-accent h-16 w-16 xl:h-32 xl:w-32 animate-spin rounded-full" />
      </div>
    </div>
  );
};

const AppLayout = () => {
  const navigation = useNavigation();

  return (
    <div className="xl:bg-[url(/src/assets/bg-cover.png)] bg-cover bg-fixed bg-background">
      <div className="flex flex-col h-screen overflow-y-auto">
        <TopNav className="sticky top-0 bg-background py-5 px-4 xl:px-16 z-40" />
        <div className="xl:container flex flex-col xl:mx-auto items-center flex-grow xl:h-100">
          <div className="relative bg-background xl:rounded-xl flex xl:border-y xl:border-r border-border xl:shadow-xl w-full xl:h-[864px] xl:my-auto overflow-y-hidden">
            <SideNav className="hidden bg-surface-primary xl:block p-3 border-r border-border rounded-l-[inherit]" />
            {navigation.state !== "idle" ? <NavigationProgress /> : <Outlet />}
            <div className="absolute bottom-0 left-0 h-full w-full xl:bg-linear-to-tr xl:from-primary/8 xl:from-20% xl:via-transparent xl:to-transparent xl:to-100% pointer-events-none" />
          </div>
        </div>
        <div className="py-5 px-4 xl:px-16">
          <BottomNav />
        </div>
      </div>
    </div>
  );
};

export default AppLayout;
