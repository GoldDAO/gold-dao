import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import Layout from "@components/shared/Layout";
import Protected from "@components/shared/routes/Protected";
import NotFound from "@components/shared/routes/NotFound";

import { SwapAppProvider } from "@context/index";
import { TransactionDetailsProvider } from "@context/index";

import {
  LandingPage,
  SwapTransfer,
  Account,
  TransactionDetails,
  TransactionHistoryList,
  Faqs,
  Explorer,
  AccountOverview,
  AccountTransactionDetails,
} from "@pages/index";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <LandingPage />,
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
        path: "faqs",
        children: [
          {
            index: true,
            element: <Faqs />,
          },
        ],
      },
      {
        path: "explorer",
        children: [
          {
            index: true,
            element: <Explorer />,
          },
          {
            path: "account",
            element: <AccountOverview />,
          },
          {
            path: "transaction/:index",
            element: <AccountTransactionDetails />,
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
