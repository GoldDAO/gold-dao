import { MetadataRoute } from 'next'
import { docsPages, docsPath } from '@/docs/nav'
import { siteUrl } from '@/site'

export default function sitemap (): MetadataRoute.Sitemap {
  const lastModified = new Date()

  return [
    {
      url: `${siteUrl}/`,
      lastModified,
      changeFrequency: 'yearly',
      priority: 1
    },
    ...docsPages.map(page => ({
      url: `${siteUrl}${docsPath(page.slug)}`,
      lastModified,
      changeFrequency: 'monthly' as const,
      priority: 0.7
    }))
  ]
}
