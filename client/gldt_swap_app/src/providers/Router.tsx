import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import Layout from "@components/shared/Layout";
import Protected from "@components/shared/routes/Protected";
import NotFound from "@components/shared/routes/NotFound";

import SwapTransfer from "@pages/SwapTransfer";
import { SwapAppProvider } from "@context/index";

import Account from "@pages/Account";
import TransactionDetails from "@pages/TransactionDetails";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <div>HOME</div>,
      },
      {
        path: "swap",
        children: [
          {
            index: true,
            element: (
              <SwapAppProvider>
                <SwapTransfer />
              </SwapAppProvider>
            ),
          },
          {
            path: "account",
            element: <Protected />,
            children: [
              {
                index: true,
                element: <Account />,
              },
              {
                path: "transactions",
                children: [
                  {
                    path: ":nft_id",
                    element: <TransactionDetails />,
                  },
                ],
              },
            ],
          },
        ],
      },
      {
        path: "explorer",
        children: [
          {
            index: true,
            element: <div>EXPLORER</div>,
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
    <ReactRouterProvider router={router} fallbackElement={<p>Loading...</p>} />
  );
};

export default RouterProvider;
