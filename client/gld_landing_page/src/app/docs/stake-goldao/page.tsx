import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext, Figure, Note, Table } from '@/components/docs/elements'
import { neurons, snsRoots } from '@/docs/canisters'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import { links, snsNeuronUrl } from '@/links'

export const metadata = docsMetadata('stake-goldao')

export default function Page () {
  return (
    <DocsPage
      slug='stake-goldao'
      lead='Staking means locking GOLDAO in a neuron. The neuron votes on Gold DAO proposals and, with a two-year lock, receives rewards. Staking is done in the NNS dapp, not in the Gold DAO app.'
    >
      <h2>Before you start</h2>
      <Table labels>
        <tbody>
          <tr>
            <th scope='row'>Minimum stake</th>
            <td>100 GOLDAO</td>
          </tr>
          <tr>
            <th scope='row'>Transfer fee</th>
            <td>10 GOLDAO per transfer</td>
          </tr>
          <tr>
            <th scope='row'>Dissolve delay to vote</th>
            <td>At least 3 months (91.3 days)</td>
          </tr>
          <tr>
            <th scope='row'>Dissolve delay for rewards</th>
            <td>2 years, which is the maximum, and the neuron is not dissolving</td>
          </tr>
        </tbody>
      </Table>
      <p>
        The dissolve delay is the time your GOLDAO stays locked after you start
        dissolving the neuron. With a delay of two years you get the GOLDAO
        back two years after you start dissolving, and the neuron stops
        receiving rewards from the day you do.
      </p>

      <h2>1. Get GOLDAO</h2>
      <p>
        GOLDAO trades against ICP on{' '}
        <Ext href={links.icpSwap}>ICPSwap</Ext>. Buy a little more than you
        want to stake. Each transfer costs 10 GOLDAO, and it takes two
        transfers to get GOLDAO from the exchange into a neuron.
      </p>

      <h2>2. Send it to the NNS dapp</h2>
      <ol>
        <li>
          Open the <Ext href={links.nnsDapp}>NNS dapp</Ext> and sign in with
          your Internet Identity.
        </li>
        <li>
          Go to <strong>Tokens</strong> and select <strong>Gold DAO</strong>.
          Copy the principal at the top right.
          <Figure
            src='/docs/nns-tokens-principal.png'
            alt='The Gold DAO account in the NNS dapp. An arrow points at the principal in the top right corner.'
            width={1177}
            height={542}
          />
        </li>
        <li>
          Withdraw your GOLDAO from the exchange to that principal. The balance
          shows up after a few seconds.
        </li>
      </ol>

      <h2>3. Create the neuron</h2>
      <ol>
        <li>
          Go to <strong>Neuron Staking</strong>, select{' '}
          <strong>Gold DAO</strong> and click <strong>Stake GOLDAO</strong>.
          <Figure
            src='/docs/nns-stake-goldao.png'
            alt='The Neuron Staking page of the NNS dapp for Gold DAO with the Stake GOLDAO button.'
            width={1188}
            height={546}
          />
        </li>
        <li>Enter the amount and confirm.</li>
        <li>
          Open the new neuron.
          <Figure
            src='/docs/nns-neuron-settings.png'
            alt='A Gold DAO neuron in the NNS dapp. The Set Dissolve Delay button is marked 1, the Follow Neurons button is marked 2.'
            width={774}
            height={760}
            maxWidth={520}
          />
        </li>
        <li>
          Click <strong>Set Dissolve Delay</strong> (1 in the picture). Three
          months or more lets the neuron vote. Set two years if you want
          rewards.
        </li>
        <li>
          Click <strong>Follow Neurons</strong> (2 in the picture) unless you
          plan to vote on every proposal yourself. Pick a neuron to follow for
          each topic. A topic you leave out is a topic your neuron does not
          vote on.
          <Figure
            src='/docs/nns-delegate-voting.png'
            alt='The Delegate Voting dialog of the NNS dapp with three critical and four non-critical topics.'
            width={840}
            height={1103}
            maxWidth={420}
          />
        </li>
      </ol>
      <Note>
        <p>
          A neuron only earns rewards for the proposals it votes on, either
          itself or through the neuron it follows.
        </p>
      </Note>
      <p>
        If you want to follow the main contributing team, its neuron is{' '}
        <Ext href={snsNeuronUrl(snsRoots.goldDao, neurons.team)}>
          <code>{neurons.team}</code>
        </Ext>
        . You can follow any other neuron you trust instead.
      </p>

      <h2>Next</h2>
      <p>
        Rewards are claimed in the Gold DAO app. For that the neuron has to be
        linked to the app once, see{' '}
        <Link href={docsPath('claim-rewards')}>Claim rewards</Link>. Which
        neurons receive rewards and when is on the{' '}
        <Link href={docsPath('rewards')}>Rewards</Link> page.
      </p>
    </DocsPage>
  )
}
