import type { Metadata } from 'next'
import { ogImage, siteName } from '@/site'
import { docsPages, docsPath } from './nav'

// Metadata of a docs page. Next.js replaces nested objects such as openGraph
// as a whole instead of merging them with the root layout, so they are spelled
// out here in full. Without the canonical URL a page would inherit "/".
export function docsMetadata (slug: string): Metadata {
  const page = docsPages.find(candidate => candidate.slug === slug)
  if (!page) {
    throw new Error(`"${slug}" is not listed in src/docs/nav.ts`)
  }

  const title = slug ? `${page.title} | Gold DAO docs` : 'Gold DAO docs'
  const { description } = page
  const url = docsPath(slug)

  return {
    title,
    description,
    alternates: { canonical: url },
    openGraph: {
      type: 'article',
      url,
      siteName,
      title,
      description,
      images: [ogImage]
    },
    twitter: {
      card: 'summary_large_image',
      title,
      description,
      images: [ogImage]
    }
  }
}
