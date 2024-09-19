import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
  Navigate,
} from "react-router-dom";

import Layout from "@components/shared/Layout";
import Protected from "@components/shared/routes/Protected";
import NotFound from "@components/shared/routes/NotFound";

import SwapTransfer from "@pages/SwapTransfer";
import { SwapAppProvider } from "@context/index";

import Account from "@pages/Account";
import TransactionDetails from "@pages/TransactionDetails";
import TransactionHistoryList from "@pages/TransactionHistoryList";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Navigate to="/swap" replace />,
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
                    index: true,
                    element: <TransactionHistoryList />,
                  },
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
