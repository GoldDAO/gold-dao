// import { Principal } from "@dfinity/principal";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";

export const isValidPrincipalOrICRCAccount = (principal: string): boolean => {
  try {
    decodeIcrcAccount(principal);
    return true;
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (e) {
    return false;
  }
};
