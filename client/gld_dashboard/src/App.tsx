import "@nfid/identitykit/react/styles.css";
import "./App.css";
import { useEffect } from "react";
import { Tooltip } from "@components/index";

import Router from "./router.tsx";

function App() {
  //! FIX Hide PLUG element from NFID IdentityKit
  useEffect(() => {
    const hideElement = () => {
      const el = document.getElementById(
        "signer_71edc834-bab2-4d59-8860-c36a01fee7b8"
      );
      if (el && el.parentElement) {
        el.parentElement.style.display = "none";
      }
    };
    hideElement();
    const observer = new MutationObserver(hideElement);
    observer.observe(document.body, { childList: true, subtree: true });
    return () => observer.disconnect();
  }, []);
  return (
    <>
      <Router />
      <Tooltip id="tooltip" className="max-w-xs xl:max-w-md" />
    </>
  );
}

export default App;
