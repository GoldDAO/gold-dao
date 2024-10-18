import { ReactNode, useEffect } from "react";
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
  useIdentityKit,
  useTargetAuthenticatedAgent,
  // useNonTargetAuthenticatedAgent,
} from "@nfid/identitykit/react";
import { HttpAgent } from "@dfinity/agent";

import { stateAtom } from "../atoms";
import { Canisters } from "../interfaces";

const AuthProviderInit = ({
  canisters,
  children,
}: {
  canisters: Canisters;
  children: ReactNode;
}) => {
  const connected = localStorage.getItem("connected");

  const { user, isInitializing } = useIdentityKit();

  const [state, setState] = useAtom(stateAtom);
  const agent = useTargetAuthenticatedAgent();

  useEffect(() => {
    HttpAgent.create({ host: "https://icp-api.io/" }).then((res) => {
      setState((prevState) => ({
        ...prevState,
        agent: res,
      }));
    });
    setState((prevState) => ({
      ...prevState,
      canisters: canisters,
    }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!isInitializing && connected === "1") {
      setState((prevState) => ({
        ...prevState,
        isConnecting: true,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [connected, isInitializing]);

  useEffect(() => {
    if (user && agent) {
      setState((prevState) => ({
        ...prevState,
        principalId: user.principal.toText(),
        isConnected: true,
        isConnecting: false,
        agent,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        principalId: "",
        isConnected: false,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [user, agent, state.canisters]);

  return children;
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
      onConnectFailure={(e: Error) => {
        window.location.reload();
        console.log(e);
      }}
      onConnectSuccess={() => {
        console.log("connected");
        queryClient.clear();
      }}
      onDisconnect={() => {
        setState((prevState) => ({
          ...prevState,
          principalId: "",
          isConnected: false,
          isConnecting: false,
          agent: undefined,
        }));
        console.log("disconnected");
      }}
    >
      <AuthProviderInit canisters={canisters}>{children}</AuthProviderInit>
    </IdentityKitProvider>
  );
};
