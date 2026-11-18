import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { IDL } from "@dfinity/candid";
import { Principal } from "@dfinity/principal";
import {
  ICP_LEDGER_CANISTER_ID_IC,
  ICPSWAP_SWAP_FACTORY_CANISTER_ID_IC,
} from "@constants";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import icrc1_decimals from "@services/ledger/icrc1_decimals";

import { idlFactory as idlFactoryPool } from "./idls/swap_factory";
import { Result_6 as PoolMetadataResult } from "./interfaces/swap_factory";

const ICP_DECIMALS = 8;
// ICPSwap creates its pools in the 0.3% fee tier.
const POOL_FEE = 3000n;

// Only the method of the ICPSwap SwapFactory that looks a pool up.
const idlFactorySwapFactory: IDL.InterfaceFactory = ({ IDL }) => {
  const Token = IDL.Record({ address: IDL.Text, standard: IDL.Text });
  const GetPoolArgs = IDL.Record({
    fee: IDL.Nat,
    token0: Token,
    token1: Token,
  });
  const PoolData = IDL.Record({
    fee: IDL.Nat,
    key: IDL.Text,
    tickSpacing: IDL.Int,
    token0: Token,
    token1: Token,
    canisterId: IDL.Principal,
  });
  const Error = IDL.Variant({
    CommonError: IDL.Null,
    InternalError: IDL.Text,
    UnsupportedToken: IDL.Text,
    InsufficientFunds: IDL.Null,
  });
  return IDL.Service({
    getPool: IDL.Func(
      [GetPoolArgs],
      [IDL.Variant({ ok: PoolData, err: Error })],
      ["query"]
    ),
  });
};

type GetPoolResult = { ok: { canisterId: Principal } } | { err: unknown };

// The pool of a pair never changes, so a pool that was found is kept.
const poolCanisterIds = new Map<string, string>();

const get_icp_pool_canister_id = async (
  agent: Agent | HttpAgent,
  tokenCanisterId: string
): Promise<string | null> => {
  const known = poolCanisterIds.get(tokenCanisterId);
  if (known) return known;

  const factory = Actor.createActor(idlFactorySwapFactory, {
    agent,
    canisterId: ICPSWAP_SWAP_FACTORY_CANISTER_ID_IC,
  });
  // The factory sorts the pair itself and ignores the standards given here.
  const result = (await factory.getPool({
    fee: POOL_FEE,
    token0: { address: tokenCanisterId, standard: "ICRC1" },
    token1: { address: ICP_LEDGER_CANISTER_ID_IC, standard: "ICP" },
  })) as GetPoolResult;

  if ("err" in result) return null;

  const poolCanisterId = result.ok.canisterId.toText();
  poolCanisterIds.set(tokenCanisterId, poolCanisterId);
  return poolCanisterId;
};

// Price of one whole token in ICP: the spot price of the token's ICPSwap pool
// against ICP. null when there is no such pool, or the pool has no liquidity.
const get_icp_pool_price = async (
  agent: Agent | HttpAgent,
  tokenCanisterId: string
): Promise<number | null> => {
  const poolCanisterId = await get_icp_pool_canister_id(agent, tokenCanisterId);
  if (!poolCanisterId) return null;

  const pool = Actor.createActor(idlFactoryPool, {
    agent,
    canisterId: poolCanisterId,
  });
  const ledger = Actor.createActor(idlFactoryLedger, {
    agent,
    canisterId: tokenCanisterId,
  });
  const [metadata, tokenDecimals] = await Promise.all([
    pool.metadata() as Promise<PoolMetadataResult>,
    icrc1_decimals(ledger),
  ]);

  if ("err" in metadata) return null;
  const { sqrtPriceX96, liquidity, token0 } = metadata.ok;
  if (sqrtPriceX96 === 0n || liquidity === 0n) return null;

  // sqrtPriceX96 is the square root of (token1 per token0, in their smallest
  // units), scaled by 2^96.
  const token1PerToken0 = (Number(sqrtPriceX96) / 2 ** 96) ** 2;
  const icpPerToken =
    token0.address === tokenCanisterId ? token1PerToken0 : 1 / token1PerToken0;

  return icpPerToken * 10 ** (tokenDecimals - ICP_DECIMALS);
};

export default get_icp_pool_price;
