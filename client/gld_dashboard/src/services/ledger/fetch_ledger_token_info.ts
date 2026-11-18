import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryIndex } from "@services/ledger_indexer/idlFactory";

// What an ICRC-1 ledger says about its token. All of it is set by whoever
// controls the ledger, so none of it proves which token this is.
export interface LedgerTokenInfo {
  symbol: string;
  name: string;
  decimals: number;
  // The logo as a data URL, or null when the ledger has none.
  logo: string | null;
  // The index canister the ledger publishes (ICRC-106), or null.
  index_id: string | null;
}

type MetadataValue =
  | { Text: string }
  | { Nat: bigint }
  | { Int: bigint }
  | { Blob: Uint8Array | number[] };

const metadataText = (
  metadata: [string, MetadataValue][],
  key: string
): string | null => {
  const entry = metadata.find(([name]) => name === key);
  return entry && "Text" in entry[1] ? entry[1].Text : null;
};

// A logo is only shown when it is embedded as an image. A logo given as a link
// would make the browser fetch a file from a host the ledger picks.
const logoFromMetadata = (metadata: [string, MetadataValue][]) => {
  const logo = metadataText(metadata, "icrc1:logo");
  return logo && logo.startsWith("data:image/") ? logo : null;
};

const indexFromMetadata = (metadata: [string, MetadataValue][]) => {
  const index = metadataText(metadata, "icrc106:index_principal");
  if (!index) return null;
  try {
    return Principal.fromText(index).toText();
  } catch {
    return null;
  }
};

const createLedgerActor = (agent: Agent | HttpAgent, ledgerId: string) =>
  Actor.createActor(idlFactoryLedger, { agent, canisterId: ledgerId });

// Throws when the canister does not answer the ICRC-1 queries.
export const fetch_ledger_token_info = async (
  agent: Agent | HttpAgent,
  ledgerId: string
): Promise<LedgerTokenInfo> => {
  const ledger = createLedgerActor(agent, ledgerId);
  const [symbol, name, decimals, metadata] = await Promise.all([
    ledger.icrc1_symbol() as Promise<string>,
    ledger.icrc1_name() as Promise<string>,
    ledger.icrc1_decimals() as Promise<number>,
    ledger.icrc1_metadata() as Promise<[string, MetadataValue][]>,
  ]);

  return {
    symbol,
    name,
    decimals,
    logo: logoFromMetadata(metadata),
    index_id: indexFromMetadata(metadata),
  };
};

export const fetch_ledger_logo = async (
  agent: Agent | HttpAgent,
  ledgerId: string
): Promise<string | null> => {
  const ledger = createLedgerActor(agent, ledgerId);
  const metadata = (await ledger.icrc1_metadata()) as [
    string,
    MetadataValue,
  ][];
  return logoFromMetadata(metadata);
};

// True when the canister is an index canister that follows this ledger.
export const is_index_of_ledger = async (
  agent: Agent | HttpAgent,
  indexId: string,
  ledgerId: string
): Promise<boolean> => {
  try {
    const index = Actor.createActor(idlFactoryIndex, {
      agent,
      canisterId: indexId,
    });
    const indexedLedger = (await index.ledger_id()) as Principal;
    return indexedLedger.toText() === ledgerId;
  } catch {
    return false;
  }
};
