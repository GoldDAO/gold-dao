/* eslint-disable camelcase */
/* eslint-disable no-unused-vars */

export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    authorized_principals: IDL.Vec(IDL.Principal),
  });
  return IDL.Service({
    get_gld_dashboard_maintenance_mode: IDL.Func([], [IDL.Bool], ['query']),
    update_gld_dashboard_maintenance_mode: IDL.Func([IDL.Bool], [IDL.Null], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    authorized_principals: IDL.Vec(IDL.Principal),
  });
  return [InitArgs];
};
