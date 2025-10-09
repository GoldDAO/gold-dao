import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import Layout from "@components/shared/Layout";
import Protected from "@components/shared/routes/Protected";
import NotFound from "@components/shared/routes/NotFound";

import { SwapAppProvider } from "@context/index";
import { TransactionDetailsProvider } from "@context/index";

import Home from "features/home";

import {
  SwapTransfer,
  Account,
  TransactionDetails,
  TransactionHistoryList,
  Explorer,
  AccountOverview,
  AccountTransactionDetails,
} from "features/index";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "explorer",
        children: [
          {
            index: true,
            element: <Explorer />,
          },
          {
            path: "transactions/account",
            element: <AccountOverview />,
          },
          {
            path: "top_holders/account",
            element: <AccountOverview />,
          },
          {
            path: "transaction/:index",
            element: <AccountTransactionDetails />,
          },
        ],
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
                    element: (
                      <TransactionDetailsProvider>
                        <TransactionDetails />
                      </TransactionDetailsProvider>
                    ),
                  },
                ],
              },
            ],
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
