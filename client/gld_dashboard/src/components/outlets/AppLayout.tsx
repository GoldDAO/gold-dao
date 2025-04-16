import { Outlet, useNavigation } from "react-router-dom";

import TopNav from "@components/navbars/TopNav";
import SideNav from "@components/navbars/SideNav";
import BottomNav from "@components/navbars/BottomNav";

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
    <div className="lg:bg-[url(/src/assets/bg-cover.png)] bg-cover bg-fixed bg-background">
      <div className="flex flex-col h-screen overflow-y-auto">
        <TopNav className="sticky top-0 bg-background py-5 px-4 lg:px-16 z-40" />
        <div className="lg:container flex flex-col lg:mx-auto items-center flex-grow lg:h-100">
          <div className="relative bg-background lg:rounded-xl flex lg:border-y lg:border-r border-border lg:shadow-xl w-full lg:h-[864px] lg:my-auto overflow-y-hidden">
            <SideNav className="hidden bg-surface-primary lg:block p-4 border-r border-border rounded-l-[inherit]" />
            {navigation.state !== "idle" ? <NavigationProgress /> : <Outlet />}
            <div className="absolute bottom-0 left-0 h-full w-full bg-linear-to-tr from-primary/8 from-20% via-transparent to-transparent to-100% pointer-events-none" />
          </div>
        </div>
        <div className="py-5 px-4 lg:px-16">
          <BottomNav />
        </div>
      </div>
    </div>
  );
};

export default AppLayout;
