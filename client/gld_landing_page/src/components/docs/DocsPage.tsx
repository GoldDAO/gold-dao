import type { ReactNode } from 'react'
import Link from 'next/link'
import SiteFooter from '@/components/SiteFooter'
import SiteHeader from '@/components/SiteHeader'
import { docsNav, docsPages, docsPath } from '@/docs/nav'
import { container } from '@/site'

function NavList ({ current }: { current: string }) {
  return (
    <ul className='space-y-6'>
      {docsNav.map(section => (
        <li key={section.title}>
          <p className='text-sm text-content/60'>{section.title}</p>
          <ul className='mt-2 border-l border-line'>
            {section.pages.map(page => (
              <li key={page.slug}>
                <Link
                  href={docsPath(page.slug)}
                  aria-current={page.slug === current ? 'page' : undefined}
                  className='docs-nav-link'
                >
                  {page.title}
                </Link>
              </li>
            ))}
          </ul>
        </li>
      ))}
    </ul>
  )
}

// Frame of a docs page: site header, sidebar, heading, content and the links
// to the previous and next page. Each page passes its own slug, so the sidebar
// can mark the current page without client-side code.
export default function DocsPage ({
  slug,
  heading,
  lead,
  children
}: {
  slug: string
  // Defaults to the title from src/docs/nav.ts
  heading?: string
  lead: ReactNode
  children: ReactNode
}) {
  const index = docsPages.findIndex(page => page.slug === slug)
  if (index === -1) {
    throw new Error(`"${slug}" is not listed in src/docs/nav.ts`)
  }
  const page = docsPages[index]
  const previous = docsPages[index - 1]
  const next = docsPages[index + 1]

  return (
    <>
      <SiteHeader />

      <div
        className={`${container} lg:grid lg:grid-cols-[13rem_minmax(0,1fr)] lg:gap-16`}
      >
        {/* Below the lg breakpoint the sidebar folds into a menu */}
        <details className='group border-b border-line lg:hidden'>
          <summary className='flex cursor-pointer list-none items-center justify-between py-4 [&::-webkit-details-marker]:hidden'>
            <span>
              <span className='text-content/60'>Docs / </span>
              <span className='font-medium'>{page.title}</span>
            </span>
            <svg
              viewBox='0 0 16 16'
              aria-hidden='true'
              className='h-4 w-4 transition-transform group-open:rotate-180'
            >
              <path
                d='M3 6l5 5 5-5'
                fill='none'
                stroke='currentColor'
                strokeWidth='1.5'
              />
            </svg>
          </summary>
          <nav aria-label='Docs' className='pb-6 pt-2'>
            <NavList current={slug} />
          </nav>
        </details>

        <nav aria-label='Docs' className='hidden lg:block'>
          <div className='sticky top-0 max-h-screen overflow-y-auto py-14'>
            <NavList current={slug} />
          </div>
        </nav>

        <main className='min-w-0 pb-20 pt-10 lg:pb-28 lg:pt-14'>
          <article className='docs-content max-w-3xl'>
            <h1>{heading ?? page.title}</h1>
            <p className='docs-lead'>{lead}</p>
            {children}
          </article>

          <nav
            aria-label='Previous and next page'
            className='mt-16 grid max-w-3xl gap-6 border-t border-line pt-8 sm:grid-cols-2'
          >
            {previous && (
              <Link href={docsPath(previous.slug)} className='group/link'>
                <span className='block text-sm text-content/60'>Previous</span>
                <span className='font-medium group-hover/link:underline group-hover/link:underline-offset-4'>
                  {previous.title}
                </span>
              </Link>
            )}
            {next && (
              <Link
                href={docsPath(next.slug)}
                className='group/link sm:col-start-2 sm:text-right'
              >
                <span className='block text-sm text-content/60'>Next</span>
                <span className='font-medium group-hover/link:underline group-hover/link:underline-offset-4'>
                  {next.title}
                </span>
              </Link>
            )}
          </nav>
        </main>
      </div>

      <SiteFooter />
    </>
  )
}
