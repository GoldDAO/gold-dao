import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext } from '@/components/docs/elements'
import { canisters } from '@/docs/canisters'
import { docsMetadata } from '@/docs/metadata'
import { docsPath } from '@/docs/nav'
import {
  canisterUrl,
  goldaoAccountUrl,
  icpAccountUrl,
  links,
  proposalUrl
} from '@/links'

export const metadata = docsMetadata('buyback-and-burn')

// ICP ledger account of the buyback_burn canister
const BUYBACK_ICP_ACCOUNT =
  '31836130dcff35502d04752ea5b82a24e44d41955f2a30bb8c2d284f4a318d82'
// The account of the rewards canister that used to be the GOLDAO reserve pool
// (subaccount 1). GOLDAO from the treasury passes through it to be burned.
const BURN_WALLET = `${canisters.snsRewards}-j6jb7ey.100000000000000000000000000000000000000000000000000000000000000`

export default function Page () {
  return (
    <DocsPage
      slug='buyback-and-burn'
      lead='A third of the ICP that the ICP neuron earns is set aside to buy back GOLDAO and burn it. The canister only buys while GOLDAO is below a set price. Apart from that, the GOLDAO held by the treasury is being burned.'
    >
      <h2>Buyback</h2>
      <p>
        33% of the voting rewards of the ICP neuron go to the buyback_burn
        canister{' '}
        <Ext href={canisterUrl(canisters.buybackBurn)}>
          <code>{canisters.buybackBurn}</code>
        </Ext>
        . Every four hours, at a random moment within that window, the canister
        takes 1/42 of its ICP balance and looks at the prices on ICPSwap. It
        does the first of these that applies:
      </p>
      <ol>
        <li>
          If 1 ICP buys 500 GOLDAO or more, it buys GOLDAO. The canister burns
          the GOLDAO it holds every day at 12:00 UTC.
        </li>
        <li>
          If 1 ICP buys 1,000 OGY or more, it buys OGY and adds it to the OGY
          neuron of the DAO.
        </li>
        <li>Otherwise it adds the ICP to the ICP neuron of the DAO.</li>
      </ol>
      <p>
        So the ICP reduces the GOLDAO supply while GOLDAO is cheap, and grows
        the treasury while it is not. Using a small part of the balance each
        time keeps a single trade from moving the price much. Earlier versions
        of the canister bought GOLDAO at any price. The rules above apply since
        version 1.3.0 from May 2026.
      </p>
      <p>
        The trades are in the{' '}
        <Ext href={icpAccountUrl(BUYBACK_ICP_ACCOUNT)}>ICP transactions</Ext>{' '}
        of the canister, the burns in its{' '}
        <Ext href={goldaoAccountUrl(canisters.buybackBurn)}>
          GOLDAO transactions
        </Ext>
        .
      </p>

      <h2>GLDT for stakers</h2>
      <p>
        The same canister receives another 33% of the voting rewards on a
        separate account. Every six hours it swaps 1/28 of that balance to GLDT
        on ICPSwap and sends the GLDT to the rewards pool, from where it is
        paid to GOLDAO neurons once a month. See{' '}
        <Link href={docsPath('rewards')}>Rewards</Link>.
      </p>

      <h2>Burning the GOLDAO of the treasury</h2>
      <p>
        <Ext href={proposalUrl(317)}>Proposal 317</Ext> decided that the GOLDAO
        held by the SNS treasury is burned. A burn straight from the treasury
        fails because of a bug in the SNS framework (
        <Ext href={proposalUrl(325)}>proposal 325</Ext>), so each batch takes
        two proposals:
      </p>
      <ol>
        <li>
          One transfers a batch of GOLDAO from the treasury to the{' '}
          <Ext href={goldaoAccountUrl(BURN_WALLET)}>burn wallet</Ext>, an
          account of the rewards canister.
        </li>
        <li>
          One calls <code>burn_goldao_reserve_pool</code> on the rewards
          canister. The method sends everything in the burn wallet to the
          minting account of the GOLDAO ledger, which destroys the tokens.
        </li>
      </ol>
      <p>
        Proposals <Ext href={proposalUrl(413)}>413</Ext> and{' '}
        <Ext href={proposalUrl(415)}>415</Ext> from November 2026 are one such
        pair.
      </p>

      <h2>Supply</h2>
      <p>
        The current GOLDAO supply and every burn transaction are on the{' '}
        <Ext href={links.icpDashboard}>ICP dashboard</Ext>.
      </p>
    </DocsPage>
  )
}
