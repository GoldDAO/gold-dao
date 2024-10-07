import { ReactNode } from "react";
import {
  IdentityKitAuthType,
  NFIDW,
  Plug,
  InternetIdentity,
} from "@nfid/identitykit";
import { IdentityKitProvider } from "@nfid/identitykit/react";
import { IDL } from "@dfinity/candid";

import { AuthProvider } from "@context/auth";

interface Canisters {
  [canisterName: string]: {
    canisterId: string;
    idlFactory: IDL.InterfaceFactory;
  };
}

const IKProvider = ({
  children,
  targets,
  canisters,
}: {
  children: ReactNode;
  targets?: string[];
  canisters: Canisters;
}) => {
  return (
    <IdentityKitProvider
      signers={[NFIDW, Plug, InternetIdentity]}
      authType={IdentityKitAuthType.DELEGATION}
      signerClientOptions={{
        targets: targets ?? [],
        derivationOrigin: "https://oj7ri-2qaaa-aaaap-abrzq-cai.icp0.io",
        maxTimeToLive: 604800000000000n, // ? one week
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
      }}
      onDisconnect={() => {
        console.log("disconnected");
      }}
    >
      <AuthProvider canisters={canisters}>{children}</AuthProvider>
    </IdentityKitProvider>
  );
};

export default IKProvider;
