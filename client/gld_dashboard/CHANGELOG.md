# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [2.3.1] - 2026-05-13

#### Fix

- Internet Identity login failing with "Unable to connect" after the migration from `identity.internetcomputer.org` to `id.ai`

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
