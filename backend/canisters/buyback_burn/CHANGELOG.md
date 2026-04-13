# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.2.0] - 2026-03-03

#### Removed

- **ICP-GOLDAO exchange job**: The automated GOLDAO buyback mechanism has been permanently disabled.
- **GOLDAO token burn**: The final canister-initiated GOLDAO burn is executed immediately after post_upgrade, ensuring the canister’s GOLDAO balance is reduced to zero.


### [1.1.0] - 2026-01-19

#### Added

- **GLDT distribution job for GOLDAO stakers**: Introduced a new exchange job that continuously converts 33% of ICP rewards into GLDT and routes the acquired GLDT to a reward pool for distribution to GOLDAO stakers.
- **Multi-job support in buyback & burn canister**: Extended the canister to support multiple exchange jobs with distinct interval and post-swap behaviors (burn vs. reward distribution), enabling future expansion without deploying additional canisters.

### [1.0.3] - 2025-07-18

#### Added

- **Support of ICRC10 standard**: ICRC-10 is a standard aimed at simplifying the discovery of supported standards by canisters on the Internet Computer. By providing a unified method, icrc10_supported_standards, canisters can easily expose the standards they implement, enhancing interoperability and easing integration efforts across the ecosystem.

### [1.0.2] - 2024-12-03

#### Changed

- **Burn frequency**: The burn process would become less frequent - once per day at 12:00 UTC.

### [1.0.1] - 2024-10-14

#### Changed

- **Burn amount calculation**: Previously, the buyback&burn amount was calculated once per week, while in current version, it's calculated dynamically every interval.

### [1.0.0] - 2024-08-29

#### Description
This marks the initial release of the buyback_burn canister. The canister is designed to support a deflationary tokenomics model by recieving ICP tokens from various sources and selling them on a decentralized exchange (DEX). The obtained GOLDAO tokens are then sent to a minting address to be burned, thereby reducing the token supply over time. Currently, the canister supports swaps through ICPSwap.

#### Added
- **DEX Integration** : Support for ICPSwap to facilitate ICP token swaps. The design is scalable, so that other DEXs could be added in future
- **Token Burning** : Mechanism to send swapped tokens to a minting address for burning.
- **Error Handling** : Basic error handling during swap and burn processes to ensure reliable operations.
