import "./App.css";
import { Tooltip } from "@components/ui";

import ConnectingDialog from "@components/auth/ConnectingDialog";
import RouterProvider from "@providers/Router.tsx";

function App() {
  return (
    <>
      <RouterProvider />
      <Tooltip id="tooltip" />
      <ConnectingDialog />
    </>
  );
}

export default App;
