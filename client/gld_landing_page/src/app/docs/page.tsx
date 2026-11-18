import Link from 'next/link'
import DocsPage from '@/components/docs/DocsPage'
import { Ext } from '@/components/docs/elements'
import { docsMetadata } from '@/docs/metadata'
import { docsNav, docsPath } from '@/docs/nav'
import { links, proposalUrl } from '@/links'

export const metadata = docsMetadata('')

// The sections of the sidebar, without the one this page is in
const sections = docsNav.filter(section =>
  section.pages.every(page => page.slug !== '')
)

export default function Page () {
  return (
    <DocsPage
      slug=''
      heading='Gold DAO docs'
      lead='How Gold DAO works, how to stake GOLDAO and how to claim rewards.'
    >
      <p>
        Gold DAO is a DAO on the Internet Computer. It holds a treasury of
        staked ICP, OGY and WTN. People who stake its governance token, GOLDAO,
        vote on what the DAO does and receive a share of what the treasury
        earns.
      </p>
      <p>
        The DAO runs as an SNS, so its canisters only change through proposals
        that GOLDAO neurons vote on. The model described in these docs was set
        by{' '}
        <Ext href={proposalUrl(317)}>
          proposal 317, &quot;Gold DAO Chapter 2.0&quot;
        </Ext>
        , adopted in December 2025.{' '}
        <Ext href={links.chapter2}>The paper that came with it</Ext> gives the
        reasoning.
      </p>

      {sections.map(section => (
        <section key={section.title}>
          <h2>{section.title}</h2>
          <ul>
            {section.pages.map(page => (
              <li key={page.slug}>
                <Link href={docsPath(page.slug)}>{page.title}</Link>
                <br />
                {page.description}
              </li>
            ))}
          </ul>
        </section>
      ))}

      <h2>Links</h2>
      <ul>
        <li>
          <Ext href={links.app}>Gold DAO app</Ext>: add your neurons, claim
          rewards, wallet
        </li>
        <li>
          <Ext href={links.proposals}>Proposals</Ext> in the NNS dapp
        </li>
        <li>
          <Ext href={links.icpDashboard}>Gold DAO on the ICP dashboard</Ext>:
          canisters, neurons, GOLDAO supply and transactions
        </li>
        <li>
          <Ext href={links.github}>Source code</Ext> on GitHub
        </li>
        <li>
          <Ext href={links.x}>X</Ext>
        </li>
      </ul>
    </DocsPage>
  )
}
