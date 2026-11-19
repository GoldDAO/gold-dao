# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [2.1.2] - 2026-11-19

### Changed

- Read the treasury figures from `api.gold-dao.org`, the new address of the Gold DAO API, instead of `api.gldt.org`. The Content-Security-Policy allows the new host and no longer the old one, and the Privacy Policy names the new host.

### [2.1.1] - 2026-11-18

### Changed

- Terms of Service: the sections that named GLDT as a token of Gold DAO (services, token usage, risks) now speak of GOLDAO and treat the reward tokens as tokens of third parties. The date of the last change is stated.
- Privacy Policy: rewritten. The old text only covered a newsletter sign-up that the website does not have. The new one says what the website and the Gold DAO app store in the browser, what ends up on the blockchain and which third parties are involved.

### [2.1.0] - 2026-11-18

### Added

- Docs under `/docs`, moved from the GitBook on docs.gold-dao.org and brought up to date: staking and claim guides, rewards, treasury, buyback and burn, GOLDAO token, canisters, brand assets, FAQ, Terms of Service and Privacy Policy
- An unknown path under `/docs/`, such as an old GitBook link forwarded from docs.gold-dao.org, opens the docs introduction

### Changed

- The Docs link and the staking guide link open the new docs instead of the GitBook
- Pages are exported as `<path>/index.html` so the asset canister serves `/docs` and `/docs/` alike

### Removed

- GitBook content that no longer applies to Gold DAO: GLDT staking, redeeming physical gold, the GLD NFT, GLDT and USDG token pages and logos, vault audit reports, partner page, roadmap, AMA summary and master spreadsheet

### [2.0.0] - 2026-11-18

### Changed

- New one-page website about GOLDAO and the treasury. The GLD NFT, GLDT and USDG content is gone.
- The treasury holdings are shown live from the analytics API
- Typeface and colors are the ones of the Gold DAO dApp
- New social preview image
- The Content-Security-Policy only allows the two hosts the page uses

### Removed

- Translations, canister bindings and the dependencies they needed (i18next, react-query, @dfinity)
- The old images, the background video and the old whitepaper PDF. The build went from 30 MB to 1 MB.

### [1.0.11] - 2026-11-17

### Changed

- Dashboard links point to app.gold-dao.org; the GLDT buy link opens gldt.org

### [1.0.10] - 2026-10-27

### Changed

- Remove the unused token metrics hook that called the stopped token_metrics canister

### [1.0.9] - 2025-07-09

### Fix

- Hero animation not loading
- GLDT USD price not fetched from API

### [1.0.8] - 2025-07-03

### Changed

- Links now redirect to the new Gold DAO dApp
- Footer copyright year (from 2024 to 2025)

### [1.0.7] - 2025-06-04

#### Changed

Update twitter link

### [1.0.4] - 2025-04-16

#### Changed

Update audit report link

### [1.0.3] - 2024-10-30

#### Description

Update GLDT swap button.

### [1.0.2] - 2024-10-30

#### Description

Update texts, images and add popups on the website.

### [1.0.1] - 2024-10-28

#### Description

Update texts and storyline on the website.

### [1.0.0] - 2024-10-18

#### Description

This is the initial release of the new Gold DAO website.
