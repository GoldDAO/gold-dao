import { Actor, ActorSubclass, Agent, HttpAgent } from "@dfinity/agent";
import {
  CKUSDT_LEDGER_CANISTER_ID,
  CKUSDT_LEDGER_CANISTER_ID_IC,
  GLDT_LEDGER_CANISTER_ID_IC,
  ICP_LEDGER_CANISTER_ID_IC,
  OGY_LEDGER_CANISTER_ID_IC,
  GOLDAO_LEDGER_CANISTER_ID_IC,
  WTN_LEDGER_CANISTER_ID_IC,
  ICPSWAP_GLDT_CKUSDT_POOL_CANISTER_ID_IC,
  ICPSWAP_ICP_CKUSDC_POOL_CANISTER_ID_IC,
  ICPSWAP_OGY_ICP_POOL_CANISTER_ID_IC,
  ICPSWAP_GOLDAO_ICP_POOL_CANISTER_ID_IC,
  ICPSWAP_WTN_ICP_POOL_CANISTER_ID_IC,
  CK_USDC_LEDGER_CANISTER_ID_IC,
} from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import icrc1_decimals from "@services/ledger/icrc1_decimals";
import { isBuiltinLedger } from "@shared/utils/tokens";
import get_icp_pool_price from "./get_icp_pool_price";

import { idlFactory as idlFactorySwapFactory } from "./idls/swap_factory";
import { Result } from "./interfaces/swap_factory";

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

interface PoolLeg {
  poolCanisterId: string;
  zeroForOne: boolean;
}

interface PriceRoute {
  legs: PoolLeg[];
  stablecoinCanisterId: string;
}

const PRICE_ROUTES: Record<string, PriceRoute> = {
  [GLDT_LEDGER_CANISTER_ID_IC]: {
    legs: [
      { poolCanisterId: ICPSWAP_GLDT_CKUSDT_POOL_CANISTER_ID_IC, zeroForOne: true },
    ],
    stablecoinCanisterId: CKUSDT_LEDGER_CANISTER_ID_IC,
  },
  [ICP_LEDGER_CANISTER_ID_IC]: {
    legs: [
      { poolCanisterId: ICPSWAP_ICP_CKUSDC_POOL_CANISTER_ID_IC, zeroForOne: true },
    ],
    stablecoinCanisterId: CK_USDC_LEDGER_CANISTER_ID_IC,
  },
  [OGY_LEDGER_CANISTER_ID_IC]: {
    legs: [
      { poolCanisterId: ICPSWAP_OGY_ICP_POOL_CANISTER_ID_IC, zeroForOne: true },
      { poolCanisterId: ICPSWAP_ICP_CKUSDC_POOL_CANISTER_ID_IC, zeroForOne: true },
    ],
    stablecoinCanisterId: CK_USDC_LEDGER_CANISTER_ID_IC,
  },
  [GOLDAO_LEDGER_CANISTER_ID_IC]: {
    legs: [
      // token0=ICP, token1=GOLDAO, so GOLDAO to ICP is zeroForOne=false
      { poolCanisterId: ICPSWAP_GOLDAO_ICP_POOL_CANISTER_ID_IC, zeroForOne: false },
      { poolCanisterId: ICPSWAP_ICP_CKUSDC_POOL_CANISTER_ID_IC, zeroForOne: true },
    ],
    stablecoinCanisterId: CK_USDC_LEDGER_CANISTER_ID_IC,
  },
  [WTN_LEDGER_CANISTER_ID_IC]: {
    legs: [
      { poolCanisterId: ICPSWAP_WTN_ICP_POOL_CANISTER_ID_IC, zeroForOne: true },
      { poolCanisterId: ICPSWAP_ICP_CKUSDC_POOL_CANISTER_ID_IC, zeroForOne: true },
    ],
    stablecoinCanisterId: CK_USDC_LEDGER_CANISTER_ID_IC,
  },
};

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
    (tokenSymbol ? tokens.find((t) => t.symbol === tokenSymbol) : undefined);
  if (!token) {
    throw new Error(
      `Token ${tokenSymbol ?? tokenCanisterId} not found on ICPSwap`
    );
  }
  return token.priceUSD;
};

const quote_swap_pool = async (
  agent: Agent | HttpAgent,
  swapPoolCanisterId: string,
  options: { amountIn: string; zeroForOne: boolean; amountOutMinimum: string }
): Promise<bigint> => {
  const actor = Actor.createActor(idlFactorySwapFactory, {
    agent,
    canisterId: swapPoolCanisterId,
  });

  const result = (await actor.quote(options)) as Result;

  if ("err" in result) {
    throw new Error(JSON.stringify(result.err));
  }

  return result.ok;
};

