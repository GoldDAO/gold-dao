# Gold DAO landing page

The website at <https://www.gold-dao.org>. It has a home page (what Gold DAO is, what the treasury holds, and where staking, voting and claiming happen) and the docs under `/docs`, which replace the GitBook that used to be on docs.gold-dao.org. Built with Next.js 14 (App Router), exported as static files and served from an asset canister.

## Run it locally

From the repository root, with the same install command CI uses:

```bash
pnpm --filter gld_landing_page install --frozen-lockfile
pnpm --filter gld_landing_page run dev
```

The page is then on `http://localhost:3000`.

## Where things are

- `src/app/page.tsx` holds the home page and its copy.
- `src/app/layout.tsx` holds the metadata of the home page (title, description, social preview) and loads the typeface.
- `src/components/SiteHeader.tsx` and `SiteFooter.tsx` are the header and footer of every page.
- `src/components/Treasury.tsx` is the treasury table. The browser reads it from `https://api.gldt.org/v1/daos/golddao/treasury/summary`. If the request fails or the response changes shape, a short notice replaces the table.
- `src/links.ts` lists every outgoing link.
- `tailwind.config.ts` has the colors and typefaces, the same ones as `client/gld_dashboard`.
- `public/.ic-assets.json5` sets the response headers of the asset canister, including the Content-Security-Policy. A new external host (an API, a font, an image) has to be added there or the browser blocks it.
- `public/og.png` is the social preview image, 1200 x 630.

## The docs

- `src/docs/nav.ts` lists the docs pages in the order of the sidebar, with the title and description of each. The sidebar, the previous and next links, the page metadata and the sitemap are all built from it.
- A page is `src/app/docs/<slug>/page.tsx`; the introduction is `src/app/docs/page.tsx`. A page wraps its content in `DocsPage` (`src/components/docs/DocsPage.tsx`) and writes plain HTML elements. Their styles are the `.docs-content` rules in `src/app/globals.css`.
- `src/components/docs/elements.tsx` has the few extra elements: external link, note, table and figure.
- `src/docs/canisters.ts` has the canister and neuron ids the pages refer to.
- `public/docs` holds the screenshots, `public/brand` the logo files of the brand assets page.

To add a page, add it to `src/docs/nav.ts` and create its `page.tsx`. The build fails if a page uses a slug that is not in the list.

The pages state facts that live in canister code and canister configuration: reward shares, thresholds, schedules, dissolve delays, neuron ids. When a release of `sns_rewards`, `icp_neuron`, `sns_neuron_controller` or `buyback_burn` changes one of them, the docs have to change with it.

`next.config.js` sets `trailingSlash`, so every page is exported as `<path>/index.html`. The asset canister then serves `/docs` and `/docs/` alike.

docs.gold-dao.org forwards to `/docs` and keeps the path of the old GitBook link, so such a link arrives as `/docs/how-to/...`. The asset canister answers a path it does not know with the home page. A one-line script at the top of the home page (`docsFallback` in `src/app/page.tsx`) therefore sends any unknown path under `/docs/` to the docs introduction.

## Build and deploy

```bash
pnpm --filter gld_landing_page run build
```

This writes the static site to `dist`, the folder `dfx.json` deploys for the `gld_landing_page` canister.

CI deploys to staging when a change to this folder lands on `develop`, and offers the staging deploy as a manual job on merge requests. Production is deployed from a tag named `gld_landing_page-vX.Y.Z`, with a manual deploy job. The canister ids are in `canister_ids.json` at the repository root.
