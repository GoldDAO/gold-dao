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
  useIdentityKit,
  useAgent,
} from "@nfid/identitykit/react";
import { HttpAgent } from "@dfinity/agent";
import { isMobile } from "react-device-detect";

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

  const { user, isInitializing } = useIdentityKit();

  const [state, setState] = useAtom(stateAtom);
  const [unauthenticatedAgent, setUnauthenticatedAgent] = useState<
    HttpAgent | undefined
  >();
  const agent = useAgent();

  useEffect(() => {
    HttpAgent.create({ host: "https://icp-api.io/" }).then((res) => {
      setState((prevState) => ({
        ...prevState,
        agent: res,
      }));
      setUnauthenticatedAgent(res);
    });
    setState((prevState) => ({
      ...prevState,
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
        agent,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        principalId: "",
        isConnected: false,
        agent: unauthenticatedAgent,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [user, agent, state.canisters, unauthenticatedAgent]);

  if (!Object.keys(state.canisters).length || !state.agent) {
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
      signers={isMobile ? [NFIDW, InternetIdentity] : signers}
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
        // console.log("connected");
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
        window.location.reload();
        // console.log("disconnected");
      }}
    >
      <AuthProviderInit canisters={canisters}>{children}</AuthProviderInit>
    </IdentityKitProvider>
  );
};
