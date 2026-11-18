# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [2.5.1] - 2026-11-18

#### Changed

- The Docs link and the staking guide link in the "Add Neuron" dialog open the docs on `www.gold-dao.org/docs`. They pointed to `docs.gold-dao.org`, which is only a forward to the new docs now.

#### Fix

- Allow requests to `api.gldt.org` in the Content-Security-Policy. Version 2.5.0 reads the gold price from there, but the policy still listed the previous price source, so the browser blocked the request and the USD value of the GLD NFTs was still not shown.

### [2.5.0] - 2026-11-18

#### Added

- Show the total USD value of the wallet: the token balances plus the gold value of the GLD NFTs

#### Changed

- Move Earn out of the navigation; it is now a button in the GLDT tab of the wallet
- List the wallet tokens as GOLDAO, ICP, OGY, GLDT, GLD NFT, ckUSDT, WTN and select GOLDAO by default
- Drop `app.gldt.org` from the custom domains and the Internet Identity alternative origins; the domain now forwards to app.gold-dao.org at the DNS level

#### Fix

- Read the gold price from api.gldt.org; the previous source stopped answering, so the USD value of the GLD NFTs was not shown

### [2.4.0] - 2026-11-17

#### Changed

- Rebrand the app from GLDT to Gold DAO: title "Gold DAO dApp", Gold DAO logo and wordmark, favicon, description and Open Graph tags
- Add `app.gold-dao.org` as custom domain and Internet Identity alternative origin; `app.gldt.org` stays listed during the transition
- Point the Docs link to docs.gold-dao.org and the "Buy GLDT" button in the burn flow to gldt.org
- `pnpm dev` now runs against the mainnet canisters (`.env.development`)

#### Fix

- `deploy:staging` and `deploy:preprod` deployed `gldt_swap_app` instead of `gld_dashboard`

### [2.3.3] - 2026-10-27

#### Fix

- Remove the failing call to the stopped GOLDAO super_stats canister on the home page

### [2.3.2] - 2026-05-13

#### Fix

- Bug fixes

### [2.3.1] - 2026-05-13

#### Fix

- Internet Identity login failing with `Unable to connect` after the migration from `identity.internetcomputer.org` to `id.ai`

### [2.3.0] - 2026-04-02

#### Changed

- Prepare for KongSwap shutdown
- Replaced all KongSwap canister calls with ICPSwap's getAllTokens API for USD price lookups across the app (govern, earn, wallet)
- Made /govern the default route, with /buy redirecting to it
- Minor fix regarding the position of the $ sign

#### Removed

- Removed the buy page and swap feature from the wallet
- Removed KongSwap service files, useFetchSwapAmount hook, and KONGSWAP_CANISTER_ID_IC constant

### [2.2.5] - 2026-01-22

#### Changed

- Disable the ability to claim GOLDAO rewards
- Enable the ability to claim GLDT rewards
- Minor improvements for better user experience

### [2.2.4] - 2025-12-11

#### Changed

- Disabled the ability to stake GLDT on the dashboard

### [2.2.3] - 2025-12-10

#### Added

- Gold NFT viewer
- User can select specific NFTs for transfer, mint and burn
- Minor improvements for better user experience

### [2.2.2] - 2025-12-01

#### Fix

- Display correct NFT value on wallet transactions history view

### [2.2.1] - 2025-11-27

#### Fix

- Use correct canisters for NFT collections

### [2.2.0] - 2025-11-24

#### Changed

- Update for the new NFT version
- UI improvements for better user experience

### [2.1.4] - 2025-10-16

#### Changed

- UI improvements for better user experience

### [2.1.3] - 2025-10-15

#### Changed

- Remove rewards OGY & ICP on GLDT stake

### [2.1.2] - 2025-10-09

#### Fix

- Fetching stake user positions
- Incorrect stake fee amount
- Minor UI

### [2.1.1] - 2025-09-09

#### Changed

- Fix affected qix package versions

### [2.1.0] - 2025-09-05

#### Added

- GLDT Stake
- Enable PLUG wallet

#### Changed

- UI improvements for better user experience

### [2.0.8] - 2025-08-08

#### Fix

- Increase transfer fee decimals precision

### [2.0.7] - 2025-07-15

#### Added

- Swap tokens feature on wallet view
- Set max balance button on Buy GLDT view

#### Changed

- Minor UI fix & improvements

### [2.0.6] - 2025-07-11

#### Changed

- Separate calls for fetching GLD NFT's and gold price in USD

### [2.0.5] - 2025-07-11

#### Changed

- Update request headers in ic-assets.json5

### [2.0.4] - 2025-07-11

#### Fix

- CORS Removing header Access-Control-Allow-Origin

### [2.0.3] - 2025-07-09

#### Fix

- Gold USD price not fetched from API
- Range error on Buy GLDT amount input

### [2.0.2] - 2025-07-03

#### Fix

- Security policy issue

### [2.0.1] - 2025-07-03

#### Fix

- Fetching Gold price API not working

### [2.0.0] - 2025-06-30

#### Description

This marks the initial release of Gold DAO dApp.
