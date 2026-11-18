import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext } from '@/components/docs/elements'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import { links, proposalUrl } from '@/links'

export const metadata = docsMetadata('faq')

export default function Page () {
  return (
    <DocsPage
      slug='faq'
      heading='Frequently asked questions'
      lead='Short answers. Each one links to the page with the details.'
    >
      <h2>Gold DAO</h2>

      <h3>What is Gold DAO?</h3>
      <p>
        A DAO on the Internet Computer that holds a treasury of staked ICP, OGY
        and WTN. People who stake GOLDAO vote on what the DAO does and receive
        a share of what the treasury earns. The{' '}
        <Link href={docsPath('treasury')}>Treasury</Link> page lists what it
        holds.
      </p>

      <h3>Who decides what happens?</h3>
      <p>
        Gold DAO is an SNS. Upgrades of its canisters and transfers from its
        treasury need a proposal that GOLDAO neurons vote on. All proposals are
        public in the <Ext href={links.proposals}>NNS dapp</Ext>.
      </p>

      <h2>Staking and rewards</h2>

      <h3>Where do I stake and vote?</h3>
      <p>
        In the NNS dapp. The Gold DAO app does not create neurons; it lists the
        neurons you linked to it and lets you claim their rewards. See{' '}
        <Link href={docsPath('stake-goldao')}>Stake GOLDAO</Link>.
      </p>

      <h3>Which tokens are rewards paid in?</h3>
      <p>
        ICP, OGY and WTN on Wednesdays, GLDT on the first Wednesday of the
        month. GOLDAO is not paid as a reward since December 2025. See{' '}
        <Link href={docsPath('rewards')}>Rewards</Link>.
      </p>

      <h3>How much does a neuron earn?</h3>
      <p>
        There is no fixed rate. The amount depends on what the treasury earns
        and on how many other neurons share it, and its value depends on the
        prices of the four tokens.
      </p>

      <h3>Why does my neuron get no rewards?</h3>
      <p>
        Most often because its dissolve delay is below two years or because it
        is dissolving. Only neurons with a two-year delay that are not
        dissolving receive rewards, and only if they vote. The checklist is at
        the end of{' '}
        <Link href={docsPath('claim-rewards')}>Claim rewards</Link>.
      </p>

      <h3>
        Why is my principal in the Gold DAO app different from the one in the
        NNS dapp?
      </h3>
      <p>
        Internet Identity gives you a separate principal for every app. That is
        why neurons are linked to the Gold DAO app with a hotkey. See{' '}
        <Link href={docsPath('claim-rewards')}>Claim rewards</Link>.
      </p>

      <h3>How do I get my GOLDAO back?</h3>
      <p>
        Start dissolving the neuron in the NNS dapp. When the dissolve delay
        has run down to zero you can disburse the GOLDAO. A neuron stops
        receiving rewards when it starts dissolving.
      </p>

      <h2>GLDT</h2>

      <h3>What happened to GLDT?</h3>
      <p>
        GLDT, the gold token that Gold DAO launched, is run by the ORIGYN DAO
        since autumn 2026. The move was decided in{' '}
        <Ext href={proposalUrl(317)}>proposal 317</Ext> and carried out with
        proposals <Ext href={proposalUrl(403)}>403</Ext> and{' '}
        <Ext href={proposalUrl(406)}>406</Ext>. GLDT now lives at{' '}
        <Ext href={links.gldt}>gldt.org</Ext>.
      </p>
      <p>
        Gold DAO still buys GLDT with a third of its ICP rewards and pays it to
        GOLDAO neurons once a month.
      </p>

      <h3>I still have GLDT staked in the Gold DAO app</h3>
      <p>
        GLDT staking has been discontinued. You can claim the outstanding
        rewards and withdraw your GLDT on the{' '}
        <Ext href={links.appEarn}>Earn page</Ext> of the app.
      </p>
    </DocsPage>
  )
}
