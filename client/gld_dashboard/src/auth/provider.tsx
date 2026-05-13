import { ReactNode, useEffect, useState } from "react";
import {
  // IdentityKitAuthType,
  NFIDW,
  InternetIdentity,
  OISY,
} from "@amerej/identitykit";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";
import {
  IdentityKitProvider,
  useAuth,
  useIsInitializing,
  useAgent,
} from "@amerej/identitykit/react";
import { Agent, HttpAgent } from "@dfinity/agent";
import {
  APP_MODE,
  GLDT_STAKE_CANISTER_ID,
  SWAP_CANISTER_ID,
} from "@constants";
import authStateAtom from "./atoms";

// `@amerej/identitykit` v1.0.15 ships InternetIdentity with an empty providerUrl,
// so @dfinity/auth-client falls back to the deprecated identity.internetcomputer.org.
// Point it at id.ai with the guided-upgrade flag so existing II users can migrate.
const InternetIdentityIdAi = {
  ...InternetIdentity,
  providerUrl: "https://id.ai/?feature_flag_guided_upgrade=true",
};

const AuthProviderInit = ({ children }: { children: ReactNode }) => {
  const { user } = useAuth();
  const isInitializing = useIsInitializing();
  const [state, setState] = useAtom(authStateAtom);
  const [unauthenticatedAgent, setUnauthenticatedAgent] = useState<
    HttpAgent | Agent | undefined
  >();
  const authenticatedAgent = useAgent({ host: "https://ic0.app" });

  useEffect(() => {
    HttpAgent.create({ host: "https://ic0.app" }).then(setUnauthenticatedAgent);
  }, []);

  useEffect(() => {
    setState((prevState) => ({
      ...prevState,
      unauthenticatedAgent,
      authenticatedAgent,
    }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [unauthenticatedAgent, authenticatedAgent]);

  useEffect(() => {
    if (user) {
      setState((prevState) => ({
        ...prevState,
        principalId: user.principal.toText(),
        isConnected: true,
        isInitializing: false,
        authenticatedAgent,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        principalId: "",
        isConnected: false,
        isInitializing: false,
        authenticatedAgent: undefined,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [user]);

  if (isInitializing || (user && !state.isConnected)) {
    return (
      <div className="flex h-screen">
        <div className="m-auto">Loading...</div>
      </div>
    );
  } else return children;
};

const AuthProvider = ({ children }: { children: ReactNode }) => {
  const queryClient = useQueryClient();

  return (
    <IdentityKitProvider
      signers={[NFIDW, InternetIdentityIdAi, OISY]}
      signerClientOptions={{
        targets: [
          SWAP_CANISTER_ID,
          GLDT_STAKE_CANISTER_ID,
        ],
        maxTimeToLive: 604800000000000n,
        derivationOrigin: ["preprod", "production"].includes(APP_MODE)
          ? "https://rbsh4-yyaaa-aaaal-qdigq-cai.icp0.io"
          : undefined,
        idleOptions: {
          disableIdle: false,
        },
      }}
      onConnectFailure={(err: Error) => {
        console.log(err);
      }}
      onConnectSuccess={() => {
        queryClient.clear();
      }}
      onDisconnect={() => {}}
    >
      <AuthProviderInit>{children}</AuthProviderInit>
    </IdentityKitProvider>
  );
};

export default AuthProvider;
