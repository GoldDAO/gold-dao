import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
  useMemo,
} from "react";
import { IDL } from "@dfinity/candid";
// import { IdentityKitDelegationType } from "@nfid/identitykit";
import { useIdentityKit } from "@nfid/identitykit/react";
import { Actor, ActorSubclass, HttpAgent, SignIdentity } from "@dfinity/agent";

interface Canisters {
  [canisterName: string]: {
    canisterId: string;
    idlFactory: IDL.InterfaceFactory;
  };
}

export interface AuthState {
  isInitializing: boolean;
  isConnected: boolean;
  isConnecting: boolean;
  principalId: string;
}

const initialState: AuthState = {
  isInitializing: true,
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

const useAuthProviderValue = ({ canisters }: { canisters: Canisters }) => {
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
    isInitializing,
    isUserConnecting,
  } = useIdentityKit();
  // console.log(user);
  // console.log(identity);
  // console.log(agent);
  // console.log(authenticatedNonTargetAgent);

  useEffect(() => {
    if (!isInitializing && !user && connected !== "1") {
      setState((prevState) => ({
        ...prevState,
        isInitializing: false,
      }));
    }
  }, [connected, isInitializing, user]);

  useEffect(() => {
    if (!isInitializing && !!user && connected === "1") {
      setState((prevState) => ({
        ...prevState,
        isConnecting: true,
      }));
    }
  }, [connected, isInitializing, user]);

  useEffect(() => {
    if (isUserConnecting || (!!user && connected === "1")) {
      setState((prevState) => ({
        ...prevState,
        isConnecting: true,
        isInitializing,
      }));
    }
  }, [isUserConnecting, user, connected, isInitializing]);

  useEffect(() => {
    HttpAgent.create({ host: "https://icp-api.io/" }).then(
      setUnauthenticatedAgent
    );
  }, []);

  // ? remove this condition for plug delegationType === IdentityKitDelegationType.ANONYMOUS
  useEffect(() => {
    if (user && agent && identity) {
      HttpAgent.create({ identity, host: "https://icp-api.io/" }).then(
        setAuthenticatedNonTargetAgent
      );
    }
  }, [identity, delegationType, user, agent]);

  useEffect(() => {
    if (user && authenticatedNonTargetAgent) {
      setState((prevState) => ({
        ...prevState,
        principalId: user.principal.toText(),
        isConnected: true,
        isConnecting: false,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        principalId: "",
        isConnected: false,
      }));
    }
  }, [user, authenticatedNonTargetAgent, agent]);

  const getActor = (
    canister:
      | string
      | "gld_nft_1g"
      | "gld_nft_10g"
      | "gld_nft_100g"
      | "gld_nft_1000g"
      | "gldt_swap"
      | "gldt_ledger"
      | "ogy_ledger"
      | "icp_swap"
  ): ActorSubclass => {
    const { canisterId, idlFactory } = canisters[canister];

    const actor = authenticatedNonTargetAgent
      ? Actor.createActor(idlFactory, {
          agent: authenticatedNonTargetAgent,
          canisterId,
        })
      : Actor.createActor(idlFactory, {
          agent: unauthenticatedAgent,
          canisterId: canisterId,
        });

    return actor as ActorSubclass;
  };

  useEffect(() => {
    if (state.isConnected && !(identity instanceof SignIdentity)) {
      console.log("Lost SignIdentity");
      disconnect();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [identity, state.isConnected]);

  const disconnect = () => {
    disconnectIK();
    setState({
      isInitializing: false,
      isConnected: false,
      isConnecting: false,
      principalId: "",
    });
  };

  const value = useMemo(
    () => ({
      state,
      connect,
      disconnect,
      getActor,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state, identity]
  );
  return value;
};

export const AuthProvider = ({
  children,
  canisters,
}: {
  children: ReactNode;
  canisters: Canisters;
}) => {
  const contextValue = useAuthProviderValue({ canisters });

  return (
    <AuthContext.Provider value={contextValue}>{children}</AuthContext.Provider>
  );
};
