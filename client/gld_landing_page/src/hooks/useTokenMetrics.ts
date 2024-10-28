import { useQuery } from '@tanstack/react-query'
import { Actor, HttpAgent } from '@dfinity/agent'
import { idlFactory as icrc7IdlFactory } from '../data/canisters/gold/did'
import { idlFactory as icrc1IdlFactory } from '../data/canisters/ledger/did'
import { Principal } from '@dfinity/principal'
import { Account } from '../data/canisters/gold/interfaces/gld_nft'

const goldTokenCanisters = {
  '1g': {
    canisterId: process.env.NEXT_PUBLIC_CANISTER_1G!,
    weightInGrams: BigInt(1)
  },
  '10g': {
    canisterId: process.env.NEXT_PUBLIC_CANISTER_10G!,
    weightInGrams: BigInt(10)
  },
  '100g': {
    canisterId: process.env.NEXT_PUBLIC_CANISTER_100G!,
    weightInGrams: BigInt(100)
  },
  '1kg': {
    canisterId: process.env.NEXT_PUBLIC_CANISTER_1KG!,
    weightInGrams: BigInt(1000)
  }
}

const swapCanisterId = process.env.NEXT_PUBLIC_SWAP_CANISTER_ID!
const gldtLedgerCanisterId = process.env.NEXT_PUBLIC_GLDT_LEDGER_CANISTER_ID!

export interface TokenMetrics {
  gold_price: number
  total_gold_grams: string
  total_gold_kg: number
  tvl: number
}

export const fetchTokenMetrics = async (): Promise<TokenMetrics> => {
  try {
    const agent = new HttpAgent({ host: 'https://ic0.app' })
    const swapCanisterPrincipal = Principal.fromText(swapCanisterId)

    const response = await fetch(
      'https://teiwz-pqaaa-aaaap-ag7hq-cai.raw.icp0.io/gold_nft_metrics'
    )
    if (!response.ok) {
      throw new Error('Failed to fetch gold price')
    }
    const data = await response.json()
    const gold_price = parseFloat(data.gold_price)

    let total_gold_grams = BigInt(0)

    for (const { canisterId, weightInGrams } of Object.values(
      goldTokenCanisters
    )) {
      const actor = Actor.createActor(icrc7IdlFactory, {
        agent,
        canisterId
      })

      const account: Account = {
        account: {
          owner: swapCanisterPrincipal,
          sub_account: []
        }
      }

      const accounts: Account[] = [account]

      const balances = (await actor.icrc7_balance_of(accounts)) as bigint[]

      for (const balance of balances) {
        total_gold_grams += balance * weightInGrams
      }
    }

    const total_gold_kg = Number(total_gold_grams) / 1000

    const ledgerActor = Actor.createActor(icrc1IdlFactory, {
      agent,
      canisterId: gldtLedgerCanisterId
    })

    const totalSupply = (await ledgerActor.icrc1_total_supply()) as bigint
    const totalSupplyTokens = Number(totalSupply) / 1e8

    const tvl = totalSupplyTokens * gold_price

    return {
      gold_price,
      total_gold_grams: total_gold_grams.toString(),
      total_gold_kg,
      tvl
    }
  } catch (error) {
    console.error('Error fetching token metrics:', error)
    throw error
  }
}

export const useTokenMetrics = () => {
  return useQuery<TokenMetrics>({
    queryKey: ['tokenMetrics'],
    queryFn: fetchTokenMetrics,
    staleTime: 5 * 60 * 1000,
    refetchOnWindowFocus: false
  })
}
