import DocsPage from '@/components/docs/DocsPage'
import { Table } from '@/components/docs/elements'
import { docsMetadata } from '@/docs/metadata'

export const metadata = docsMetadata('brand-assets')

// Files in public/brand. The white versions have a white wordmark for dark
// backgrounds and come as SVG only.
const logos = [
  {
    name: 'Logo',
    file: 'gold-dao-logo',
    width: 802,
    height: 255,
    dark: false,
    png: true
  },
  {
    name: 'Logo, white wordmark',
    file: 'gold-dao-logo-white',
    width: 802,
    height: 255,
    dark: true,
    png: false
  },
  {
    name: 'Stacked logo',
    file: 'gold-dao-logo-vertical',
    width: 277,
    height: 361,
    dark: false,
    png: true
  },
  {
    name: 'Stacked logo, white wordmark',
    file: 'gold-dao-logo-vertical-white',
    width: 277,
    height: 361,
    dark: true,
    png: false
  },
  {
    name: 'Token icon',
    file: 'gold-dao-icon',
    width: 528,
    height: 527,
    dark: false,
    png: true
  }
]

// The color tokens of tailwind.config.ts, which are the ones of the Gold DAO app
const colors = [
  { name: 'Gold', hex: '#D3B872', oklch: 'oklch(79.05% 0.0944 89.09)' },
  { name: 'Black', hex: '#000000', oklch: 'oklch(0% 0 0)' },
  { name: 'Background', hex: '#FDFCFB', oklch: 'oklch(99.16% 0.0017 67.8)' },
  { name: 'Line', hex: '#E8E6E1', oklch: 'oklch(92.51% 0.0071 88.65)' }
]

export default function Page () {
  return (
    <DocsPage
      slug='brand-assets'
      lead='Logos, colors and typeface of Gold DAO, for anyone who writes about the DAO or lists GOLDAO.'
    >
      <h2>Logos</h2>
      <div className='mt-6 grid gap-6 sm:grid-cols-2'>
        {logos.map(({ name, file, width, height, dark, png }) => (
          <div key={file}>
            <div
              className={`flex h-44 items-center justify-center rounded-lg border border-line p-8 ${
                dark ? 'bg-black' : 'bg-white'
              }`}
            >
              {/* The site is a static export, which has no image optimizer for next/image */}
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={`/brand/${file}.svg`}
                alt={`${name} of Gold DAO`}
                width={width}
                height={height}
                loading='lazy'
                className='max-h-full w-auto max-w-full'
              />
            </div>
            <p className='!mt-3'>
              {name}:{' '}
              <a href={`/brand/${file}.svg`} download>
                SVG
              </a>
              {png && (
                <>
                  ,{' '}
                  <a href={`/brand/${file}.png`} download>
                    PNG
                  </a>
                </>
              )}
            </p>
          </div>
        ))}
      </div>

      <h2>Colors</h2>
      <Table>
        <thead>
          <tr>
            <th scope='col'>Color</th>
            <th scope='col'>Hex</th>
            <th scope='col'>OKLCH</th>
          </tr>
        </thead>
        <tbody>
          {colors.map(({ name, hex, oklch }) => (
            <tr key={name}>
              <th scope='row'>
                <span className='flex items-center gap-3'>
                  <span
                    aria-hidden='true'
                    className='h-6 w-6 shrink-0 rounded-full border border-line'
                    style={{ backgroundColor: hex }}
                  />
                  {name}
                </span>
              </th>
              <td>
                <code className='whitespace-nowrap'>{hex}</code>
              </td>
              <td>
                <code>{oklch}</code>
              </td>
            </tr>
          ))}
        </tbody>
      </Table>

      <h2>Typeface</h2>
      <p>
        This website and the Gold DAO app are set in Polymath, which is
        licensed through Adobe Fonts. The wordmark in the logo files is set in
        Inter and comes as outlines, so the logos need no font.
      </p>
    </DocsPage>
  )
}
