import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import { Buffer } from "buffer";

// Whole bytes in hexadecimal. Buffer.from(text, "hex") does not reject other
// text: it drops everything from the first character that is not hexadecimal,
// and a last digit without a pair. "0x01" would become the default account.
const SUBACCOUNT_HEX = /^([0-9a-fA-F]{2})*$/;

export const isValidPrincipalAndSubaccount = (
  principal: string,
  subaccount?: string
): boolean => {
  if (subaccount && !SUBACCOUNT_HEX.test(subaccount)) return false;
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
