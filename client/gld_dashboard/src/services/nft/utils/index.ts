import type { ICRC3Value } from "../interfaces";
import type { MetadataNFT } from "./interfaces";

export const bigIntTo32ByteArray = (value: bigint) => {
  const byteArray = new Uint8Array(32);
  for (let i = byteArray.length - 1; i >= 0; i--) {
    byteArray[i] = Number(value & 0xffn);
    value >>= 8n;
  }
  return byteArray.reverse();
};

export const parseMetadata = async (
  result: [] | [Array<[string, ICRC3Value]>]
): Promise<MetadataNFT> => {
  const defaultReturn: MetadataNFT = {};

  try {
    const value = result?.[0]?.[0]?.[1];
    if (
      value &&
      "Array" in value &&
      Array.isArray(value.Array) &&
      value.Array.length > 0 &&
      "Text" in value.Array[0]
    ) {
      const response = await fetch(value.Array[0].Text);
      if (!response.ok) return defaultReturn;
      const rawResult = await response.json();

      let attributes = rawResult.attributes || [];
      const weightGrams = attributes.find(
        (a: { trait_type: string; value: string | number }) =>
          a.trait_type === "Weight" &&
          String(a.value).endsWith("g") &&
          !String(a.value).endsWith("kg")
      );

      const extracted: Partial<MetadataNFT> = {
        img_preview: rawResult.image || "",
      };

      if (weightGrams) {
        extracted.weight = weightGrams.value;
        extracted.value = parseFloat(weightGrams.value) || undefined;
      }

      attributes = attributes.filter(
        (a: { trait_type: string; value: string | number }) =>
          a.trait_type !== "Weight"
      );

      for (const attr of attributes) {
        const traitType = attr.trait_type;
        const attrValue = attr.value;

        switch (traitType) {
          case "Fineness":
            extracted.fineness = attrValue;
            break;
          case "Dimensions":
            extracted.dimensions = attrValue;
            break;
          case "Hardness":
            extracted.hardness = attrValue;
            break;
          case "Manufacturer":
            extracted.manufacturer = attrValue;
            break;
          case "Serial Number":
            extracted.serial_number = Number(attrValue);
            break;
          default:
            if (traitType.startsWith("front-")) {
              extracted.img_front = attrValue;
            } else if (traitType.startsWith("back-")) {
              extracted.img_back = attrValue;
            }
            break;
        }
      }

      const result: MetadataNFT = {
        name: rawResult.name,
        description: rawResult.description,
        ...extracted,
      };
      return result;
    }
    return defaultReturn;
  } catch {
    return defaultReturn;
  }
};
