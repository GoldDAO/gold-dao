# Gold DAO landing page

The website at <https://www.gold-dao.org>. It is one static page: what Gold DAO is, what the treasury holds, and where staking, voting and claiming happen. Built with Next.js 14 (App Router), exported as static files and served from an asset canister.

## Run it locally

From the repository root, with the same install command CI uses:

```bash
pnpm --filter gld_landing_page install --frozen-lockfile
pnpm --filter gld_landing_page run dev
```

The page is then on `http://localhost:3000`.

## Where things are

- `src/app/page.tsx` holds the page and its copy.
- `src/app/layout.tsx` holds the metadata (title, description, social preview) and loads the typeface.
- `src/components/Treasury.tsx` is the treasury table. The browser reads it from `https://api.gldt.org/v1/daos/golddao/treasury/summary`. If the request fails or the response changes shape, a short notice replaces the table.
- `src/links.ts` lists every outgoing link.
- `tailwind.config.ts` has the colors and typefaces, the same ones as `client/gld_dashboard`.
- `public/.ic-assets.json5` sets the response headers of the asset canister, including the Content-Security-Policy. A new external host (an API, a font, an image) has to be added there or the browser blocks it.
- `public/og.png` is the social preview image, 1200 x 630.

## Build and deploy

```bash
pnpm --filter gld_landing_page run build
```

This writes the static site to `dist`, the folder `dfx.json` deploys for the `gld_landing_page` canister.

CI deploys to staging when a change to this folder lands on `develop`, and offers the staging deploy as a manual job on merge requests. Production is deployed from a tag named `gld_landing_page-vX.Y.Z`, with a manual deploy job. The canister ids are in `canister_ids.json` at the repository root.
