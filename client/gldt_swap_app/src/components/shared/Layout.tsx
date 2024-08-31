// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import { useCallback } from "react";
import type { Location, useMatches } from "react-router-dom";
import {
  ScrollRestoration,
  Outlet,
  useNavigation,
  useLocation,
} from "react-router-dom";

import NavbarHome from "./navbars/Home";
import NavbarSwap from "./navbars/Swap";

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

  const getKey = useCallback(
    (location: Location, matches: ReturnType<typeof useMatches>) => {
      const match = matches.find((m) => m.handle?.scrollMode);
      if (match?.handle?.scrollMode === "pathname") {
        return location.pathname;
      }
      return location.key;
    },
    []
  );

  return (
    <div className="flex flex-col bg-cover-img bg-cover bg-fixed bg-background min-h-screen pb-12">
      {isSwap ? <NavbarSwap /> : <NavbarHome />}
      <div className="flex-grow px-4 sm:px-6">
        {navigation.state !== "idle" ? <NavigationProgress /> : <Outlet />}
      </div>
      {/* <ScrollRestoration
        getKey={(location, matches) => {
          return location.pathname;
        }}
      /> */}
    </div>
  );
};

export default Layout;
