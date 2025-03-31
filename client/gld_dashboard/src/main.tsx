// import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";
import "./i18n";

import { APP_MODE, GLDT_STAKE_CANISTER_ID } from "@constants";

import App from "./App";

import AuthProvider from "@auth/provider";

import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  SWAP_CANISTER_ID,
} from "@constants";

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <>
    <Toaster
      position="bottom-center"
      reverseOrder={false}
      toastOptions={{
        duration: 3000,
        // style: {
        //   background: themeColors.surface[2],
        //   color: themeColors.content,
        // },
        success: {
          duration: 3000,
        },
        error: {
          duration: 4000,
        },
      }}
    />

    <QueryClientProvider client={queryClient}>
      <AuthProvider
        derivationOrigin={
          ["preprod", "production"].includes(APP_MODE)
            ? "https://rbsh4-yyaaa-aaaal-qdigq-cai.icp0.io"
            : undefined
        }
        targets={[
          GLD_NFT_1G_CANISTER_ID,
          GLD_NFT_10G_CANISTER_ID,
          GLD_NFT_100G_CANISTER_ID,
          GLD_NFT_1000G_CANISTER_ID,
          SWAP_CANISTER_ID,
          // GLD_GOV_LEDGER_CANISTER_ID,
          // OGY_LEDGER_CANISTER_ID,
          GLDT_LEDGER_CANISTER_ID,
          GLDT_STAKE_CANISTER_ID,
        ]}
      >
        <App />
      </AuthProvider>
    </QueryClientProvider>
  </>
);
