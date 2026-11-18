import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext, Table } from '@/components/docs/elements'
import { canisters } from '@/docs/canisters'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import { canisterUrl, links } from '@/links'

export const metadata = docsMetadata('goldao-token')

export default function Page () {
  return (
    <DocsPage
      slug='goldao-token'
      lead='GOLDAO is the governance token of Gold DAO. Staked in a neuron it gives voting power and, under the conditions on the Rewards page, a share of what the treasury earns.'
    >
      <Table labels>
        <tbody>
          <tr>
            <th scope='row'>Symbol</th>
            <td>GOLDAO</td>
          </tr>
          <tr>
            <th scope='row'>Ledger canister</th>
            <td>
              <Ext href={canisterUrl(canisters.snsLedger)}>
                <code>{canisters.snsLedger}</code>
              </Ext>
            </td>
          </tr>
          <tr>
            <th scope='row'>Index canister</th>
            <td>
              <Ext href={canisterUrl(canisters.snsIndex)}>
                <code>{canisters.snsIndex}</code>
              </Ext>
            </td>
          </tr>
          <tr>
            <th scope='row'>Standards</th>
            <td>ICRC-1, ICRC-2, ICRC-3</td>
          </tr>
          <tr>
            <th scope='row'>Decimals</th>
            <td>8</td>
          </tr>
          <tr>
            <th scope='row'>Transfer fee</th>
            <td>10 GOLDAO</td>
          </tr>
          <tr>
            <th scope='row'>Minimum stake</th>
            <td>100 GOLDAO</td>
          </tr>
        </tbody>
      </Table>
      <p>The token was called GLDGov until January 2025.</p>

      <h2>What it is for</h2>
      <p>
        GOLDAO is staked in neurons. Neurons vote on the proposals of the DAO:
        upgrades of its canisters, changes to its parameters and what happens
        with the treasury. Neurons that are locked for two years and vote
        receive rewards. See{' '}
        <Link href={docsPath('stake-goldao')}>Stake GOLDAO</Link> and{' '}
        <Link href={docsPath('rewards')}>Rewards</Link>.
      </p>

      <h2>Supply</h2>
      <p>
        No GOLDAO is issued for rewards. New GOLDAO only comes into existence
        when a neuron disburses its maturity, which the SNS grants to voting
        neurons at 0.1% of the supply per year. Against that stand the burns
        described on the{' '}
        <Link href={docsPath('buyback-and-burn')}>Buyback and burn</Link> page.
        The current supply is on the{' '}
        <Ext href={links.icpDashboard}>ICP dashboard</Ext>.
      </p>

      <h2>Where to trade</h2>
      <p>
        GOLDAO trades against ICP on{' '}
        <Ext href={links.icpSwap}>ICPSwap</Ext>. This is the pool the buyback
        canister of the DAO uses.
      </p>
    </DocsPage>
  )
}
