'use client'

import { useEffect, useState } from 'react'

// Treasury summary of the Gold DAO analytics API: the liquid tokens of the SNS
// treasury and the neurons the DAO controls, with their value in US dollars.
const TREASURY_URL = 'https://api.gldt.org/v1/daos/golddao/treasury/summary'
const TIMEOUT_MS = 10_000

interface Holding {
  token: string
  held: 'Staked' | 'Liquid'
  amount: number
  usd: number
}

type State =
  | { status: 'loading' }
  | { status: 'failed' }
  | { status: 'ready', total: number, holdings: Holding[] }

const isNumber = (value: unknown): value is number =>
  typeof value === 'number' && Number.isFinite(value)

// The DAO's own token is left out: the GOLDAO in the treasury is going to be
// burned, so it is not shown as a holding and not counted in the total.
const OWN_TOKEN = 'GOLDAO'

// Returns null when the response does not have the expected shape, so a change
// of the API shows the fallback instead of wrong or empty figures.
const readTreasury = (data: any): { total: number, holdings: Holding[] } | null => {
  const staked = data?.staked_tokens?.items
  const liquid = data?.liquid_tokens?.items
  if (!Array.isArray(staked) || !Array.isArray(liquid)) {
    return null
  }

  const holdings: Holding[] = [
    ...staked.map(item => ({
      token: item?.token,
      held: 'Staked' as const,
      amount: item?.total,
      usd: item?.total_usd
    })),
    ...liquid.map(item => ({
      token: item?.token,
      held: 'Liquid' as const,
      amount: item?.value,
      usd: item?.value_usd
    }))
  ]
    .filter(
      ({ token, amount, usd }) =>
        typeof token === 'string' &&
        token !== OWN_TOKEN &&
        isNumber(amount) &&
        isNumber(usd) &&
        amount > 0
    )
    .sort((a, b) => b.usd - a.usd)

  // The total is the sum of the rows shown, so the two always agree
  const total = holdings.reduce((sum, { usd }) => sum + usd, 0)

  return holdings.length > 0 ? { total, holdings } : null
}

const usdFormat = new Intl.NumberFormat('en-US', {
  style: 'currency',
  currency: 'USD',
  notation: 'compact',
  maximumSignificantDigits: 3
})
const compactFormat = new Intl.NumberFormat('en-US', {
  notation: 'compact',
  maximumFractionDigits: 2
})
const wholeFormat = new Intl.NumberFormat('en-US', { maximumFractionDigits: 0 })

const formatAmount = (amount: number) =>
  amount >= 1_000_000 ? compactFormat.format(amount) : wholeFormat.format(amount)

const Placeholder = ({ className }: { className: string }) => (
  <span
    className={`inline-block h-4 rounded bg-line align-middle motion-safe:animate-pulse ${className}`}
  />
)

export default function Treasury () {
  const [state, setState] = useState<State>({ status: 'loading' })

  useEffect(() => {
    let cancelled = false
    const controller = new AbortController()
    const timeout = setTimeout(() => controller.abort(), TIMEOUT_MS)

    fetch(TREASURY_URL, { signal: controller.signal })
      .then(response => {
        if (!response.ok) throw new Error(`Treasury request failed: ${response.status}`)
        return response.json()
      })
      .then(readTreasury)
      .catch(() => null)
      .then(treasury => {
        if (cancelled) return
        setState(treasury ? { status: 'ready', ...treasury } : { status: 'failed' })
      })
      .finally(() => clearTimeout(timeout))

    return () => {
      cancelled = true
      clearTimeout(timeout)
      controller.abort()
    }
  }, [])

  return (
    <section
      id='treasury'
      aria-labelledby='treasury-title'
      className='mx-auto max-w-6xl px-5 pt-20 sm:px-8 md:pt-28'
    >
      <div className='flex flex-wrap items-baseline justify-between gap-x-8 gap-y-2'>
        <h2 id='treasury-title' className='text-3xl font-semibold md:text-4xl'>
          The treasury
        </h2>
        {state.status !== 'failed' && (
          <p className='font-display text-4xl font-light tabular-nums md:text-5xl'>
            {state.status === 'ready' ? (
              usdFormat.format(state.total)
            ) : (
              <Placeholder className='h-8 w-36' />
            )}
          </p>
        )}
      </div>

      {state.status === 'failed' ? (
        <div className='mt-8 border-t border-content pt-6'>
          <p className='leading-relaxed text-content/70 md:text-lg md:leading-relaxed'>
            The live figures could not be loaded. Reload the page to try again.
          </p>
        </div>
      ) : (
        <>
          <table
            aria-busy={state.status === 'loading'}
            className='mt-8 w-full border-t border-content text-left tabular-nums md:text-lg'
          >
            <caption className='sr-only'>What the Gold DAO treasury holds</caption>
            <thead>
              <tr className='border-b border-line text-sm text-content/60'>
                <th scope='col' className='py-3 pr-4 font-normal'>Asset</th>
                <th scope='col' className='py-3 pr-4 font-normal'>Held</th>
                <th scope='col' className='py-3 pr-4 text-right font-normal'>Amount</th>
                <th scope='col' className='py-3 text-right font-normal'>Value</th>
              </tr>
            </thead>
            <tbody>
              {state.status === 'ready'
                ? state.holdings.map(({ token, held, amount, usd }) => (
                  <tr key={`${token}-${held}`} className='border-b border-line'>
                    <th scope='row' className='py-4 pr-4 font-semibold'>{token}</th>
                    <td className='py-4 pr-4 text-content/70'>{held}</td>
                    <td className='py-4 pr-4 text-right'>{formatAmount(amount)}</td>
                    <td className='py-4 text-right'>{usdFormat.format(usd)}</td>
                  </tr>
                ))
                : ['w-10', 'w-12', 'w-11'].map(width => (
                  <tr key={width} className='border-b border-line'>
                    <td className='py-4 pr-4'><Placeholder className={width} /></td>
                    <td className='py-4 pr-4'><Placeholder className='w-14' /></td>
                    <td className='py-4 pr-4 text-right'><Placeholder className='w-20' /></td>
                    <td className='py-4 text-right'><Placeholder className='w-16' /></td>
                  </tr>
                ))}
            </tbody>
          </table>
          <p className='mt-4 text-sm text-content/60'>
            Live figures from the DAO&apos;s accounts and neurons, valued in US
            dollars.
          </p>
        </>
      )}
    </section>
  )
}