const get_live_quote_price_usd = async (
  agent: Agent | HttpAgent,
  tokenCanisterId: string
): Promise<number | null> => {
  if (
    tokenCanisterId === CKUSDT_LEDGER_CANISTER_ID ||
    tokenCanisterId === CKUSDT_LEDGER_CANISTER_ID_IC ||
    tokenCanisterId === CK_USDC_LEDGER_CANISTER_ID_IC
  ) {
    return 1;
  }

  const route = PRICE_ROUTES[tokenCanisterId];
  if (!route) {
    return null;
  }

  const tokenActor = Actor.createActor(idlFactoryLedger, {
    agent,
    canisterId: tokenCanisterId,
  });
  const stablecoinActor = Actor.createActor(idlFactoryLedger, {
    agent,
    canisterId: route.stablecoinCanisterId,
  });
  const [tokenDecimals, stablecoinDecimals] = await Promise.all([
    icrc1_decimals(tokenActor),
    icrc1_decimals(stablecoinActor),
  ]);

  let amount = (10n ** BigInt(tokenDecimals)).toString();
  for (const leg of route.legs) {
    const out = await quote_swap_pool(agent, leg.poolCanisterId, {
      amountIn: amount,
      amountOutMinimum: "0",
      zeroForOne: leg.zeroForOne,
    });
    amount = out.toString();
  }

  return Number(BigInt(amount)) / 10 ** stablecoinDecimals;
};

const PRICE_TTL_MS = 30 * 1000;
type CachedPrice = { at: number; price: Promise<number | null> };
const customPrices = new Map<string, CachedPrice>();

// One page asks for the same price many times (every transaction row does), so
// a price is fetched once and kept for a short time. A fetch that failed is
// forgotten, and the next call tries again.
const cachedPrice = (
  key: string,
  fetchPrice: () => Promise<number | null>
): Promise<number | null> => {
  const cached = customPrices.get(key);
  if (cached && Date.now() - cached.at < PRICE_TTL_MS) return cached.price;

  const price = fetchPrice();
  customPrices.set(key, { at: Date.now(), price });
  price.catch(() => {
    if (customPrices.get(key)?.price === price) customPrices.delete(key);
  });
  return price;
};

// Query calls fail now and then when a page sends many at once.
const withRetry = async <T>(
  run: () => Promise<T>,
  attempts = 3
): Promise<T> => {
  for (let attempt = 1; ; attempt++) {
    try {
      return await run();
    } catch (error) {
      if (attempt >= attempts) throw error;
      await new Promise((resolve) => setTimeout(resolve, 500 * attempt));
    }
  }
};

// USD price of a ledger the app does not ship with: its ICPSwap pool against
// ICP, times the ICP price. null when the token has no such pool. The ledger
// is identified by canister ID only; a symbol proves nothing, since any ledger
// can use any symbol.
const get_custom_token_price_usd = async (
  agent: Agent | HttpAgent,
  tokenCanisterId: string
): Promise<number | null> => {
  try {
    return await cachedPrice(tokenCanisterId, () =>
      withRetry(async () => {
        const [icpPerToken, icpPriceUSD] = await Promise.all([
          get_icp_pool_price(agent, tokenCanisterId),
          cachedPrice(ICP_LEDGER_CANISTER_ID_IC, () =>
            get_live_quote_price_usd(agent, ICP_LEDGER_CANISTER_ID_IC)
          ),
        ]);
        if (icpPerToken === null || icpPriceUSD === null) return null;
        return icpPerToken * icpPriceUSD;
      })
    );
  } catch (error) {
    // A price that cannot be read must not hide the balance.
    console.warn(error);
    return null;
  }
};

// Like get_token_price_usd, but null when there is no price. That only happens
// for a ledger the app does not ship with (a token the user added).
export const get_token_price_usd_or_null = async (
  actor: ActorSubclass,
  tokenCanisterId: string,
  tokenSymbol?: string,
  options: { agent?: Agent | HttpAgent } = {}
): Promise<number | null> => {
  const { agent } = options;

  if (agent) {
    try {
      const livePrice = await get_live_quote_price_usd(agent, tokenCanisterId);
      if (typeof livePrice === "number") {
        return livePrice;
      }
    } catch (error) {
      console.warn(error);
    }
  }

  if (!isBuiltinLedger(tokenCanisterId)) {
    return agent ? get_custom_token_price_usd(agent, tokenCanisterId) : null;
  }

  const tokens = await fetch_all_tokens(actor);
  return find_token_price_usd(tokens, tokenCanisterId, tokenSymbol);
};

const get_token_price_usd = async (
  actor: ActorSubclass,
  tokenCanisterId: string,
  tokenSymbol?: string,
  options: { agent?: Agent | HttpAgent } = {}
): Promise<number> => {
  const price = await get_token_price_usd_or_null(
    actor,
    tokenCanisterId,
    tokenSymbol,
    options
  );
  if (price === null) {
    throw new Error(
      `No price found for token ${tokenSymbol ?? tokenCanisterId}`
    );
  }
  return price;
};

export default get_token_price_usd;
