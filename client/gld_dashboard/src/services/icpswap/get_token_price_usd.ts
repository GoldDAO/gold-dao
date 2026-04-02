import { ActorSubclass } from "@dfinity/agent";

export interface PublicTokenOverview {
  id: bigint;
  volumeUSD1d: number;
  volumeUSD7d: number;
  totalVolumeUSD: number;
  name: string;
  volumeUSD: number;
  feesUSD: number;
  priceUSDChange: number;
  address: string;
  txCount: bigint;
  priceUSD: number;
  standard: string;
  symbol: string;
}

export const fetch_all_tokens = async (
  actor: ActorSubclass
): Promise<PublicTokenOverview[]> => {
  return (await actor.getAllTokens()) as PublicTokenOverview[];
};

export const find_token_price_usd = (
  tokens: PublicTokenOverview[],
  tokenCanisterId: string,
  tokenSymbol?: string
): number => {
  const token =
    tokens.find((t) => t.address === tokenCanisterId) ??
    (tokenSymbol
      ? tokens.find((t) => t.symbol === tokenSymbol)
      : undefined);
  if (!token) {
    throw new Error(
      `Token ${tokenSymbol ?? tokenCanisterId} not found on ICPSwap`
    );
  }
  return token.priceUSD;
};

const get_token_price_usd = async (
  actor: ActorSubclass,
  tokenCanisterId: string,
  tokenSymbol?: string
): Promise<number> => {
  const tokens = await fetch_all_tokens(actor);
  return find_token_price_usd(tokens, tokenCanisterId, tokenSymbol);
};

export default get_token_price_usd;
