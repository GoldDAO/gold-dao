import type { Principal } from "@dfinity/principal";
import type { ActorMethod } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";

export type Args = { Upgrade: UpgradeArgs } | { Init: InitArgs };
export interface BuildVersion {
  major: number;
  minor: number;
  patch: number;
}
export interface InitArgs {
  test_mode: boolean;
  authorized_principals: Array<Principal>;
  version: BuildVersion;
  commit_hash: string;
}
export type Result = { Ok: string } | { Err: string };
export interface SupportedStandard {
  url: string;
  name: string;
}
export interface UpgradeArgs {
  version: BuildVersion;
  commit_hash: string;
}
export interface _SERVICE {
  dex_transfer_position_validate: ActorMethod<
    [Principal, Principal, bigint],
    Result
  >;
  get_gld_dashboard_maintenance_mode: ActorMethod<[], boolean>;
  icrc10_supported_standards: ActorMethod<[], Array<SupportedStandard>>;
  update_gld_dashboard_maintenance_mode: ActorMethod<[boolean], null>;
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
