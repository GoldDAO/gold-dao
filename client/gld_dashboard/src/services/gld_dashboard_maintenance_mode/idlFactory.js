export const idlFactory = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const Args = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  const Result = IDL.Variant({ Ok: IDL.Text, Err: IDL.Text });
  const SupportedStandard = IDL.Record({ url: IDL.Text, name: IDL.Text });
  return IDL.Service({
    dex_transfer_position_validate: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat],
      [Result],
      ["query"]
    ),
    get_gld_dashboard_maintenance_mode: IDL.Func([], [IDL.Bool], ["query"]),
    icrc10_supported_standards: IDL.Func(
      [],
      [IDL.Vec(SupportedStandard)],
      ["query"]
    ),
    update_gld_dashboard_maintenance_mode: IDL.Func([IDL.Bool], [IDL.Null], []),
  });
};
export const init = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const Args = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  return [Args];
};
