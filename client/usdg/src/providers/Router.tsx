import {
  createBrowserRouter,
  RouterProvider as ReactRouterProvider,
} from "react-router-dom";

import { NotFound } from "@components/index";
import { ComingSoon } from "@pages/index";

const router = createBrowserRouter([
  {
    path: "/",
    children: [
      {
        index: true,
        element: <ComingSoon />,
      },
      {
        path: "*",
        element: <NotFound />,
      },
    ],
  },
]);

export const RouterProvider = () => {
  return (
    <ReactRouterProvider router={router} fallbackElement={<p>Loading...</p>} />
  );
};
