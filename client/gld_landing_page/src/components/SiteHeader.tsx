import Link from 'next/link'
import Logo from '@/components/Logo'
import { docsPath } from '@/docs/nav'
import { links } from '@/links'
import { container } from '@/site'

export default function SiteHeader () {
  return (
    <header className='bg-black text-white'>
      <div className={`${container} flex items-center justify-between py-5`}>
        <Link href='/' className='flex items-center gap-3'>
          <Logo id='header-mark' className='h-9 w-auto' />
          <span className='text-xl font-medium'>Gold DAO</span>
        </Link>
        <nav aria-label='Main' className='flex items-center gap-6'>
          <Link
            href={docsPath('')}
            className='hidden hover:underline hover:underline-offset-4 sm:block'
          >
            Docs
          </Link>
          <a href={links.app} className='button button-outline h-11 px-5'>
            Launch app
          </a>
        </nav>
      </div>
    </header>
  )
}
