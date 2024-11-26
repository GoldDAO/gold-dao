import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";

import "@nfid/identitykit/react/styles.css";

import { APP_MODE } from "@constants";
import { colors as themeColors } from "@theme/preset";
import App from "./App";

import { AuthProvider } from "./auth";

import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  GLDT_LEDGER_INDEXER_CANISTER_ID,
  SWAP_CANISTER_ID,
  ICP_SWAP_CANISTER_ID,
} from "@constants";

import { idlFactory as gld_nft_idl } from "@canisters/gld_nft/did";
import { idlFactory as gldt_swap_idl } from "@canisters/gldt_swap/did";
import { idlFactory as ledger_idl } from "@canisters/ledger/did";
import { idlFactory as icp_swap_idl } from "@canisters/icp_swap/did";
import { idlFactory as gldt_ledger_indexer_idl } from "@canisters/gldt_ledger_indexer/interface";

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <>
    <Toaster
      position="bottom-center"
      reverseOrder={false}
      toastOptions={{
        duration: 3000,
        style: {
          background: themeColors.surface[2],
          color: themeColors.content,
        },
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
          OGY_LEDGER_CANISTER_ID,
          GLDT_LEDGER_CANISTER_ID,
          ICP_SWAP_CANISTER_ID,
        ]}
        canisters={{
          gld_nft_1g: {
            canisterId: GLD_NFT_1G_CANISTER_ID,
            idlFactory: gld_nft_idl,
          },
          gld_nft_10g: {
            canisterId: GLD_NFT_10G_CANISTER_ID,
            idlFactory: gld_nft_idl,
          },
          gld_nft_100g: {
            canisterId: GLD_NFT_100G_CANISTER_ID,
            idlFactory: gld_nft_idl,
          },
          gld_nft_1000g: {
            canisterId: GLD_NFT_1000G_CANISTER_ID,
            idlFactory: gld_nft_idl,
          },
          gldt_swap: {
            canisterId: SWAP_CANISTER_ID,
            idlFactory: gldt_swap_idl,
          },
          gldt_ledger: {
            canisterId: GLDT_LEDGER_CANISTER_ID,
            idlFactory: ledger_idl,
          },
          gldt_ledger_indexer: {
            canisterId: GLDT_LEDGER_INDEXER_CANISTER_ID,
            idlFactory: gldt_ledger_indexer_idl,
          },
          ogy_ledger: {
            canisterId: OGY_LEDGER_CANISTER_ID,
            idlFactory: ledger_idl,
          },
          icp_swap: {
            canisterId: ICP_SWAP_CANISTER_ID,
            idlFactory: icp_swap_idl,
          },
        }}
      >
        <App />
      </AuthProvider>
    </QueryClientProvider>
  </>
);
