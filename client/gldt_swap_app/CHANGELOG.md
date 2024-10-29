# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

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
