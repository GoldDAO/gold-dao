import { Principal } from "@dfinity/principal";

export const isValidPrincipal = (principal: string): boolean => {
  try {
    return principal === Principal.fromText(principal).toText();
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (e) {
    return false;
  }
};
