import { useWalletInit } from "@amerej/artemis-react";

import "./App.css";
import RouterProvider from "@providers/Router.tsx";

function App() {
  useWalletInit({
    host: "https://identity.ic0.app",
    derivationOrigin: "",
    whitelist: [],
    canisters: {},
  });
  return (
    <>
      <RouterProvider />
    </>
  );
}

export default App;
