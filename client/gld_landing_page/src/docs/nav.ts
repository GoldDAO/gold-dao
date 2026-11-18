// The docs pages, in the order of the sidebar. A page lives in
// src/app/docs/<slug>/page.tsx; the introduction has the empty slug and lives
// in src/app/docs/page.tsx. The title is also the page heading, the description
// goes into the page metadata.
export interface DocsPage {
  slug: string
  title: string
  description: string
}

export interface DocsSection {
  title: string
  pages: DocsPage[]
}

export const docsNav: DocsSection[] = [
  {
    title: 'Start',
    pages: [
      {
        slug: '',
        title: 'Introduction',
        description:
          'How Gold DAO works, how to stake GOLDAO and how to claim rewards.'
      }
    ]
  },
  {
    title: 'Guides',
    pages: [
      {
        slug: 'stake-goldao',
        title: 'Stake GOLDAO',
        description:
          'Get GOLDAO, create a neuron in the NNS dapp and set it up to vote and to receive rewards.'
      },
      {
        slug: 'claim-rewards',
        title: 'Claim rewards',
        description:
          'Add your GOLDAO neurons to the Gold DAO app with a hotkey and claim their rewards.'
      }
    ]
  },
  {
    title: 'How it works',
    pages: [
      {
        slug: 'rewards',
        title: 'Rewards',
        description:
          'Which neurons receive rewards, where the ICP, OGY, WTN and GLDT come from and when they are paid.'
      },
      {
        slug: 'treasury',
        title: 'Treasury',
        description:
          'The ICP, OGY and WTN neurons Gold DAO owns, the canisters that hold them and where their rewards go.'
      },
      {
        slug: 'buyback-and-burn',
        title: 'Buyback and burn',
        description:
          'How Gold DAO buys back GOLDAO with ICP rewards and burns the GOLDAO held by its treasury.'
      }
    ]
  },
  {
    title: 'Reference',
    pages: [
      {
        slug: 'goldao-token',
        title: 'GOLDAO token',
        description:
          'Ledger, fee, minimum stake and where to trade the GOLDAO governance token.'
      },
      {
        slug: 'canisters',
        title: 'Canisters',
        description: 'The canisters of the Gold DAO SNS and what each one does.'
      },
      {
        slug: 'brand-assets',
        title: 'Brand assets',
        description: 'Gold DAO logos, colors and typeface.'
      },
      {
        slug: 'faq',
        title: 'FAQ',
        description:
          'Short answers about staking, rewards, principals and what happened to GLDT.'
      }
    ]
  },
  {
    title: 'Legal',
    pages: [
      {
        slug: 'terms-of-service',
        title: 'Terms of Service',
        description: 'Terms of Service of Gold DAO.'
      },
      {
        slug: 'privacy-policy',
        title: 'Privacy Policy',
        description: 'Privacy Policy of Gold DAO.'
      }
    ]
  }
]

export const docsPages = docsNav.flatMap(section => section.pages)

// Paths end with a slash because of trailingSlash in next.config.js
export const docsPath = (slug: string) => (slug ? `/docs/${slug}/` : '/docs/')
