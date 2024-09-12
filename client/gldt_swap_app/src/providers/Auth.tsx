import { ReactNode } from "react";
import { Provider } from "@amerej/artemis-react";
import { IDL } from "@dfinity/candid";

import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  SWAP_CANISTER_ID,
  ICP_SWAP_CANISTER_ID,
} from "@constants";

import { idlFactory as gld_nft_idl } from "@canisters/gld_nft/did";
import { idlFactory as gldt_swap_idl } from "@canisters/gldt_swap/did";
import { idlFactory as ledger_idl } from "@canisters/ledger/did";
import { idlFactory as icp_swap_idl } from "@canisters/icp_swap/did";

interface Canisters {
  [canisterName: string]: {
    canisterId: string;
    idlFactory: IDL.InterfaceFactory;
  };
}

// eslint-disable-next-line react-refresh/only-export-components
export const canisters: Canisters = {
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
  ogy_ledger: {
    canisterId: OGY_LEDGER_CANISTER_ID,
    idlFactory: ledger_idl,
  },
  icp_swap: {
    canisterId: ICP_SWAP_CANISTER_ID,
    idlFactory: icp_swap_idl
  }
};

const AuthProvider = ({ children }: { children: ReactNode }) => {
  return (
    <Provider
      host="https://identity.ic0.app"
      derivationOrigin=""
      whitelist={[
        GLD_NFT_1G_CANISTER_ID,
        GLD_NFT_10G_CANISTER_ID,
        GLD_NFT_100G_CANISTER_ID,
        GLD_NFT_1000G_CANISTER_ID,
        SWAP_CANISTER_ID,
        GLDT_LEDGER_CANISTER_ID,
        OGY_LEDGER_CANISTER_ID,
      ]}
    >
      {children}
    </Provider>
  );
};

export default AuthProvider;
