import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext, Table } from '@/components/docs/elements'
import { canisters } from '@/docs/canisters'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import { canisterUrl, links, proposalUrl } from '@/links'

export const metadata = docsMetadata('rewards')

export default function Page () {
  return (
    <DocsPage
      slug='rewards'
      lead='GOLDAO neurons that are locked for two years and vote receive ICP, OGY, WTN and GLDT. The tokens come from what the treasury earns.'
    >
      <h2>Which neurons receive rewards</h2>
      <p>A neuron receives rewards when both of these are true:</p>
      <ul>
        <li>
          Its dissolve delay is at least 730 days and it is not dissolving. The
          longest delay a GOLDAO neuron can have is two years, so in practice
          the delay has to be set to the maximum.
        </li>
        <li>
          It votes, either itself or by following another neuron. A neuron that
          does not vote gains no maturity, and rewards follow maturity (see
          below).
        </li>
      </ul>
      <p>
        The two-year rule was decided with{' '}
        <Ext href={proposalUrl(337)}>proposal 337</Ext> in January 2026. A
        neuron that starts dissolving drops out at the next daily scan. What it
        was assigned before that stays claimable.
      </p>

      <h2>Where the rewards come from</h2>
      <Table>
        <thead>
          <tr>
            <th scope='col'>Token</th>
            <th scope='col'>Source</th>
            <th scope='col'>Goes to stakers</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope='row'>ICP</th>
            <td>Voting rewards of the NNS neuron</td>
            <td>33%</td>
          </tr>
          <tr>
            <th scope='row'>ICP</th>
            <td>ICP the WTN neuron receives from WaterNeuron</td>
            <td>All of it</td>
          </tr>
          <tr>
            <th scope='row'>OGY</th>
            <td>Rewards of the OGY neuron</td>
            <td>All of it</td>
          </tr>
          <tr>
            <th scope='row'>WTN</th>
            <td>Rewards of the WTN neuron</td>
            <td>All of it</td>
          </tr>
          <tr>
            <th scope='row'>GLDT</th>
            <td>
              Bought on ICPSwap with another 33% of the voting rewards of the
              NNS neuron
            </td>
            <td>All of it</td>
          </tr>
        </tbody>
      </Table>
      <p>
        The neurons and the rest of the split are described on the{' '}
        <Link href={docsPath('treasury')}>Treasury</Link> page. GLDT is the
        gold token that Gold DAO handed over to ORIGYN in 2026, see{' '}
        <Ext href={links.gldt}>gldt.org</Ext>.
      </p>

      <h2>How the share of a neuron is calculated</h2>
      <p>
        The SNS itself pays voting neurons a small reward in the form of
        maturity: 0.1% of the GOLDAO supply per year, split by voting power
        among the neurons that vote. The{' '}
        <Ext href={canisterUrl(canisters.snsRewards)}>rewards canister</Ext>{' '}
        uses that maturity as its measure.
      </p>
      <ul>
        <li>
          Once a day, at 09:00 UTC, it reads all neurons and records how much
          maturity each eligible neuron has gained.
        </li>
        <li>
          At a payout it splits the pool of each token in proportion to the
          maturity the neurons gained since the last payout of that token.
        </li>
        <li>
          The share of a neuron is moved to an account of the rewards canister
          that belongs to that neuron. It stays there until it is claimed.
        </li>
      </ul>
      <p>
        Voting power grows with the stake and with the age of the neuron. A
        neuron that has not been dissolving for two years has 50% more voting
        power than a new neuron of the same size.
      </p>
      <p>
        Disbursing or staking the maturity of a neuron in the NNS dapp does not
        reduce its share. The canister counts what a neuron gained, not what it
        still holds. Maturity gained while a neuron was not eligible does not
        count.
      </p>

      <h2>When rewards are paid</h2>
      <ul>
        <li>
          ICP, OGY and WTN: every Wednesday at 14:00 UTC, for each token that
          has a balance in the pool.
        </li>
        <li>GLDT: on the first Wednesday of the month at 12:00 UTC.</li>
      </ul>
      <p>
        The pool does not fill evenly, so a weekly payout can contain one, two
        or all three tokens. If a token is skipped, its next payout covers the
        weeks in between.
      </p>
      <Table>
        <thead>
          <tr>
            <th scope='col'>Token</th>
            <th scope='col'>Reaches the pool</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope='row'>ICP</th>
            <td>
              Seven days after the maturity of the NNS neuron passes 780 ICP,
              which at the current rate happens about once a week. ICP from the
              WTN neuron is forwarded once 100 ICP have come together.
            </td>
          </tr>
          <tr>
            <th scope='row'>OGY</th>
            <td>Once 100,000 OGY of rewards can be claimed from ORIGYN</td>
          </tr>
          <tr>
            <th scope='row'>WTN</th>
            <td>Once the WTN neuron has more than 10,000 WTN of maturity</td>
          </tr>
          <tr>
            <th scope='row'>GLDT</th>
            <td>Bought in small portions every six hours</td>
          </tr>
        </tbody>
      </Table>
      <p>
        After a payout the rewards show in the Gold DAO app, see{' '}
        <Link href={docsPath('claim-rewards')}>Claim rewards</Link>.
      </p>

      <h2>GOLDAO is no longer paid as a reward</h2>
      <p>
        Until December 2025 neurons also received GOLDAO from a reserve pool.{' '}
        <Ext href={proposalUrl(317)}>Proposal 317</Ext> ended that, and what
        was left in the pool was burned in January 2026.
      </p>
    </DocsPage>
  )
}
