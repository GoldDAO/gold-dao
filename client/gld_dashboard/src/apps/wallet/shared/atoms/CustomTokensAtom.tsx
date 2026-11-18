import { atom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { TOKENS } from "@shared/utils/tokens";
import {
  CUSTOM_TOKENS_STORAGE_KEY,
  StoredCustomToken,
  parseStoredCustomTokens,
  toCustomToken,
} from "@shared/utils/tokens/custom";

// Raw localStorage value. getOnInit reads it before the first render, so a
// page opened on a custom token finds that token right away.
const StoredCustomTokensAtom = atomWithStorage<unknown>(
  CUSTOM_TOKENS_STORAGE_KEY,
  [],
  undefined,
  { getOnInit: true }
);

// The tokens the user added to the wallet, in the order they were added.
export const CustomTokensAtom = atom((get) =>
  parseStoredCustomTokens(get(StoredCustomTokensAtom), TOKENS).map(
    toCustomToken
  )
);

export const AddCustomTokenAtom = atom(
  null,
  (get, set, token: StoredCustomToken) => {
    const stored = parseStoredCustomTokens(get(StoredCustomTokensAtom), TOKENS);
    set(StoredCustomTokensAtom, [...stored, token]);
  }
);

export const RemoveCustomTokenAtom = atom(
  null,
  (get, set, ledgerId: string) => {
    const stored = parseStoredCustomTokens(get(StoredCustomTokensAtom), TOKENS);
    set(
      StoredCustomTokensAtom,
      stored.filter((token) => token.ledger_id !== ledgerId)
    );
  }
);

// Open state of the "Add token" dialog. The dialog is opened from the desktop
// list and from the mobile list dialog, and rendered once by the wallet page.
export const AddCustomTokenDialogAtom = atom(false);
