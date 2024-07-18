import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import Layout from "@components/shared/Layout";
import Protected from "@components/shared/routes/Protected";
import NotFound from "@components/shared/routes/NotFound";

import Swap from "@pages/swap/Swap";

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
            element: <Swap />,
          },
          {
            path: "transfer",
            element: <div>TRANSFER</div>,
          },
          {
            path: "account",
            element: <Protected />,
            children: [
              {
                index: true,
                element: <div>ACCOUNT</div>,
              },
              {
                path: "transactions",
                children: [
                  {
                    path: "swap/account/transactions/:index",
                    element: <div>ACCOUNT TX DETAILS</div>,
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
