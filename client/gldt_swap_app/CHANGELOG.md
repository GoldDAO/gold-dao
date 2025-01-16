# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [1.1.1] - 2025-01-13

#### Changed

- Update NFID identityKit to version 1.0.7
- Enable Plug wallet on mobile device (for in-browser Plug app)

### [1.1.0] - 2024-12-06

#### Description

This version integrate Explorer view, also fix small issues.

#### Added

- Explorer view, users can now follow transactions, top 100 GLDT holders and token infos.

### [1.0.8] - 2024-11-19

#### Fix

- Improve handling onError state reverse swap.

### [1.0.7] - 2024-11-14

#### Fix

- Retry on reverse/forward swap errors.

### [1.0.6] - 2024-11-12

#### Changed

- Improve retry on reverse/forward swap errors.

### [1.0.5] - 2024-11-06

#### Changed

- Handling errors on reverse swap.

#### Fix

- User can't reconnect after manually disconnect.

### [1.0.4] - 2024-11-04

#### Description

This update fixes ledger's transfer issues.

#### Fix

- Ledger transfer issues (incorrect amount/fees calculation).

### [1.0.3] - 2024-11-01

#### Description

This update fixes a few bugs and adds some minor improvements to the user interface.

#### Added

- Update displayed data on account page on success transactions.

#### Changed

- Notation for numbers > 1000 improving readability.

#### Fix

- Responsive issue on landing page.


### [1.0.2] - 2024-10-30

#### Description

This version mostly improve wordings on landing page and frequently asked questions page, also improve authentication.

#### Changed

- Wording landing page and FAQ's.

#### Fix

- start_date on price-history.api.bity that ensure it correctly fetch gold price of the current day.
- remove derivationOrigin for staging canister.
- isInitializing state for useIdentityKit.

### [1.0.1] - 2024-10-29

#### Description

This version improve global UI of app, also improve authentication and fix small issues.

#### Changed

- Loading state UI on transaction details page.
- Version of identity kit from 1.0.0 to 1.0.1 fixing NFIDW not working when derivationOrigin setted.
- "refetchInterval" for pooling data on account page from 10 to 5 seconds.
- Declaration files for swap canisters.
- Removed isConnecting dialog on landing page.
- Derivation origin from gldt swap to gld dashboard, for consistency between Gold related apps.

#### Fix

- Wording on forward swap confirm.
- Wording on landing page.
- Transfer fee and amount sended on Ledger transfer.
- Correctly reset state swap on errors.

### [1.0.0] - 2024-10-28

#### Description

This marks the initial release of Swap App. It allows users to swap GLD NFT to GLDT and vice versa.  
It allows also transfer of GLD NFT, GLDT and OGY.
