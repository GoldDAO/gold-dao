import DocsPage from '@/components/docs/DocsPage'
import { Ext } from '@/components/docs/elements'
import { canisters } from '@/docs/canisters'
import { docsMetadata } from '@/docs/metadata'
import { canisterUrl, links, proposalUrl } from '@/links'

export const metadata = docsMetadata('canisters')

interface Row {
  name: string
  id: string
  does: string
}

const snsCanisters: Row[] = [
  {
    name: 'Root',
    id: canisters.snsRoot,
    does: 'Controls the other canisters of the DAO'
  },
  {
    name: 'Governance',
    id: canisters.snsGovernance,
    does: 'Neurons, proposals and voting'
  },
  { name: 'Ledger', id: canisters.snsLedger, does: 'The GOLDAO ledger' },
  {
    name: 'Index',
    id: canisters.snsIndex,
    does: 'Transaction history of GOLDAO accounts'
  },
  {
    name: 'Ledger archive',
    id: canisters.snsArchive,
    does: 'Older blocks of the GOLDAO ledger'
  },
  {
    name: 'Swap',
    id: canisters.snsSwap,
    does: 'The decentralization swap of December 2023. It is closed.'
  }
]

const daoCanisters: Row[] = [
  {
    name: 'gld_dashboard',
    id: canisters.app,
    does: 'The Gold DAO app at app.gold-dao.org'
  },
  {
    name: 'sns_rewards',
    id: canisters.snsRewards,
    does: 'Tracks the maturity of the GOLDAO neurons, holds the reward pool and pays out the rewards'
  },
  {
    name: 'icp_neuron',
    id: canisters.icpNeuron,
    does: 'Holds the ICP neuron and passes on its voting rewards'
  },
  {
    name: 'sns_neuron_controller',
    id: canisters.snsNeuronController,
    does: 'Holds the OGY and WTN neurons and passes on their rewards'
  },
  {
    name: 'buyback_burn',
    id: canisters.buybackBurn,
    does: 'Buys back and burns GOLDAO, buys GLDT for stakers'
  }
]

// The ledgers the rewards canister pays from
const rewardLedgers: Row[] = [
  { name: 'ICP', id: 'ryjl3-tyaaa-aaaaa-aaaba-cai', does: 'Run by the NNS' },
  {
    name: 'OGY',
    id: 'lkwrt-vyaaa-aaaaq-aadhq-cai',
    does: 'Run by the ORIGYN SNS'
  },
  {
    name: 'WTN',
    id: 'jcmow-hyaaa-aaaaq-aadlq-cai',
    does: 'Run by the WaterNeuron SNS'
  },
  {
    name: 'GLDT',
    id: '6c7su-kiaaa-aaaar-qaira-cai',
    does: 'Run by the ORIGYN SNS'
  }
]

// One row per canister: name and purpose, and the id as a link to the ICP
// dashboard. The id is kept on one line; on a phone it moves below the name.
function CanisterList ({ rows }: { rows: Row[] }) {
  return (
    <ul className='docs-rows'>
      {rows.map(({ name, id, does }) => (
        <li key={id}>
          <span>
            <strong className='block'>{name}</strong>
            {does}
          </span>
          <span className='shrink-0'>
            <Ext href={canisterUrl(id)}>
              <code>{id}</code>
            </Ext>
          </span>
        </li>
      ))}
    </ul>
  )
}

export default function Page () {
  return (
    <DocsPage
      slug='canisters'
      lead='The canisters of the Gold DAO SNS and what each one does. Every id links to the canister on the ICP dashboard.'
    >
      <p>
        The list can be checked against the{' '}
        <Ext href={links.icpDashboard}>Gold DAO page of the ICP dashboard</Ext>
        , which reads it from the root canister. The code of the canisters is{' '}
        <Ext href={links.github}>on GitHub</Ext>, where{' '}
        <Ext href={links.githubCanisterIds}>canister_ids.json</Ext> has the ids
        for every network.
      </p>

      <h2>SNS canisters</h2>
      <CanisterList rows={snsCanisters} />

      <h2>Canisters the DAO controls</h2>
      <p>
        These are registered with the SNS. Their code only changes through a
        proposal.
      </p>
      <CanisterList rows={daoCanisters} />

      <h2>Ledgers of the reward tokens</h2>
      <CanisterList rows={rewardLedgers} />

      <h2>GLDT canisters</h2>
      <p>
        The GLDT ledger, its index and the GLDT swap canisters are no longer
        Gold DAO canisters. They were handed over to the ORIGYN DAO with
        proposals <Ext href={proposalUrl(403)}>403</Ext> and{' '}
        <Ext href={proposalUrl(406)}>406</Ext> in autumn 2026.
      </p>
    </DocsPage>
  )
}
