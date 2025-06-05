import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import { Buffer } from "buffer";

export const isValidPrincipalAndSubaccount = (
  principal: string,
  subaccount?: string
): boolean => {
  try {
    const encoded = encodeIcrcAccount({
      owner: Principal.fromText(principal),
      subaccount: subaccount ? Buffer.from(subaccount, "hex") : [],
    });
    decodeIcrcAccount(encoded);
    return true;
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (e) {
    return false;
  }
};
