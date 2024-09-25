import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
  useMemo,
} from "react";
import { IDL } from "@dfinity/candid";
import { IdentityKitDelegationType } from "@nfid/identitykit";
import { useIdentityKit } from "@nfid/identitykit/react";
import { Actor, ActorSubclass, HttpAgent } from "@dfinity/agent";

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
export const CANISTERS: Canisters = {
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
    idlFactory: icp_swap_idl,
  },
};

export interface AuthState {
  isConnected: boolean;
  isConnecting: boolean;
  principalId: string;
}

const initialState: AuthState = {
  isConnected: false,
  isConnecting: false,
  principalId: "",
};

const AuthContext = createContext<ReturnType<
  typeof useAuthProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within a AuthProvider");
  }
  return context;
};

const useAuthProviderValue = () => {
  const [state, setState] = useState<AuthState>(initialState);
  const connected = localStorage.getItem("connected") || "";
  const [unauthenticatedAgent, setUnauthenticatedAgent] = useState<
    HttpAgent | undefined
  >();
  const [authenticatedNonTargetAgent, setAuthenticatedNonTargetAgent] =
    useState<HttpAgent | undefined>();
  const {
    user,
    agent,
    identity,
    connect,
    disconnect: disconnectIK,
    delegationType,
  } = useIdentityKit();

  useEffect(() => {
    HttpAgent.create({ host: "https://icp-api.io/" }).then(
      setUnauthenticatedAgent
    );
  }, []);

  useEffect(() => {
    if (identity && delegationType === IdentityKitDelegationType.ANONYMOUS) {
      HttpAgent.create({ identity, host: "https://icp-api.io/" }).then(
        setAuthenticatedNonTargetAgent
      );
    }
  }, [identity, delegationType]);

  useEffect(() => {
    if (user && authenticatedNonTargetAgent && agent) {
      setState((prevState) => ({
        ...prevState,
        principalId: user.principal.toText(),
        isConnected: true,
        isConnecting: false,
      }));
    }
  }, [user, authenticatedNonTargetAgent, agent]);

  useEffect(() => {
    if (!!user && connected !== undefined && connected === "1") {
      setState((prevState) => ({
        ...prevState,
        isConnecting: true,
      }));
    }
  }, [user, connected]);

  const getActor = (
    canisterName:
      | string
      | "gld_nft_1g"
      | "gld_nft_10g"
      | "gld_nft_100g"
      | "gld_nft_1000g"
      | "gldt_swap"
      | "gldt_ledger"
      | "ogy_ledger"
      | "icp_swap",
    options: { authenticated: boolean } = { authenticated: true }
  ): ActorSubclass => {
    const { canisterId, idlFactory } = CANISTERS[canisterName];

    const nonTargetActor =
      identity &&
      delegationType === IdentityKitDelegationType.ANONYMOUS &&
      authenticatedNonTargetAgent &&
      Actor.createActor(idlFactory, {
        agent: authenticatedNonTargetAgent,
        canisterId,
      });

    const nonTargetUnauthenticatedActor =
      unauthenticatedAgent &&
      Actor.createActor(idlFactory, {
        agent: unauthenticatedAgent,
        canisterId: canisterId,
      });

    const actor = options.authenticated
      ? nonTargetActor
      : nonTargetUnauthenticatedActor;
    return actor as ActorSubclass;
  };

  const disconnect = () => {
    disconnectIK();
    setState(initialState);
  };

  const value = useMemo(
    () => ({
      state,
      connect,
      disconnect,
      getActor,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state]
  );
  return value;
};

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const contextValue = useAuthProviderValue();

  return (
    <AuthContext.Provider value={contextValue}>{children}</AuthContext.Provider>
  );
};
