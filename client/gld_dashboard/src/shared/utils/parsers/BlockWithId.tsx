import type { BlockWithId } from "@services/swap_indexer/interfaces";
import { getDateUTC } from "@shared/utils/dates";
import parseICRC3Value, {
  ICRC3ParsedValue,
} from "@shared/utils/parsers/ICRC3Value";
import _capitalize from "lodash/capitalize";

const capitalizeType = (type: string | null) => {
  if (!type) return null;
  const map: Record<string, string> = {
    "7xfer": "Transfer",
    "7mint": "Mint",
  };
  const lower = type.toLowerCase();
  return map[lower] || _capitalize(type);
};

export const parseBlockWithId = (blockWithId: BlockWithId) => {
  const { id, block } = blockWithId;
  let created_at: string | null = null,
    from: string | null = null,
    to: string | null = null,
    type: string | null = null,
    nft_id: bigint = 0n;
  if (block && "Map" in block) {
    const map = parseICRC3Value(block);
    if (typeof map === "object" && map !== null && "tx" in map) {
      const tx = (map as { [key: string]: ICRC3ParsedValue }).tx;
      if (
        tx &&
        typeof tx === "object" &&
        !Array.isArray(tx) &&
        !(tx instanceof Uint8Array)
      ) {
        const txObj = tx as { [key: string]: ICRC3ParsedValue };
        created_at =
          txObj.created_at_time != null
            ? getDateUTC(txObj.created_at_time.toString(), { fromNanos: true })
            : "-";
        from = (txObj.from as string | null) ?? null;
        to = (txObj.to as string | null) ?? null;
        nft_id =
          typeof txObj.tid === "bigint"
            ? txObj.tid
            : typeof txObj.tid === "number"
            ? BigInt(txObj.tid)
            : 0n;
        type = capitalizeType((txObj.op as string | null) ?? null);
      }
    }
  }
  return { tx_id: id, nft_id, created_at, from, to, type };
};

export default parseBlockWithId;
