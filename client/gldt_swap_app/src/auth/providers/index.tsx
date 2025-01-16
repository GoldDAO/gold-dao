import { ReactNode, useEffect, useState } from "react";
import {
  IdentityKitAuthType,
  NFIDW,
  Plug,
  InternetIdentity,
  IdentityKitSignerConfig,
} from "@nfid/identitykit";
import { useAtom, useSetAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";
import {
  IdentityKitProvider,
  useAgent,
  useAuth,
  useIsInitializing,
  useIdentity,
} from "@nfid/identitykit/react";
import { Agent, HttpAgent } from "@dfinity/agent";
// import { isMobile } from "react-device-detect";

import { stateAtom } from "../atoms";
import { Canisters } from "../interfaces";
import { LoaderSpin } from "@components/ui";

const AuthProviderInit = ({
  canisters,
  children,
}: {
  canisters: Canisters;
  children: ReactNode;
}) => {
  const connected = localStorage.getItem("connected");

  const { user } = useAuth();
  const isInitializing = useIsInitializing();
  const HOST = "https://icp-api.io/"; // "https://ic0.app"
  const identity = useIdentity();
  const agent = HttpAgent.createSync({
    host: HOST,
    identity: identity,
  });

  const [state, setState] = useAtom(stateAtom);
  const [, setUnauthenticatedAgent] = useState<HttpAgent | undefined>();

  useEffect(() => {
    setUnauthenticatedAgent(agent);

    setState((prevState) => ({
      ...prevState,
      unauthenticatedAgent: agent,
      canisters: canisters,
    }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!isInitializing && connected === "1" && (!user || !agent)) {
      setState((prevState) => ({
        ...prevState,
        isConnecting: true,
      }));

      const timer = setTimeout(() => {
        if (!user || !agent) {
          localStorage.clear();
          setState((prevState) => ({
            ...prevState,
            isConnecting: false,
          }));
        }
      }, 3000);
      return () => clearTimeout(timer);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [connected, user, agent, isInitializing]);

  useEffect(() => {
    if (user && agent) {
      setState((prevState) => ({
        ...prevState,
        principalId: user.principal.toText(),
        isConnected: true,
        isConnecting: false,
        authenticatedAgent: agent as unknown as Agent,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        principalId: "",
        isConnected: false,
        authenticatedAgent: undefined,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [user, state.canisters]);

  if (!Object.keys(state.canisters).length || !state.unauthenticatedAgent) {
    return (
      <div className="flex h-screen">
        <div className="m-auto">
          <LoaderSpin />
        </div>
      </div>
    );
  } else return children;
};

export const AuthProvider = ({
  children,
  targets = [],
  signers = [NFIDW, Plug, InternetIdentity],
  canisters = {},
  derivationOrigin = undefined,
  maxTimeToLive = 604800000000000n, // ? one week
}: {
  children: ReactNode;
  targets?: string[];
  signers?: IdentityKitSignerConfig[];
  canisters: Canisters;
  derivationOrigin?: string | undefined;
  maxTimeToLive?: bigint;
}) => {
  const setState = useSetAtom(stateAtom);
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
        // window.location.reload();
        console.log(err);
      }}
      onConnectSuccess={() => {
        queryClient.clear();
      }}
      onDisconnect={() => {
        setState((prevState) => ({
          ...prevState,
          principalId: "",
          isConnected: false,
          isConnecting: false,
          authenticatedAgent: undefined,
        }));
        // window.location.reload();
      }}
    >
      <AuthProviderInit canisters={canisters}>{children}</AuthProviderInit>
    </IdentityKitProvider>
  );
};
