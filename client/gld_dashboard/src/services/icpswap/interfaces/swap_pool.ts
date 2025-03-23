export interface GetPoolsForTokenResponse  {
    fee: number,
    token0Id: string
    token1Id: string
    pool: string
    token1Price: number
    token1Standard: string
    token1Decimals: number
    token0Standard: string
    token0Symbol: string
    token0Decimals: number
    token0Price: number
    token1Symbol: string
}