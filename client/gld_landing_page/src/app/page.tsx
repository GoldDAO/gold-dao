import Link from 'next/link'
import Logo from '@/components/Logo'
import SiteFooter from '@/components/SiteFooter'
import SiteHeader from '@/components/SiteHeader'
import Treasury from '@/components/Treasury'
import { docsPath } from '@/docs/nav'
import { links } from '@/links'
import { container } from '@/site'

const newTab = { target: '_blank', rel: 'noopener noreferrer' }

const steps = [
  {
    title: 'Stake',
    text: 'Lock GOLDAO in a neuron in the NNS dapp. The staking guide covers getting GOLDAO and creating the neuron.',
    link: { label: 'Read the staking guide', href: docsPath('stake-goldao') }
  },
  {
    title: 'Vote',
    text: "Proposals decide how the treasury is used and how the DAO's canisters change. Vote on each one, or follow a neuron you trust.",
    link: { label: 'See the proposals', href: links.proposals, ...newTab }
  },
  {
    title: 'Claim',
    text: 'A share of what the treasury earns goes to GOLDAO neurons locked for at least two years and not dissolving. Add your neuron to the Gold DAO app and claim there.',
    link: { label: 'Launch app', href: links.app }
  }
]

// The asset canister answers every path it does not know with this page. Under
// /docs that is an old GitBook link (docs.gold-dao.org forwards here and keeps
// the path) or a typo, and the visitor belongs on the docs, not on the home
// page. The script runs while the HTML is parsed, before anything is shown.
// The real docs pages are files of their own and never contain it.
const docsFallback = `if(location.pathname.indexOf('/docs/')===0)location.replace('${docsPath('')}')`

export default function Page () {
  return (
    <>
      <script dangerouslySetInnerHTML={{ __html: docsFallback }} />
      <SiteHeader />

      <main>
        <section className='overflow-hidden bg-black text-white'>
          <div className={`${container} relative pb-24 pt-16 md:pb-36 md:pt-24`}>
            {/* Lines up with the right edge of the content and runs off the bottom of the section */}
            <Logo
              id='hero-mark'
              className='pointer-events-none absolute -bottom-20 right-5 hidden h-[26rem] sm:right-8 lg:block xl:-bottom-24 xl:h-[30rem]'
            />
            <h1 className='relative font-display text-[clamp(2.75rem,7.2vw,5.5rem)] leading-none tracking-[-0.01em]'>
              <span className='block font-semibold text-gold'>
                Stake GOLDAO.
              </span>
              <span className='block font-light'>Govern the treasury.</span>
            </h1>
            <p className='relative mt-8 max-w-xl text-lg leading-relaxed text-white/75 md:text-xl md:leading-relaxed'>
              Gold DAO holds a treasury of staked ICP, OGY and WTN on the
              Internet Computer. GOLDAO stakers vote on what happens with it
              and receive a share of what it earns.
            </p>
            <div className='relative mt-10 flex flex-wrap items-center gap-x-8 gap-y-4'>
              <a href={links.app} className='button button-gold'>
                Launch app
              </a>
              <a href='#how-it-works' className='text-link'>
                How it works
              </a>
            </div>
          </div>
        </section>

        <Treasury />

        <section
          id='how-it-works'
          aria-labelledby='how-it-works-title'
          className={`${container} py-20 md:py-28`}
        >
          <h2
            id='how-it-works-title'
            className='text-3xl font-semibold md:text-4xl'
          >
            How it works
          </h2>
          <ol className='mt-10 grid gap-10 md:mt-14 md:grid-cols-3 md:gap-12'>
            {steps.map(({ title, text, link: { label, href, ...link } }, index) => (
              <li
                key={title}
                className='flex flex-col border-t border-content pt-6'
              >
                <h3 className='flex items-baseline gap-3 text-2xl font-semibold'>
                  <span className='font-display font-light text-content/60'>
                    {index + 1}
                  </span>
                  {title}
                </h3>
                <p className='mt-3 leading-relaxed text-content/70 md:text-lg md:leading-relaxed'>
                  {text}
                </p>
                {/* The staking guide is a page of this site and goes through the Next.js router */}
                {href.startsWith('/') ? (
                  <Link
                    href={href}
                    className='text-link mt-5 self-start font-medium'
                  >
                    {label}
                  </Link>
                ) : (
                  <a
                    href={href}
                    {...link}
                    className='text-link mt-5 self-start font-medium'
                  >
                    {label}
                  </a>
                )}
              </li>
            ))}
          </ol>
        </section>
      </main>

      <SiteFooter />
    </>
  )
}
