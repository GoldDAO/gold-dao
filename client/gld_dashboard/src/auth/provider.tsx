import { ReactNode, useEffect, useState } from "react";
import {
  IdentityKitAuthType,
  NFIDW,
  InternetIdentity,
  IdentityKitSignerConfig,
} from "@nfid/identitykit";
import { useAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";
import {
  IdentityKitProvider,
  // useIdentity,
  useAuth,
  useIsInitializing,
  useAgent,
} from "@nfid/identitykit/react";
import { Agent, HttpAgent } from "@dfinity/agent";
// import { isMobile } from "react-device-detect";

import authStateAtom from "./atoms";

// const ICP_API_HOST = "https://icp-api.io/";

const AuthProviderInit = ({ children }: { children: ReactNode }) => {
  const { user } = useAuth();
  // const identity = useIdentity();
  const isInitializing = useIsInitializing();
  const [state, setState] = useAtom(authStateAtom);
  const [unauthenticatedAgent, setUnauthenticatedAgent] = useState<
    HttpAgent | Agent | undefined
  >();
  const authenticatedAgent = useAgent({ host: "https://ic0.app" });

  // const authenticatedAgent = HttpAgent.createSync({
  //   host: "https://ic0.app",
  //   identity: identity,
  // });

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

const AuthProvider = ({
  children,
  targets = [],
  signers = [NFIDW, InternetIdentity],
  derivationOrigin = undefined,
  maxTimeToLive = 604800000000000n, // ? one week
}: {
  children: ReactNode;
  targets?: string[];
  signers?: IdentityKitSignerConfig[];
  derivationOrigin?: string | undefined;
  maxTimeToLive?: bigint;
}) => {
  // const setState = useSetAtom(authStateAtom);
  const queryClient = useQueryClient();

  return (
    <IdentityKitProvider
      // signers={isMobile ? [NFIDW, InternetIdentity] : signers}
      signers={signers}
      authType={IdentityKitAuthType.DELEGATION}
      signerClientOptions={{
        targets,
        maxTimeToLive,
        derivationOrigin,
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
