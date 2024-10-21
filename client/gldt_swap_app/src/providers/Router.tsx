import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import Layout from "@components/shared/Layout";
import Protected from "@components/shared/routes/Protected";
import NotFound from "@components/shared/routes/NotFound";

import { SwapAppProvider } from "@context/index";

import {
  LandingPage,
  SwapTransfer,
  Account,
  TransactionDetails,
  TransactionHistoryList,
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
