import Link from 'next/link'
import Logo from '@/components/Logo'
import { docsPath } from '@/docs/nav'
import { links } from '@/links'
import { container } from '@/site'

const newTab = { target: '_blank', rel: 'noopener noreferrer' }

const footerLinks = [
  { label: 'App', href: links.app },
  { label: 'Docs', href: docsPath('') },
  { label: 'Proposals', href: links.proposals, ...newTab },
  { label: 'ICP dashboard', href: links.icpDashboard, ...newTab },
  { label: 'X', href: links.x, ...newTab }
]

export default function SiteFooter () {
  return (
    <footer className='bg-black text-white'>
      <div className={`${container} py-12`}>
        <div className='flex flex-col gap-8 md:flex-row md:items-center md:justify-between'>
          <div className='flex items-center gap-3'>
            <Logo id='footer-mark' className='h-8 w-auto' />
            <span className='text-lg font-medium'>Gold DAO</span>
          </div>
          <nav aria-label='Footer'>
            <ul className='flex flex-wrap gap-x-7 gap-y-3'>
              {footerLinks.map(({ label, href, ...link }) => (
                <li key={label}>
                  {/* Pages of this site go through the Next.js router */}
                  {href.startsWith('/') ? (
                    <Link
                      href={href}
                      className='hover:underline hover:underline-offset-4'
                    >
                      {label}
                    </Link>
                  ) : (
                    <a
                      href={href}
                      {...link}
                      className='hover:underline hover:underline-offset-4'
                    >
                      {label}
                    </a>
                  )}
                </li>
              ))}
            </ul>
          </nav>
        </div>
        <p className='mt-10 border-t border-white/20 pt-6 text-sm text-white/60'>
          © {new Date().getFullYear()} Gold DAO. GOLDAO is a governance token.
          Rewards vary and are not guaranteed.
        </p>
      </div>
    </footer>
  )
}
