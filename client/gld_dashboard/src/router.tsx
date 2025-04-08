import { Suspense } from "react";
import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import AppLayout from "@components/outlets/AppLayout";

import { NotFound, LoadingNavigation } from "views/index";

import BuyGLDT from "apps/buy-gldt";
import Earn from "apps/earn";
import Govern from "apps/govern";
import Wallet from "apps/wallet";

// import routesDashboard from "apps/dashboard/routes/index";

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
        path: "govern",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <Govern />
              </Suspense>
            ),
          },
        ],
      },
      {
        path: "wallet",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <Wallet />
              </Suspense>
            ),
          },
        ],
      },
      // ...routesDashboard,
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
