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
    try {
        const value = result?.[0]?.[0]?.[1];
        const defaultReturn = {
            name: "",
            description: "",
            image: "",
            attributes: [],
        }
        if (
            value &&
            "Array" in value &&
            Array.isArray(value.Array) &&
            value.Array.length > 0 &&
            "Text" in value.Array[0]
        ) {
            const response = await fetch(value.Array[0].Text);
            if (!response.ok)
                return defaultReturn;
            return await response.json();
        }
        return defaultReturn;
    } catch {
        throw new Error("Failed to parse metadata");
    }
};