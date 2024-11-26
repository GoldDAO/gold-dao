import { decodeIcrcAccount } from "@dfinity/ledger-icrc";

export const getAccountFromString = (account: string) => {
  const decodedAccount = decodeIcrcAccount(account);
  const owner = decodedAccount.owner;
  const subaccount = decodedAccount?.subaccount
    ? [decodedAccount.subaccount]
    : [];
  return {
    owner,
    subaccount,
  };
};
