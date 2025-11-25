import type { ICRC3Value } from "@services/nft_indexer/interfaces";

export type ICRC3ParsedValue =
  | number
  | string
  | Uint8Array
  | ICRC3ParsedValue[]
  | { [key: string]: ICRC3ParsedValue }
  | null;

const parseICRC3Value = (value: ICRC3Value): ICRC3ParsedValue => {
  if (!value) return null;
  if ("Int" in value) return Number(value.Int);
  if ("Nat" in value) return Number(value.Nat);
  if ("Text" in value) return value.Text;
  if ("Blob" in value) return value.Blob;
  if ("Map" in value) {
    const obj: { [key: string]: ICRC3ParsedValue } = {};
    value.Map.forEach(([k, v]) => {
      obj[k] = parseICRC3Value(v);
    });
    return obj;
  }
  if ("Array" in value) return value.Array.map(parseICRC3Value);
  return null;
};

export default parseICRC3Value;
