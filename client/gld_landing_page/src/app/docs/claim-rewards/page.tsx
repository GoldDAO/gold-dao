import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext, Figure } from '@/components/docs/elements'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import { links } from '@/links'

export const metadata = docsMetadata('claim-rewards')

export default function Page () {
  return (
    <DocsPage
      slug='claim-rewards'
      lead='Rewards are claimed in the Gold DAO app. The app finds your neurons through a hotkey, which you add once to each neuron in the NNS dapp.'
    >
      <h2>Why a hotkey</h2>
      <p>
        Internet Identity gives you a different principal in every app. Your
        principal in the Gold DAO app is therefore not the one that owns your
        neurons in the NNS dapp, even if you sign in to both with the same
        identity.
      </p>
      <p>
        Adding your Gold DAO app principal as a hotkey to a neuron lets you
        claim for it. A hotkey can vote, make proposals and claim rewards for
        the neuron. It cannot disburse or dissolve it.
      </p>

      <h2>Add your neuron to the app</h2>
      <ol>
        <li>
          Open the <Ext href={links.appGovern}>Gold DAO app</Ext> and sign in.
        </li>
        <li>
          On the <strong>Govern</strong> page click <strong>Add neuron</strong>
          . The dialog shows your principal. Copy it.
        </li>
        <li>
          In the <Ext href={links.nnsNeurons}>NNS dapp</Ext> go to{' '}
          <strong>Neuron Staking</strong>, select <strong>Gold DAO</strong> and
          open your neuron. Scroll to <strong>Hotkeys</strong>, click{' '}
          <strong>Add Hotkey</strong> and paste the principal.
          <Figure
            src='/docs/nns-add-hotkey.png'
            alt='The Hotkeys section of a neuron in the NNS dapp with the Add Hotkey button.'
            width={1200}
            height={315}
          />
        </li>
        <li>
          Go back to the Gold DAO app and reload the page. The neuron is now
          listed under <strong>My GOLDAO neurons</strong>.
        </li>
      </ol>
      <p>Repeat step 3 for every neuron you want to claim for.</p>

      <h2>Claim</h2>
      <p>
        When rewards are waiting, the Govern page shows{' '}
        <strong>Unclaimed rewards available</strong> with their total value in
        US dollars. <strong>Claim rewards</strong> in that box claims for all
        your neurons at once. The <strong>Claim rewards</strong> button in the
        row of a neuron claims for that neuron only.
      </p>
      <p>
        The tokens arrive in your wallet in the app after a few seconds. From
        the <Ext href={links.appWallet}>Wallet</Ext> page you can send them to
        another address.
      </p>
      <p>
        A token can be claimed once the amount is larger than the transfer fee
        of that token. The fee is deducted from what you receive.
      </p>

      <h2>Nothing to claim</h2>
      <p>If a neuron is listed but shows no rewards, check these points:</p>
      <ul>
        <li>
          The dissolve delay is below two years, or the neuron is dissolving.
          Such neurons receive no rewards, see{' '}
          <Link href={docsPath('rewards')}>Rewards</Link>.
        </li>
        <li>
          The neuron did not vote. Set it to follow another neuron if you do
          not vote yourself, see{' '}
          <Link href={docsPath('stake-goldao')}>Stake GOLDAO</Link>.
        </li>
        <li>
          Nothing was paid since the neuron became eligible. ICP, OGY and WTN
          are paid on Wednesdays, GLDT on the first Wednesday of the month.
        </li>
      </ul>
    </DocsPage>
  )
}
