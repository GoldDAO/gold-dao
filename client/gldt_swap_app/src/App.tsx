import "./App.css";
import { Tooltip } from "@components/ui";

import RouterProvider from "@providers/Router.tsx";

function App() {
  return (
    <>
      <RouterProvider />
      <Tooltip id="tooltip" />
    </>
  );
}

export default App;
