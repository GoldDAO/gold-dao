import { Actor, ActorSubclass, Agent, HttpAgent } from "@dfinity/agent";
import {
  GLDT_LEDGER_CANISTER_ID_IC,
  ICP_LEDGER_CANISTER_ID_IC,
  OGY_LEDGER_CANISTER_ID_IC,
  GOLDAO_LEDGER_CANISTER_ID_IC,
  WTN_LEDGER_CANISTER_ID_IC,
  CKUSDT_LEDGER_CANISTER_ID_IC,
  CK_USDC_LEDGER_CANISTER_ID_IC,
  ICPSWAP_GLDT_CKUSDT_POOL_CANISTER_ID_IC,
  ICPSWAP_ICP_CKUSDC_POOL_CANISTER_ID_IC,
  ICPSWAP_OGY_ICP_POOL_CANISTER_ID_IC,
  ICPSWAP_GOLDAO_ICP_POOL_CANISTER_ID_IC,
  ICPSWAP_WTN_ICP_POOL_CANISTER_ID_IC,
} from "../../constants";
import { idlFactory as idlFactoryLedger } from "../ledger/idlFactory";
import icrc1_decimals from "../ledger/icrc1_decimals";

import { idlFactory as idlFactorySwapFactory } from "./swap_factory_idlFactory";

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
  tokenCanisterId?: string,
  tokenSymbol?: string
): number => {
  const token =
    (tokenCanisterId
      ? tokens.find((t) => t.address === tokenCanisterId)
      : undefined) ??
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

const quote_swap_pool = async (
  agent: Agent | HttpAgent,
  swapPoolCanisterId: string,
  options: { amountIn: string; zeroForOne: boolean; amountOutMinimum: string }
): Promise<bigint> => {
  const actor = Actor.createActor(idlFactorySwapFactory, {
    agent,
    canisterId: swapPoolCanisterId,
  });

  const result = (await actor.quote(options)) as { ok: bigint } | { err: unknown };

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

const get_token_price_usd = async (
  actor: ActorSubclass,
  tokenCanisterId?: string,
  tokenSymbol?: string,
  options: { agent?: Agent | HttpAgent } = {}
): Promise<number> => {
  const { agent } = options;

  if (agent && tokenCanisterId) {
    try {
      const livePrice = await get_live_quote_price_usd(agent, tokenCanisterId);
      if (typeof livePrice === "number") {
        return livePrice;
      }
    } catch (error) {
      console.warn(error);
    }
  }

  const tokens = await fetch_all_tokens(actor);
  return find_token_price_usd(tokens, tokenCanisterId, tokenSymbol);
};

export default get_token_price_usd;
