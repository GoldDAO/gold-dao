import { Suspense } from "react";
import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import AppLayout from "@components/outlets/AppLayout";

import { NotFound, LoadingNavigation } from "views/index";

import BuyGLDT from "apps/buy-gldt";
import Earn from "apps/earn";
import Balance from "apps/balance";
import routesDashboard from "apps/dashboard/routes/index";
import { routes as routesStake } from "apps/stake/routes";

const router = createBrowserRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      {
        path: "buy-gldt",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <BuyGLDT />
              </Suspense>
            ),
          },
        ],
      },
      {
        path: "earn",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <Earn />
              </Suspense>
            ),
          },
        ],
      },
      {
        path: "balance",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <Balance />
              </Suspense>
            ),
          },
        ],
      },
      ...routesDashboard,
      ...routesStake,
      {
        path: "*",
        element: <NotFound />,
      },
    ],
  },
]);

const RouterProvider = () => {
  return (
    <ReactRouterProvider
      router={router}
      fallbackElement={<LoadingNavigation />}
    />
  );
};

export default RouterProvider;
