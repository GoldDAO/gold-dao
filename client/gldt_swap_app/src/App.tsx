import "./App.css";
import { Tooltip } from "@components/ui";
import Auth from "@components/shared/Auth";

import RouterProvider from "@providers/Router.tsx";

function App() {
  return (
    <>
      <RouterProvider />
      <Tooltip id="tooltip" />
      <Auth />
    </>
  );
}

export default App;
