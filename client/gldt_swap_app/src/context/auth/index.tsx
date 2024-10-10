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
import { DelegationIdentity } from "@dfinity/identity";

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
    identity,
    connect,
    disconnect: disconnectIK,
    isInitializing,
  } = useIdentityKit();

  useEffect(() => {
    HttpAgent.create({ host: "https://icp-api.io/" }).then(
      setUnauthenticatedAgent
    );
  }, []);

  useEffect(() => {
    if (!isInitializing && !user && !connected) {
      setState((prevState) => ({
        ...prevState,
        isConnecting: false,
      }));
    }
  }, [connected, isInitializing, user]);

  useEffect(() => {
    if (!isInitializing && !user && connected === "1") {
      setState((prevState) => ({
        ...prevState,
        isConnecting: true,
      }));
    }
  }, [connected, isInitializing, user]);

  useEffect(() => {
    // ? removed condition for plug delegationType === IdentityKitDelegationType.ANONYMOUS
    if (user && identity instanceof SignIdentity) {
      (async function () {
        await HttpAgent.create({ identity, host: "https://icp-api.io/" }).then(
          setAuthenticatedNonTargetAgent
        );
      })();
    }
  }, [identity, user]);

  useEffect(() => {
    if (
      user &&
      authenticatedNonTargetAgent &&
      authenticatedNonTargetAgent?.config?.identity instanceof
        DelegationIdentity
    ) {
      setState((prevState) => ({
        ...prevState,
        principalId: user.principal.toText(),
        isConnected: true,
        isConnecting: false,
      }));
    }
  }, [user, authenticatedNonTargetAgent]);

  useEffect(() => {
    if (
      state.isConnected &&
      !(
        identity instanceof SignIdentity &&
        authenticatedNonTargetAgent?.config?.identity instanceof
          DelegationIdentity
      )
    ) {
      console.log("Lost Delegation Identity");
      setState((prevState) => ({
        ...prevState,
        isConnected: false,
      }));
      disconnect();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [identity, authenticatedNonTargetAgent, state.isConnected]);

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

  const disconnect = () => {
    disconnectIK();
    setState({
      isInitializing: false,
      isConnected: false,
      isConnecting: false,
      principalId: "",
    });
  };

  // console.log(user);
  // console.log(identity);
  // console.log(agent);
  // console.log(authenticatedNonTargetAgent);

  const value = useMemo(
    () => ({
      state,
      connect,
      disconnect,
      getActor,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state, identity, isInitializing]
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
