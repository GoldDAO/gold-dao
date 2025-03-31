import { RouteObject } from "react-router-dom";
import { CreateStakePosition, MyStakePositions, Stake } from "./views/stake";

export const routes: RouteObject[] = [
  {
    path: "stake",
    element: <Stake />,
    children: [
      {
        index: true,
        element: <CreateStakePosition />,
      },
      {
        path: "create",
        element: <CreateStakePosition></CreateStakePosition>,
      },
      {
        path: "my-positions",
        element: <MyStakePositions></MyStakePositions>,
      },
    ],
  },
];
