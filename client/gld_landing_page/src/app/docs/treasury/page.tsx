import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext, Table } from '@/components/docs/elements'
import { canisters, neurons, snsRoots } from '@/docs/canisters'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import {
  canisterUrl,
  icpAccountUrl,
  nnsNeuronUrl,
  proposalUrl,
  snsNeuronUrl
} from '@/links'

export const metadata = docsMetadata('treasury')

// Where the icp_neuron canister sends the voting rewards of the NNS neuron.
// These are the reward recipients set in the canister, with the account
// identifier each one has on the ICP ledger. The 1% goes to Cecil DAO; the
// canister labels that recipient "The Good DAO wallet", which is wrong.
const recipients = [
  {
    name: 'Staking rewards',
    detail: 'The pool of the rewards canister. Paid to GOLDAO neurons as ICP.',
    share: '33%',
    account: '6dc2515bbb9b0a97b8d977ebac3eba643a1fb4b6da8b33455e0dba957f0ce7da'
  },
  {
    name: 'GLDT for stakers',
    detail:
      'An account of the buyback canister. The ICP buys GLDT, which is paid to GOLDAO neurons.',
    share: '33%',
    account: '7cfd793d618d7000b8d845104396a714045438b67b8f213811f0c1ac37086eac'
  },
  {
    name: 'Buyback and burn',
    detail: 'The main account of the buyback canister.',
    share: '33%',
    account: '31836130dcff35502d04752ea5b82a24e44d41955f2a30bb8c2d284f4a318d82'
  },
  {
    name: 'Cecil DAO',
    detail: 'A donation to humanitarian causes.',
    share: '1%',
    account: '5db74d41cd1f4e41b83844607511a02fad2602bf0c3ad515f1e03b80a4548a67'
  }
]

export default function Page () {
  return (
    <DocsPage
      slug='treasury'
      lead='The treasury is three staked positions: an ICP neuron, an OGY neuron and a WTN neuron. Each is held by a canister of the Gold DAO SNS.'
    >
      <p>
        The amounts on this page are from November 2026. The{' '}
        <Link href='/#treasury'>treasury table on the home page</Link> shows
        the current ones.
      </p>

      <h2>ICP neuron</h2>
      <Table labels>
        <tbody>
          <tr>
            <th scope='row'>Neuron</th>
            <td>
              <Ext href={nnsNeuronUrl(neurons.icp)}>
                <code>{neurons.icp}</code>
              </Ext>{' '}
              in the NNS
            </td>
          </tr>
          <tr>
            <th scope='row'>Staked</th>
            <td>About 556,000 ICP</td>
          </tr>
          <tr>
            <th scope='row'>Dissolve delay</th>
            <td>2 years, not dissolving</td>
          </tr>
          <tr>
            <th scope='row'>Held by</th>
            <td>
              The icp_neuron canister{' '}
              <Ext href={canisterUrl(canisters.icpNeuron)}>
                <code>{canisters.icpNeuron}</code>
              </Ext>
            </td>
          </tr>
        </tbody>
      </Table>
      <p>
        The DAO used to hold its ICP in seven neurons. They were merged into
        this one in August 2026.
      </p>
      <p>
        The neuron votes on NNS proposals and earns maturity for it. A daily
        job of the canister spawns the maturity once it passes 780 ICP. Seven
        days later the NNS releases the ICP, and the canister sends it on in
        these shares:
      </p>
      <Table>
        <thead>
          <tr>
            <th scope='col'>Recipient</th>
            <th scope='col'>Share</th>
            <th scope='col'>ICP account</th>
          </tr>
        </thead>
        <tbody>
          {recipients.map(({ name, detail, share, account }) => (
            <tr key={account}>
              <th scope='row'>
                {name}
                <span className='block font-normal text-content/70'>
                  {detail}
                </span>
              </th>
              <td>{share}</td>
              <td>
                <Ext href={icpAccountUrl(account)}>
                  <code>{account}</code>
                </Ext>
              </td>
            </tr>
          ))}
        </tbody>
      </Table>
      <p>
        One exception: when the account that pays for the cycles of the DAO
        canisters holds less than 1,000 ICP, the next payout goes there
        instead.
      </p>

      <h2>OGY neuron</h2>
      <Table labels>
        <tbody>
          <tr>
            <th scope='row'>Neuron</th>
            <td>
              <Ext href={snsNeuronUrl(snsRoots.origyn, neurons.ogy)}>
                <code>{neurons.ogy}</code>
              </Ext>{' '}
              in the ORIGYN SNS
            </td>
          </tr>
          <tr>
            <th scope='row'>Staked</th>
            <td>About 508 million OGY</td>
          </tr>
          <tr>
            <th scope='row'>Dissolve delay</th>
            <td>5 years, not dissolving</td>
          </tr>
          <tr>
            <th scope='row'>Held by</th>
            <td>
              The sns_neuron_controller canister{' '}
              <Ext href={canisterUrl(canisters.snsNeuronController)}>
                <code>{canisters.snsNeuronController}</code>
              </Ext>
            </td>
          </tr>
        </tbody>
      </Table>
      <p>
        The neuron started as a donation of 500 million OGY from the ORIGYN
        Foundation. ORIGYN pays its stakers through a rewards canister of its
        own. Once a day the controller checks what it can claim there. From
        100,000 OGY on it claims and sends the OGY to the Gold DAO rewards
        pool, so all of it goes to GOLDAO neurons.
      </p>
      <p>
        The stake grows over time. The maturity of the neuron is staked
        automatically, and the buyback canister adds OGY when the price
        conditions on the{' '}
        <Link href={docsPath('buyback-and-burn')}>Buyback and burn</Link> page
        are met.
      </p>

      <h2>WTN neuron</h2>
      <Table labels>
        <tbody>
          <tr>
            <th scope='row'>Neuron</th>
            <td>
              <Ext href={snsNeuronUrl(snsRoots.waterNeuron, neurons.wtn)}>
                <code>{neurons.wtn}</code>
              </Ext>{' '}
              in the WaterNeuron SNS
            </td>
          </tr>
          <tr>
            <th scope='row'>Staked</th>
            <td>About 1.37 million WTN</td>
          </tr>
          <tr>
            <th scope='row'>Dissolve delay</th>
            <td>3 years, not dissolving</td>
          </tr>
          <tr>
            <th scope='row'>Held by</th>
            <td>
              The sns_neuron_controller canister{' '}
              <Ext href={canisterUrl(canisters.snsNeuronController)}>
                <code>{canisters.snsNeuronController}</code>
              </Ext>
            </td>
          </tr>
        </tbody>
      </Table>
      <p>
        The neuron earns WTN as maturity and receives ICP from WaterNeuron. The
        controller sends the WTN to the rewards pool once more than 10,000 WTN
        have built up, and the ICP once there are 100 ICP. All of it goes to
        GOLDAO neurons.
      </p>

      <h2>GOLDAO in the treasury</h2>
      <p>
        The SNS treasury also holds GOLDAO. It is not counted as an asset
        because <Ext href={proposalUrl(317)}>proposal 317</Ext> decided to burn
        it. That happens in batches, see{' '}
        <Link href={docsPath('buyback-and-burn')}>Buyback and burn</Link>.
      </p>
    </DocsPage>
  )
}
