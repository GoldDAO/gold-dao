import { Outlet, useNavigation, useLocation } from "react-router-dom";
import NavbarSwap from "./navbars/swap/NavbarSwap";

const NavigationProgress = () => {
  return (
    <div className="flex items-center justify-center my-32 xl:my-64">
      <div className="text-center">
        <div className="border-4 xl:border-8 border-accent/20 border-t-accent h-16 w-16 xl:h-32 xl:w-32 animate-spin rounded-full" />
      </div>
    </div>
  );
};

const Layout = () => {
  const navigation = useNavigation();
  const location = useLocation();
  const isSwap = location.pathname.match(/\/swap/) ? true : false;

  // useEffect(() => {
  //   if (isSwap) document.body.classList.add("bg-background");
  //   else document.body.classList.remove("bg-surface-1");
  // }, [isSwap]);

  return isSwap ? (
    <div className="flex flex-col min-h-screen pb-12 bg-cover-img bg-cover bg-fixed bg-background">
      <NavbarSwap />
      <div className="flex-grow px-4 sm:px-6">
        {navigation.state !== "idle" ? <NavigationProgress /> : <Outlet />}
      </div>
    </div>
  ) : (
    <div className={`flex flex-col min-h-screen`}>
      <div className="flex-grow">
        {navigation.state !== "idle" ? <NavigationProgress /> : <Outlet />}
      </div>
    </div>
  );
};

export default Layout;
