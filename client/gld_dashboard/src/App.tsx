import "@nfid/identitykit/react/styles.css";
import "./App.css";
import { Tooltip } from "@components/index";

import Router from "./router.tsx";

function App() {
  return (
    <>
      <Router />
      <Tooltip id="tooltip" />
    </>
  );
}

export default App;
