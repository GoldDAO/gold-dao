import type { ReactNode } from 'react'

// A link to another site. Links between docs pages use next/link.
export function Ext ({ href, children }: { href: string, children: ReactNode }) {
  return (
    <a href={href} target='_blank' rel='noopener noreferrer'>
      {children}
    </a>
  )
}

// Something the reader should not miss
export function Note ({ children }: { children: ReactNode }) {
  return (
    <div role='note' className='docs-note'>
      {children}
    </div>
  )
}

// A table that scrolls sideways on its own when it is wider than the screen.
// With labels, the first column is a short label per row (a list of facts)
// and is kept on one line.
export function Table ({
  labels = false,
  children
}: {
  labels?: boolean
  children: ReactNode
}) {
  return (
    <div className={labels ? 'docs-table docs-table-labels' : 'docs-table'}>
      <table>{children}</table>
    </div>
  )
}

// An image from public/docs. Width and height are those of the file, so the
// page does not jump while the image loads. maxWidth is the displayed width
// in pixels when that should be less than the column.
export function Figure ({
  src,
  alt,
  width,
  height,
  maxWidth,
  caption
}: {
  src: string
  alt: string
  width: number
  height: number
  maxWidth?: number
  caption?: ReactNode
}) {
  return (
    <figure>
      {/* The site is a static export, which has no image optimizer for next/image */}
      {/* eslint-disable-next-line @next/next/no-img-element */}
      <img
        src={src}
        alt={alt}
        width={width}
        height={height}
        loading='lazy'
        style={maxWidth ? { maxWidth } : undefined}
      />
      {caption && <figcaption>{caption}</figcaption>}
    </figure>
  )
}
