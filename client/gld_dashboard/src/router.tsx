import { Suspense } from "react";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";
import AppLayout from "@shared/components/app-layout";
import LoadingNavigation from "@shared/components/LoadingNavigation";
import NotFound from "@shared/components/NotFound";
import Buy from "apps/buy";
import Earn from "apps/earn";
import Govern from "apps/govern";
import Wallet from "apps/wallet";
import AdvancedGLDT from "apps/advanced/gldt";

const router = createBrowserRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      {
        index: true,
        element: <Navigate to="/buy" replace />,
      },
      {
        path: "buy",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <Buy />
              </Suspense>
            ),
          },
        ],
      },
      {
        path: "earn-feature-coming-soon",
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
      {
        path: "advanced/gldt",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <AdvancedGLDT />
              </Suspense>
            ),
          },
        ],
      },
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
